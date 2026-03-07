/// P,A,C,K,E,D unpacker — decodes Dean Edwards' JS packer format.
///
/// AnimePahe uses this to obfuscate the Kwik.si stream URL inside eval() blocks.
///
/// Format:
///   eval(function(p,a,c,k,e,d){...}('BODY',BASE,COUNT,'tok0|tok1|...'))
///
/// The decoder replaces each word-token in `body` with the corresponding
/// dictionary entry (k[parseInt(token, base)]).  If the entry is empty the
/// original token is kept.

/// Extract the raw HLS `.m3u8` URL from a Kwik HTML page.
///
/// The page contains one or more eval(function(p,a,c,k,e,d){...}(...)) blocks.
/// We unpack each one until we find a `source='<url>'` assignment.
pub fn extract_video_url(html: &str) -> Option<String> {
    // Collect all eval(function(...){...}('...')) blocks
    let mut search = html;
    while let Some(pos) = search.find("eval(function(p,a,c,k,e,d)") {
        let block = &search[pos..];
        if let Some(url) = try_unpack_block(block) {
            return Some(url);
        }
        // advance past this occurrence
        search = &search[pos + 1..];
    }
    None
}

/// Try to unpack one eval block and extract the source URL.
fn try_unpack_block(block: &str) -> Option<String> {
    let decoded = unpack_eval(block)?;
    // Look for:  source='https://...'  or  src="https://..."
    extract_source_url(&decoded)
}

/// Decode one P,A,C,K,E,D packed string.
pub fn unpack_eval(packed: &str) -> Option<String> {
    // We need to find the arguments passed to the outer function:
    //   function(p,a,c,k,e,d){...}('BODY', BASE, COUNT, 'KEY0|KEY1|...'.split('|'), ...)
    //
    // Strategy: find the last ( before the closing )) and parse the comma-separated args.
    let args = extract_call_args(packed)?;
    if args.len() < 4 {
        return None;
    }

    // The PACKD body was stored inside a single-quoted JS string, so any `'` inside
    // the body was written as `\'`.  Unescape the body before token substitution so
    // that the reconstructed JS has literal quote characters (e.g. `source='URL'`).
    let body = unescape_js_string(&args[0]);
    let base: u32 = args[1].trim().parse().ok()?;
    // args[2] is count (c) – we don't need it; the dict length is authoritative
    let keys_raw = &args[3];
    let dict: Vec<&str> = keys_raw.split('|').collect();

    let result = replace_tokens(&body, base, &dict);
    Some(result)
}

/// Unescape a JS string body that was stored inside an outer single-quoted string.
/// `\'` → `'`, `\\` → `\`, `\n` → newline, etc.
fn unescape_js_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'\'' => { out.push('\''); i += 2; }
                b'\\' => { out.push('\\'); i += 2; }
                b'n' => { out.push('\n'); i += 2; }
                b'r' => { out.push('\r'); i += 2; }
                b't' => { out.push('\t'); i += 2; }
                other => { out.push(bytes[i] as char); out.push(other as char); i += 2; }
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

/// Replace every word-token in `body` with its dictionary lookup.
/// A token is a contiguous run of `[a-zA-Z0-9_]` characters.
fn replace_tokens(body: &str, base: u32, dict: &[&str]) -> String {
    let mut out = String::with_capacity(body.len());
    let bytes = body.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if is_word_char(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_word_char(bytes[i]) {
                i += 1;
            }
            let token = &body[start..i];
            let replacement = lookup_token(token, base, dict);
            out.push_str(replacement.unwrap_or(token));
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Convert a token from the given base to a usize dict index.
///
/// Rust's `from_str_radix` only handles bases 2–36.  PACKD commonly uses base 62
/// (alphabet: `0–9` → 0–9, `a–z` → 10–35, `A–Z` → 36–61), so we implement our
/// own decoder that covers bases up to 62.
fn lookup_token<'d>(token: &str, base: u32, dict: &[&'d str]) -> Option<&'d str> {
    let index = if base <= 36 {
        usize::from_str_radix(token, base).ok()?
    } else {
        // Custom decoder for bases 37–62.
        let mut result: usize = 0;
        for ch in token.chars() {
            let digit = match ch {
                '0'..='9' => ch as usize - '0' as usize,
                'a'..='z' => ch as usize - 'a' as usize + 10,
                'A'..='Z' => ch as usize - 'A' as usize + 36,
                _ => return None,
            };
            if digit >= base as usize {
                return None;
            }
            result = result.checked_mul(base as usize)?.checked_add(digit)?;
        }
        result
    };
    let entry = dict.get(index)?;
    if entry.is_empty() {
        None // keep original token
    } else {
        Some(entry)
    }
}

/// Extract the string arguments passed to the outer function call.
///
/// We look for the pattern: `}('BODY', BASE, COUNT, 'KEYS'.split('|')` and
/// hand-parse the four fields, handling the `.split('|')` suffix on the last one.
fn extract_call_args(packed: &str) -> Option<Vec<String>> {
    // Find `}(` which marks start of the argument list
    let start = packed.find("}(")?;
    let args_section = &packed[start + 2..];

    // We pass args_section directly to parse_arg_fields.
    // This is correct because parse_arg_fields already handles single-quoted strings
    // (scanning char-by-char and honoring escape sequences), so parens *inside*
    // the quoted body string do not confuse the parser.
    // The old approach (naive paren-depth counting) broke when the packed body
    // contained unbalanced or nested parens inside string literals.
    let mut fields: Vec<String> = Vec::new();
    parse_arg_fields(args_section, &mut fields);

    Some(fields)
}

/// Very simple arg parser that understands:
///   - single-quoted strings  → strip quotes
///   - bare numbers           → as-is
///   - 'str'.split('|')       → strip quotes + .split(…) suffix
fn parse_arg_fields(s: &str, out: &mut Vec<String>) {
    let s = s.trim();
    if s.is_empty() {
        return;
    }

    if s.starts_with('\'') {
        // single-quoted string
        let mut i = 1;
        let bytes = s.as_bytes();
        let mut value = String::new();
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                // escape sequence – keep both chars for now
                value.push(bytes[i] as char);
                value.push(bytes[i + 1] as char);
                i += 2;
            } else if bytes[i] == b'\'' {
                i += 1; // end of string
                break;
            } else {
                value.push(bytes[i] as char);
                i += 1;
            }
        }
        out.push(value);
        // skip optional .split('|') suffix
        let rest = s[i..].trim_start();
        let rest = if rest.starts_with(".split(") {
            // skip to the closing ) of split(...)
            match rest.find(')') {
                Some(p) => rest[p + 1..].trim_start(),
                None => rest,
            }
        } else {
            rest
        };
        // next field starts after the comma
        if let Some(comma) = rest.find(',') {
            parse_arg_fields(&rest[comma + 1..], out);
        }
    } else {
        // bare number or other token – find the next comma
        match s.find(',') {
            Some(comma) => {
                out.push(s[..comma].trim().to_owned());
                parse_arg_fields(&s[comma + 1..], out);
            }
            None => {
                let v = s.trim().to_owned();
                if !v.is_empty() {
                    out.push(v);
                }
            }
        }
    }
}

/// Extract a streaming URL from the decoded script body.
/// Looks for `source='…'`, `source="…"`, and any `.m3u8` URL as a fallback.
fn extract_source_url(decoded: &str) -> Option<String> {
    // source='https://...'
    if let Some(url) = find_between(decoded, "source='", "'") {
        if url.starts_with("https://") || url.starts_with("http://") {
            return Some(url.to_owned());
        }
    }
    // source="https://..."
    if let Some(url) = find_between(decoded, "source=\"", "\"") {
        if url.starts_with("https://") || url.starts_with("http://") {
            return Some(url.to_owned());
        }
    }
    // Fallback: find any https URL ending with .m3u8
    // Scan for 'https://' and collect until a quote, space, or semicolon.
    let mut search = decoded;
    while let Some(pos) = search.find("https://") {
        let rest = &search[pos..];
        let end = rest.find(|c: char| c == '\'' || c == '"' || c == ';' || c == ' ' || c == '\n')
            .unwrap_or(rest.len());
        let candidate = &rest[..end];
        if candidate.contains(".m3u8") {
            return Some(candidate.to_owned());
        }
        search = &search[pos + 8..];
    }
    None
}

fn find_between<'a>(haystack: &'a str, start_pat: &str, end_pat: &str) -> Option<&'a str> {
    let start = haystack.find(start_pat)?;
    let after = &haystack[start + start_pat.len()..];
    let end = after.find(end_pat)?;
    Some(&after[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_tokens_base_36() {
        // Simple test: base-36 tokens
        let dict = vec!["hello", "", "world"];
        let result = replace_tokens("0 1 2", 36, &dict);
        assert_eq!(result, "hello 1 world");
    }

    #[test]
    fn test_find_between() {
        let s = "before source='https://example.com/video.m3u8' after";
        assert_eq!(
            find_between(s, "source='", "'"),
            Some("https://example.com/video.m3u8")
        );
    }
}
