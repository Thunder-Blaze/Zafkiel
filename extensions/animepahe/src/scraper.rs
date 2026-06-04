/// HTML scraping helpers for AnimePahe pages.
use crate::types::{AnimeDetails, StreamSource};

// ─────────────────────────────────────────────────────────────────────────────
// Anime detail page  (GET /anime/{session})
// ─────────────────────────────────────────────────────────────────────────────

pub fn scrape_anime_details(html: &str, session: &str) -> AnimeDetails {
    let title = extract_title(html).unwrap_or_else(|| session.to_owned());
    let cover_url = extract_cover(html);
    let description = extract_description(html);
    let anilist_id = extract_anilist_id(html);

    AnimeDetails {
        id: session.to_owned(),
        title,
        cover_url,
        description,
        anilist_id,
        r#type: None,
        status: None,
        year: None,
        session_id: session.to_owned(),
    }
}

fn extract_title(html: &str) -> Option<String> {
    // <span style="user-select:text">TITLE</span>
    find_between(html, r#"<span style="user-select:text">"#, "</span>")
        .or_else(|| find_between(html, r#"<meta property="og:title" content=""#, r#"">"#))
        .map(decode_html_entities)
}

fn extract_cover(html: &str) -> Option<String> {
    find_between(html, r#"<a href="https://i.animepahe.pw/posters/"#, r#"""#)
        .map(|s| format!("https://i.animepahe.pw/posters/{s}"))
        .or_else(|| {
            find_between(html, r#"<meta property="og:image" content=""#, r#"">"#)
                .map(|s| s.to_owned())
        })
}

fn extract_description(html: &str) -> Option<String> {
    // AnimePahe puts the synopsis in a <div class="anime-synopsis"> block.
    // The content may contain nested HTML tags, so we strip inner tags.
    find_between(html, r#"<div class="anime-synopsis">"#, "</div>")
        .map(|s| strip_tags(s).trim().to_owned())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            find_between(html, r#"<meta property="og:description" content=""#, r#"">"#)
                .map(|s| decode_html_entities(s))
        })
}

fn extract_anilist_id(html: &str) -> Option<u32> {
    // <a href="//anilist.co/anime/12345">
    find_between(html, r#"href="//anilist.co/anime/"#, r#"""#)
        .and_then(|s| s.parse().ok())
}

// ─────────────────────────────────────────────────────────────────────────────
// Play page  (GET /play/{animeId}/{episodeId})
// ─────────────────────────────────────────────────────────────────────────────

/// Extract the list of Kwik stream buttons from the play page HTML.
pub fn scrape_stream_sources(html: &str) -> Vec<StreamSource> {
    let mut sources = Vec::new();
    let mut search = html;

    while let Some(pos) = search.find("<button") {
        let tag_end = search[pos..].find('>').unwrap_or(0);
        let tag = &search[pos..pos + tag_end + 1];

        // Only process buttons that have a data-src pointing to Kwik
        if let (Some(kwik_url), Some(fansub), Some(resolution), Some(audio)) = (
            attr_value(tag, "data-src"),
            attr_value(tag, "data-fansub"),
            attr_value(tag, "data-resolution"),
            attr_value(tag, "data-audio"),
        ) {
            let is_kwik = kwik_url.contains("kwik.si") || kwik_url.contains("kwik.cx");
            if is_kwik {
                sources.push(StreamSource {
                    id: kwik_url.to_owned(),
                    label: format!("{fansub} {resolution}p ({audio})"),
                    resolution: resolution.to_owned(),
                    fansub: fansub.to_owned(),
                    audio: audio.to_owned(),
                    requires_resolution: true,
                });
            }
        }

        search = &search[pos + 1..];
    }

    sources
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Extract the value of an HTML attribute like `data-src="value"` from a tag string.
fn attr_value<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
    // Try double quotes first, then single quotes
    let double = format!(r#"{attr}=""#);
    let single = format!("{attr}='");
    if let Some(v) = find_between(tag, &double, "\"") {
        return Some(v);
    }
    if let Some(v) = find_between(tag, &single, "'") {
        return Some(v);
    }
    None
}

fn find_between<'a>(haystack: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = haystack.find(start)?;
    let rest = &haystack[s + start.len()..];
    let e = rest.find(end)?;
    Some(&rest[..e])
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}
