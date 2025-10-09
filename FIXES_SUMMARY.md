# Fixes Applied - Dashboard & Auth Issues

## Issues Fixed

### 1. ✅ Tailwind CSS Parse Error in TitleBar
**Problem:** `[plugin:@tailwindcss/vite:generate:serve] Invalid declaration: onMount`

**Root Cause:** Tailwind v4 CSS parser was trying to parse comments in the `<style>` tag

**Solution:** Added `lang="postcss"` attribute to the style tag and removed unnecessary comments
```svelte
<style lang="postcss">
  [data-tauri-drag-region] {
    -webkit-app-region: drag;
  }
  ...
</style>
```

---

### 2. ✅ Auth Token Not Persisting / Decryption Issues
**Problem:** 
- Token was encrypted in config but not being decrypted on app startup
- Auth state resetting on app restart
- Logout menu showing when user not logged in

**Root Causes:**
1. **Token saving without encryption:** `auth_commands.rs` was using `config::save()` which doesn't encrypt
2. **Token loading without decryption:** `lib.rs` was reading encrypted token directly without decrypting
3. **ProfileDropdown showing logout always:** No conditional rendering based on auth state

**Solutions:**

#### A. Fixed Token Encryption on Save (`auth_commands.rs`)
```rust
// OLD - Saving plaintext
let mut config = config::load_or_default()?;
config.anilist.access_token = Some(token);
config::save(&config)?;

// NEW - Saving encrypted
let config_loader = config::ConfigLoader::new()?;
config_loader.set_anilist_token(&token)?; // Automatically encrypts
```

#### B. Fixed Token Decryption on Load (`lib.rs`)
```rust
// OLD - Loading encrypted token directly
let token = config_loader
    .get_config()
    .ok()
    .and_then(|config| config.anilist.access_token);

// NEW - Loading decrypted token
let token = config_loader
    .get_anilist_token()
    .ok()
    .flatten(); // Automatically decrypts
```

#### C. Fixed Logout on Clear (`auth_commands.rs`)
```rust
// OLD - Manual clearing
let mut config = config::load_or_default()?;
config.anilist.access_token = None;
config::save(&config)?;

// NEW - Using ConfigLoader
let config_loader = config::ConfigLoader::new()?;
config_loader.clear_anilist_token()?;
```

---

### 3. ✅ ProfileDropdown Showing Logout When Not Authenticated
**Problem:** Logout menu item visible even for guest users

**Solution:** Wrapped logout button in conditional check
```svelte
{#if $isAuthenticated}
  <DropdownMenu.Separator />
  <DropdownMenu.Item class="text-red-500" onclick={handleLogout}>
    <Icon icon="solar:logout-2-bold" class="mr-2 h-4 w-4" />
    Logout
  </DropdownMenu.Item>
{/if}
```

Also fixed other menu items to show appropriate options for guests:
- Profile → Only if authenticated
- Settings → Always visible
- Favorites → Only if authenticated

---

### 4. ✅ Dashboard Layout Issues
**Problem:** Components too wide, image dull, background not prominent

**Solutions:**

#### A. Reduced Widget Widths
- Changed from 45% to 40% width for corner widgets
- Changed from 48% to proper center positioning
- Better spacing between elements

#### B. Made Image Full Background
```svelte
<!-- Background Theme Image -->
<div class="absolute inset-0">
  <img src="..." class="w-full h-full object-cover" />
  <!-- Readable gradient overlay -->
  <div class="absolute inset-0 bg-gradient-to-br from-background/95 via-background/60 to-background/80"></div>
</div>
```

#### C. Increased Backdrop Blur
- Cards now use `backdrop-blur-md` (was `backdrop-blur-sm`)
- Increased transparency: `bg-card/60` (was `bg-card/50`)
- Better glass-morphism effect

#### D. Compact Top Left Components
```svelte
<!-- Welcome Card - Compact -->
<Card class="p-4 border-border/50 bg-card/60 backdrop-blur-md">
  <h1 class="text-xl font-bold tracking-tight">Welcome back! 👋</h1>
  <p class="text-xs text-muted-foreground">{username}</p>
</Card>

<!-- Stats Grid - Compact -->
<div class="grid grid-cols-2 gap-2">
  <Card class="p-3 ...">
    <div class="flex items-center gap-2">
      <div class="h-8 w-8 ..."><!-- icon --></div>
      <div>
        <p class="text-xl font-bold">{value}</p>
        <p class="text-[10px] text-muted-foreground">{label}</p>
      </div>
    </div>
  </Card>
</div>
```

---

## Testing Checklist

### Auth Flow
- [ ] Login and verify token is saved encrypted in `~/.config/zafkiel/config.debug.ron`
- [ ] Restart app and verify user stays logged in
- [ ] Check profile dropdown shows username and avatar
- [ ] Verify logout clears token and shows guest state
- [ ] Confirm guest users don't see logout button

### Dashboard
- [ ] Background image is visible and prominent
- [ ] Corner widgets are properly sized (40% width)
- [ ] Top left is compact with smaller text and spacing
- [ ] Cards have nice glass-morphism effect
- [ ] No Tailwind CSS errors in console
- [ ] Image is not dull (gradient overlay is balanced)

### Token Encryption
Run this to verify encrypted token format:
```bash
cat ~/.config/zafkiel/config.debug.ron
```

Should show:
```ron
anilist: (
    access_token: Some("base64_encrypted_string_here"),
),
```

---

## Files Modified

1. **src/lib/components/TitleBar.svelte**
   - Added `lang="postcss"` to style tag
   - Fixed CSS parsing error

2. **src/lib/components/ProfileDropdown.svelte**
   - Added conditional rendering for logout
   - Fixed menu items for guest users
   - Added proper auth state checks

3. **src/routes/+page.svelte**
   - Made theme image full background
   - Reduced widget widths to 40%
   - Increased backdrop blur
   - Compacted top left components
   - Adjusted spacing and padding

4. **src-tauri/src/auth_commands.rs**
   - Use `ConfigLoader::set_anilist_token()` for encryption
   - Use `ConfigLoader::clear_anilist_token()` for logout
   - Proper encrypted token handling

5. **src-tauri/src/lib.rs**
   - Use `config_loader.get_anilist_token()` for decryption
   - Load decrypted token on app startup
   - Proper token initialization

---

## Next Steps

1. **Test the app:**
   ```bash
   bun run tauri dev
   ```

2. **Clear existing config to test fresh:**
   ```bash
   rm ~/.config/zafkiel/config.debug.ron
   ```

3. **Login and verify token persists after restart**

4. **Check dashboard layout and styling**

5. **Verify no console errors**

---

## Technical Notes

### Encryption Flow
```
User Login → OAuth Token → ConfigLoader.set_anilist_token()
  → Generates encryption key (if not exists)
  → Encrypts token with AES-256-GCM
  → Saves to config.ron

App Startup → ConfigLoader.get_anilist_token()
  → Reads encrypted token from config.ron
  → Decrypts with stored encryption key
  → Passes to AniListService
```

### Security
- Tokens stored encrypted with AES-256-GCM
- Unique encryption key per installation
- Encryption key stored in same config (secure on local machine)
- Never transmit decrypted tokens over network
