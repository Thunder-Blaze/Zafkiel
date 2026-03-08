# Android Player Implementation

This document outlines the plan for implementing video playback on Android, where
`tauri-plugin-libmpv` is Linux/Windows/macOS-only. Android requires a different
native playback backend.

---

## Recommended Approach: ExoPlayer via Tauri Android Plugin

[ExoPlayer (Media3)](https://developer.android.com/guide/topics/media/exoplayer) is
Google's recommended media player for Android. It:
- Natively supports HLS, DASH, MP4, MKV, and virtually all common formats
- Handles custom HTTP headers (Referer, Cookie) via `DefaultHttpDataSource.Factory`
- Renders to a `SurfaceView` / `TextureView` embedded in the Tauri `WebView` overlay

### High-Level Architecture

```
Tauri WebView (transparent overlay)
  └── Custom SVG/HTML controls (PlayerControls.svelte)
       ↕ Tauri invoke bridge
Android LayerDrawable / SurfaceView (below WebView)
  └── ExoPlayer rendering surface
```

The same "underlay trick" used for libmpv on desktop applies on Android:
1. The native Android window lays out the ExoPlayer `SurfaceView` behind the `WebView`.
2. The `WebView` background is set to transparent via `Tauri.setBackgroundColor(0x00000000)`.
3. `PlayerControls.svelte` floats on top with a dark scrim background.

---

## Implementation Steps

### 1. Create a Tauri Android plugin

```
src-tauri/gen/android/app/src/main/java/com/zafkiel/MediaPlayerPlugin.kt
```

Key Kotlin implementation:

```kotlin
import android.view.SurfaceView
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.hls.HlsMediaSource
import androidx.media3.common.MediaItem
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.Command
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class MediaPlayerPlugin(private val activity: android.app.Activity) : Plugin(activity) {
    private var player: ExoPlayer? = null
    private var surface: SurfaceView? = null

    @Command
    fun initPlayer(invoke: Invoke) {
        activity.runOnUiThread {
            val factory = DefaultHttpDataSource.Factory()
            player = ExoPlayer.Builder(activity)
                .setMediaSourceFactory(HlsMediaSource.Factory(factory))
                .build()

            surface = SurfaceView(activity)
            player!!.setVideoSurfaceView(surface)
            // Insert surface below the WebView in the view hierarchy
            val decorView = activity.window.decorView as android.view.ViewGroup
            decorView.addView(surface, 0)    // index 0 = bottom layer

            invoke.resolve()
        }
    }

    @Command
    fun loadFile(invoke: Invoke) {
        val url = invoke.getString("url") ?: return invoke.reject("url required")
        val referer = invoke.getString("referer")
        val cookie = invoke.getString("cookie")

        activity.runOnUiThread {
            val factory = DefaultHttpDataSource.Factory().apply {
                setDefaultRequestProperties(buildMap {
                    if (!referer.isNullOrEmpty()) put("Referer", referer)
                    if (!cookie.isNullOrEmpty()) put("Cookie", cookie)
                })
            }
            val source = HlsMediaSource.Factory(factory)
                .createMediaSource(MediaItem.fromUri(url))
            player?.apply {
                setMediaSource(source)
                prepare()
                playWhenReady = true
            }
            invoke.resolve()
        }
    }

    @Command
    fun setProperty(invoke: Invoke) {
        val name = invoke.getString("name") ?: return
        val value = invoke.getString("value") ?: return
        activity.runOnUiThread {
            when (name) {
                "pause" -> if (value == "true") player?.pause() else player?.play()
                "volume" -> player?.volume = value.toFloatOrNull()?.div(100f) ?: 1f
            }
        }
        invoke.resolve()
    }

    @Command
    fun seek(invoke: Invoke) {
        val posMs = ((invoke.getDouble("pos") ?: 0.0) * 1000).toLong()
        activity.runOnUiThread { player?.seekTo(posMs) }
        invoke.resolve()
    }

    @Command
    fun destroyPlayer(invoke: Invoke) {
        activity.runOnUiThread {
            player?.release()
            player = null
            surface?.let { (activity.window.decorView as android.view.ViewGroup).removeView(it) }
            surface = null
        }
        invoke.resolve()
    }
}
```

### 2. Register the plugin in MainActivity.kt

```kotlin
// src-tauri/gen/android/app/src/main/java/com/zafkiel/MainActivity.kt
class MainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        registerPlugin(MediaPlayerPlugin::class.java)
    }
}
```

### 3. Add Media3 dependencies

In `src-tauri/gen/android/app/build.gradle`:
```gradle
dependencies {
    implementation "androidx.media3:media3-exoplayer:1.3.0"
    implementation "androidx.media3:media3-exoplayer-hls:1.3.0"
    implementation "androidx.media3:media3-ui:1.3.0"
    implementation "androidx.media3:media3-datasource-okhttp:1.3.0"
}
```

### 4. Frontend: Platform-aware VideoPlayer

The `VideoPlayer.svelte` component should detect the platform and use the
appropriate backend:

```typescript
import { platform } from '@tauri-apps/plugin-os';
import {
  init as libmpvInit,
  destroy as libmpvDestroy,
  // ...
} from 'tauri-plugin-libmpv-api';
import { invoke } from '@tauri-apps/api/core';

const currentPlatform = await platform();
const isAndroid = currentPlatform === 'android';

if (isAndroid) {
  await invoke('plugin:MediaPlayerPlugin|initPlayer');
} else {
  await libmpvInit({ /* ... */ });
}
```

A cleaner architecture is a shared `IMediaPlayer` interface with two implementations:
- `src/lib/player/libmpv-player.ts` — desktop (Linux/Windows/macOS)
- `src/lib/player/exoplayer-player.ts` — Android

`VideoPlayer.svelte` picks the implementation at runtime.

---

## Event Forwarding

ExoPlayer exposes a `Player.Listener` interface. Forward position/duration updates
back to the frontend via Tauri events:

```kotlin
player?.addListener(object : Player.Listener {
    override fun onEvents(player: Player, events: Player.Events) {
        val evt = mapOf(
            "time-pos" to player.currentPosition / 1000.0,
            "duration"  to player.duration / 1000.0,
            "pause"     to !player.isPlaying,
        )
        // Emit to frontend
        trigger("mpv-property-change", evt)
    }
})
```

---

## Limitations vs. Desktop libmpv

| Feature | libmpv (desktop) | ExoPlayer (Android) |
|---------|-----------------|----------------------|
| HLS | ✅ | ✅ |
| Custom headers | ✅ | ✅ |
| Hardware decode | ✅ auto | ✅ MediaCodec |
| Subtitles (ASS/SSA) | ✅ | ⚠️ SRT/VTT only |
| Audio tracks | ✅ | ✅ |
| DASH | ✅ | ✅ |
| MKV/VP9 | ✅ | ✅ (Android 5+) |

SSA/ASS subtitle rendering on Android requires a custom solution such as
[libass-android](https://github.com/xiongyihui/libass-android) or
[SubtitleView](https://developer.android.com/reference/androidx/media3/ui/SubtitleView).

---

## Future: mpv on Android via mpv-android

An alternative to ExoPlayer is to bind directly to
[mpv-android](https://github.com/mpv-android/mpv-android) (the library variant).
This provides full mpv feature parity including ASS subtitle rendering, but requires:
- Shipping libmpv.so ARM/ARM64 builds (~10 MB)
- JNI wrapper for the `mpv_*` C API

This is lower priority but would unify the desktop and mobile code paths.
