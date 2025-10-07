# Config System Documentation

## Overview

The Zafkiel config system provides secure, encrypted storage for application settings with a focus on protecting sensitive data like AniList API tokens.

## Architecture

### Backend (Rust)

```
src-tauri/src/config/
├── mod.rs          # Module exports
├── types.rs        # Configuration structures
├── encryption.rs   # AES-256-GCM encryption
└── loader.rs       # Config file management
```

### Frontend (TypeScript)

```
src/lib/
├── services/config.ts  # Config service API
└── stores/config.ts    # Reactive Svelte stores
```

## Features

### 🔐 Security

- **AES-256-GCM Encryption**: Military-grade encryption for sensitive data
- **Auto-generated Keys**: Encryption key is generated automatically when first needed
- **Base64 Encoding**: Keys and encrypted data are safely encoded
- **Key Management**: Encryption key is stored securely in the config file

### 📁 Storage

- **Location**: `~/.config/zafkiel/`
- **Files**:
  - `config.test.ron` - Used during test runs (isolated from dev/prod)
  - `config.debug.ron` - Used during development (debug builds)
  - `config.ron` - Used in production (release builds)
- **Format**: RON (Rusty Object Notation) - human-readable and type-safe
- **Auto-creation**: Config file and directory are created automatically

This separation ensures that:

- Tests don't interfere with your development configuration
- Development settings don't affect production deployments
- Each environment can maintain its own settings independently

### 🎨 Configuration Sections

#### 1. AniList Configuration

```ron
anilist: (
    access_token: Some("encrypted_token_here"),
)
```

#### 2. Security Configuration

```ron
security: (
    encryption_key: "base64_encoded_key_here",
)
```

#### 3. UI Configuration

```ron
ui: (
    theme: "catppuccin",
    glow_effects: true,
    animations: true,
    smooth_scroll: true,
)
```

## Usage

### Backend (Rust)

```rust
use crate::config::ConfigLoader;

// Initialize config loader
let config = ConfigLoader::new()?;

// Set encrypted token
config.set_anilist_token("my_token")?;

// Get decrypted token
let token = config.get_anilist_token()?;

// Update UI settings
config.update_ui_theme("dark".to_string())?;
config.update_animations(false)?;
```

### Frontend (TypeScript/Svelte)

#### Using the Service

```typescript
import { ConfigService } from '$lib/services/config';

// Get config
const config = await ConfigService.getConfig();

// Set token (auto-encrypted)
await ConfigService.setAniListToken('my_token');

// Get decrypted token
const token = await ConfigService.getAniListToken();

// Update UI settings
await ConfigService.updateTheme('dark');
await ConfigService.updateAnimations(false);
```

#### Using Svelte Stores

```svelte
<script lang="ts">
	import { configStore, uiConfig, isAuthenticated } from '$lib/stores/config';
	import { onMount } from 'svelte';

	onMount(async () => {
		await configStore.init();
	});

	// Reactive values
	$: theme = $uiConfig.theme;
	$: authenticated = $isAuthenticated;
</script>

<button onclick={() => configStore.updateTheme('dark')}> Dark Theme </button>

{#if $isAuthenticated}
	<p>User is authenticated!</p>
{/if}
```

## Encryption Details

### Algorithm

- **Cipher**: AES-256-GCM (Galois/Counter Mode)
- **Key Size**: 256 bits (32 bytes)
- **Nonce Size**: 96 bits (12 bytes)
- **Authentication**: Built-in authentication tag

### Key Generation

```rust
// Keys are generated using cryptographically secure random number generator
let key = encryption::generate_key(); // Returns base64-encoded 256-bit key
```

### Encryption Process

1. Generate random nonce (12 bytes)
2. Encrypt data using AES-256-GCM
3. Prepend nonce to ciphertext
4. Base64 encode the result

### Decryption Process

1. Base64 decode the encrypted data
2. Extract nonce (first 12 bytes)
3. Decrypt ciphertext using nonce and key
4. Verify authentication tag
5. Return plaintext

## API Reference

### Tauri Commands

All commands return `ConfigResponse<T>`:

```typescript
interface ConfigResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
}
```

#### Commands

- `get_config()` - Get full config
- `get_anilist_token()` - Get decrypted token
- `set_anilist_token(token: string)` - Set and encrypt token
- `clear_anilist_token()` - Remove token
- `get_ui_config()` - Get UI settings
- `update_ui_config(config: UiConfig)` - Update all UI settings
- `update_theme(theme: string)` - Update theme
- `update_glow_effects(enabled: bool)` - Toggle glow effects
- `update_animations(enabled: bool)` - Toggle animations
- `update_smooth_scroll(enabled: bool)` - Toggle smooth scroll
- `get_config_path()` - Get config file path

## Testing

### Run Backend Tests

```bash
cd src-tauri
cargo test
```

### Test Demo Page

```bash
bun run dev
# Navigate to /config-demo
```

## Security Best Practices

1. **Never commit the config file** - It contains sensitive data
2. **Never log decrypted tokens** - Always use substring for display
3. **Key rotation** - Consider implementing key rotation for long-term use
4. **Access control** - Config file permissions should be user-only (600)

## Future Enhancements

- [ ] Multiple encryption key support
- [ ] Config versioning and migration
- [ ] Cloud sync support
- [ ] Config backup and restore
- [ ] Per-user config profiles
- [ ] Config validation and schema enforcement

## Troubleshooting

### Config file not found

The config file is created automatically on first run. If missing, it will be regenerated with default values.

### Encryption key empty

If the encryption key is empty when trying to decrypt, set a token first - this will auto-generate the key.

### Invalid encrypted data

This usually means the encryption key has changed. Clear the token and set it again.

### Permission denied

Ensure the config directory (`~/.config/zafkiel/`) has proper permissions (700).

## Example Config File

```ron
(
    anilist: (
        access_token: Some("YmFzZTY0X2VuY3J5cHRlZF90b2tlbl9oZXJl"),
    ),
    security: (
        encryption_key: "YmFzZTY0X2VuY29kZWRfa2V5X2hlcmU=",
    ),
    ui: (
        theme: "catppuccin",
        glow_effects: true,
        animations: true,
        smooth_scroll: true,
    ),
)
```
