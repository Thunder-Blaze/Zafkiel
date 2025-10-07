# Config Build Profiles

## Overview

The Zafkiel config system uses separate configuration files for different build profiles to prevent environment interference and maintain proper isolation between test, development, and production settings.

## File Structure

```
~/.config/zafkiel/
├── config.test.ron    # Test environment
├── config.debug.ron   # Development environment
└── config.ron         # Production environment
```

## Build Profile Selection

The config file is automatically selected at **compile time** based on Rust's conditional compilation attributes:

```rust
pub fn get_config_path(&self) -> PathBuf {
    let filename = if cfg!(test) {
        "config.test.ron"      // Used when running `cargo test`
    } else if cfg!(debug_assertions) {
        "config.debug.ron"     // Used when running `cargo run` or `bun tauri dev`
    } else {
        "config.ron"           // Used in release builds
    };

    self.config_dir.join(filename)
}
```

## Build Profiles Explained

### 🧪 Test Profile (`config.test.ron`)

**When Used**: During test execution (`cargo test`)

**Characteristics**:
- Completely isolated from development and production configs
- Tests don't interfere with your personal settings
- Automatically created with minimal defaults
- Default settings:
  - Theme: `dark`
  - All UI effects disabled
  - No encryption key (generated on-demand)

**Activation**: Automatically when `cfg!(test)` is true

---

### 🔧 Debug Profile (`config.debug.ron`)

**When Used**: During development

**Activation Methods**:
- `cargo run` (without `--release`)
- `bun tauri dev`
- `cargo build` (without `--release`)

**Characteristics**:
- Your personal development settings
- Persists across development sessions
- Fully functional with all features enabled
- Default settings:
  - Theme: `catppuccin`
  - All UI effects enabled
  - Encryption key generated on first token save

**Activation**: Automatically when `cfg!(debug_assertions)` is true

---

### 🚀 Release Profile (`config.ron`)

**When Used**: In production builds

**Activation Methods**:
- `cargo build --release`
- `bun tauri build`
- Distributed application binaries

**Characteristics**:
- Production user settings
- Optimized for end-user experience
- Completely separate from dev/test environments
- Default settings: Same as debug (user-friendly)

**Activation**: When neither `cfg!(test)` nor `cfg!(debug_assertions)` is true

## Benefits of This Approach

### ✅ Test Isolation
```bash
# Running tests won't affect your dev config
cargo test  # Uses config.test.ron

# Your development work continues unaffected
bun tauri dev  # Uses config.debug.ron
```

### ✅ Development Safety
```bash
# Development settings don't leak into production
bun tauri dev  # Uses config.debug.ron
bun tauri build  # Uses config.ron
```

### ✅ Clean Testing
- Each test run starts with a known state
- No cross-contamination between test runs
- Predictable test behavior

### ✅ User Privacy
- Production users' settings are never touched during development
- Developers' personal settings don't ship with the app
- Clear separation of concerns

## File Creation Timeline

1. **First Test Run** (`cargo test`)
   - Creates `~/.config/zafkiel/config.test.ron`
   - Minimal defaults for fast test execution

2. **First Dev Run** (`bun tauri dev`)
   - Creates `~/.config/zafkiel/config.debug.ron`
   - Full-featured defaults for development

3. **First Production Run** (distributed app)
   - Creates `~/.config/zafkiel/config.ron`
   - User's personal settings

## Verification

Check which config files exist:
```bash
ls -la ~/.config/zafkiel/
```

View test config:
```bash
cat ~/.config/zafkiel/config.test.ron
```

View debug config:
```bash
cat ~/.config/zafkiel/config.debug.ron
```

View production config:
```bash
cat ~/.config/zafkiel/config.ron
```

## Implementation Details

### Compile-Time Selection

The file selection happens at **compile time**, not runtime. This means:

- ✅ Zero runtime overhead
- ✅ No configuration needed
- ✅ Impossible to accidentally use wrong config
- ✅ Type-safe and guaranteed by Rust compiler

### Conditional Compilation

```rust
#[cfg(test)]
mod tests {
    // These tests automatically use config.test.ron
}

#[cfg(debug_assertions)]
fn debug_only_feature() {
    // Uses config.debug.ron
}

#[cfg(not(debug_assertions))]
fn release_only_feature() {
    // Uses config.ron
}
```

## Migration from Single File

If you previously had a single `config.ron` file:

1. **Backup your config**:
   ```bash
   cp ~/.config/zafkiel/config.ron ~/.config/zafkiel/config.ron.backup
   ```

2. **Rename for your environment**:
   ```bash
   # For development
   mv ~/.config/zafkiel/config.ron ~/.config/zafkiel/config.debug.ron

   # For production (if you're an end-user)
   mv ~/.config/zafkiel/config.ron ~/.config/zafkiel/config.ron
   ```

3. **System will auto-create missing files** on next run

## Best Practices

### ✅ DO
- Let the system auto-select the config file
- Use different settings in each environment
- Trust the compile-time selection
- Keep test configs minimal

### ❌ DON'T
- Don't manually copy configs between profiles
- Don't hardcode config paths in application code
- Don't share encryption keys between environments
- Don't commit any config.*.ron files to git

## Troubleshooting

### "My test changes persist in dev mode"
- ✅ This is impossible - they use different files
- Check you're actually running tests with `cargo test`

### "Changes in dev mode affect tests"
- ✅ This is impossible - they use different files
- Check you're actually running dev with `bun tauri dev`

### "Config file not found"
- ✅ Normal - files are created on first access
- System automatically creates files with defaults

### "Wrong theme loading"
- Check which build profile you're running
- Each profile has its own theme setting
- Changes in one profile don't affect others

## Related Documentation

- [CONFIG.md](./CONFIG.md) - Main config system documentation
- [ARCHITECTURE.md](./ARCHITECTURE.md) - Overall system architecture
- [SECURITY.md](./SECURITY.md) - Security implementation details

## Technical References

- [Rust Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [cfg! Macro](https://doc.rust-lang.org/std/macro.cfg.html)
- [Debug Assertions](https://doc.rust-lang.org/book/ch09-00-error-handling.html#panic-or-not-to-panic)
