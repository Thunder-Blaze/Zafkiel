# Image Caching Debug Report

## Problem
Images are being downloaded repeatedly instead of using cached versions.

## Evidence
```
[2025-10-11][12:35:09] Downloaded and cached image: https://...jpg -> .../cache/images/fvwp1i.jpg
[2025-10-11][12:35:13] Downloaded and cached image: https://...jpg -> .../cache/images/fvwp1i.jpg  
[2025-10-11][12:35:17] Downloaded and cached image: https://...jpg -> .../cache/images/wfxfze.jpg
[2025-10-11][12:35:18] Downloaded and cached image: https://...jpg -> .../cache/images/wfxfze.jpg
```

Same images downloaded multiple times!

## Investigation

### ✅ Database Setup
```bash
$ sqlite3 ~/.local/share/com.zafkiel.dev/zafkiel.db ".schema cached_images"
CREATE TABLE cached_images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_url TEXT NOT NULL UNIQUE,
    local_path TEXT NOT NULL UNIQUE,
    file_size INTEGER,
    cached_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL
);
```
✅ Table exists with correct schema

### ❌ Database Empty
```bash
$ sqlite3 ~/.local/share/com.zafkiel.dev/zafkiel.db "SELECT COUNT(*) FROM cached_images;"
0
```
❌ NO cached images in database!

### ❌ Missing Logs
Expected logs from Rust:
- `[DB] get_cached_image_path called for: ...`
- `[DB] cache_image called: ... -> ...`
- `[DB] Cached image successfully: ...`

**NONE of these logs appear!**

This means:
1. `get_cached_image_path` Tauri command is NOT being called
2. `cache_image` Tauri command is NOT being called
3. Frontend is NOT invoking these commands

### Hypothesis
The frontend code is likely:
1. Encountering an error when trying to call Tauri commands
2. Silently catching the error
3. Falling back to original URL
4. Triggering repeated downloads

## Code Flow Analysis

### Expected Flow
```
CachedImage component loads
    ↓
ImageCacheService.getCachedImage(url)
    ↓
ClientDatabaseService.getCachedImagePath(url)
    ↓
invoke('get_cached_image_path', { url })
    ↓
Rust: get_cached_image_path()
    ↓
Query database
    ↓
IF FOUND: return path
IF NOT: download and cache
```

### Actual Flow (Suspected)
```
CachedImage component loads
    ↓
ImageCacheService.getCachedImage(url)
    ↓
ClientDatabaseService.getCachedImagePath(url)
    ↓
invoke('get_cached_image_path', { url })
    ↓
❌ ERROR (command not found? permission denied?)
    ↓
Returns null/undefined
    ↓
Falls back to original URL
    ↓
Downloads image again
```

## Next Steps

1. ✅ Added comprehensive logging to Rust commands
2. ✅ Added console.log to frontend service
3. ✅ Created test page at `/test-cache`
4. ⏳ Need to run app and check browser console
5. ⏳ Need to visit test page and see results

## Test Page
Visit http://localhost:5173/test-cache to manually test:
1. `get_cached_image_path` command
2. `cache_image` command  
3. Database persistence

This will show if the Tauri commands work at all.
