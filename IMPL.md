# Zafkiel — Exhaustive Implementation & Improvement Plan

## Preamble: Current State Analysis

After reading the full codebase, here is what exists and the state of each area:

### Critical Bugs Found

1. **Database schema mismatch**: `Database::new()` only creates `cached_images`, but `commands/db.rs` references 4 tables that **don't exist in production** — `local_progress`, `cached_media`, `cached_users`, `recently_viewed`. Every call to those commands currently returns a `"no such table"` database error.
2. **Dashboard has 100% hardcoded stats** (`src/routes/(app)/+page.svelte` lines 14-43) — 12 watching, 48 completed, 23 PTW, 1247 episodes are literal hardcoded numbers, not real user data.
3. **All mutations are stubs**: `useUpdateProgress`, `useAddToList`, `useRemoveFromList` in `src/lib/hooks/useAnilist.svelte.ts` lines 595/618 have `// TODO: Implement AniList API call` comments and never touch the AniList API.
4. **`ActionButtonStrip.svelte`** has a `// TODO: Implement API call to update status` comment — no actual status update works.

---

## Part 1 — Backend: Database Overhaul

### 1.1 Add `rusqlite_migration` for versioned schema migrations

**Why**: Currently if new tables/columns need added, there is no safe upgrade path. Every new release could corrupt user data. `rusqlite_migration` provides Rails-style versioned migrations.

**How**:

- Add `rusqlite_migration = "1.3"` and `r2d2 = "0.8"` + `r2d2_sqlite = "0.24"` to `Cargo.toml`
- Rewrite `database.rs` entirely with a `migrations::get_migrations()` function returning a `Vec<Migration>`
- Run migrations on every startup — idempotent

### 1.2 New normalized schema (all migrations)

**Migration 001** — Base tables with proper normalization (replacing the current broken state):

```sql
-- Core image cache (keep existing, no change)
CREATE TABLE cached_images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_url TEXT NOT NULL UNIQUE,
    local_path TEXT NOT NULL UNIQUE,
    file_size INTEGER,
    cached_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL
);
CREATE INDEX idx_cached_images_url ON cached_images(original_url);
```

**Migration 002** — Proper media cache with real columns (replacing JSON blob):

```sql
CREATE TABLE cached_media (
    id           INTEGER PRIMARY KEY,
    anilist_id   INTEGER NOT NULL UNIQUE,
    title_romaji TEXT,
    title_english TEXT,
    title_native TEXT,
    media_type   TEXT NOT NULL,
    format       TEXT,
    status       TEXT,
    cover_large  TEXT,
    cover_medium TEXT,
    banner_image TEXT,
    avg_score    INTEGER,
    mean_score   INTEGER,
    episodes     INTEGER,
    chapters     INTEGER,
    volumes      INTEGER,
    season       TEXT,
    season_year  INTEGER,
    genres       TEXT,
    is_adult     INTEGER DEFAULT 0,
    site_url     TEXT,
    cached_at    INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL,
    full_json    TEXT
);
CREATE INDEX idx_cached_media_type ON cached_media(media_type);
CREATE INDEX idx_cached_media_score ON cached_media(avg_score);
```

**Migration 003** — Recently viewed:

```sql
CREATE TABLE recently_viewed (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id  INTEGER NOT NULL UNIQUE,
    media_type TEXT,
    title     TEXT,
    cover_url TEXT,
    viewed_at INTEGER NOT NULL
);
CREATE INDEX idx_recently_viewed_time ON recently_viewed(viewed_at DESC);
```

**Migration 004** — User cache:

```sql
CREATE TABLE cached_users (
    id           INTEGER PRIMARY KEY,
    anilist_id   INTEGER NOT NULL UNIQUE,
    name         TEXT NOT NULL,
    avatar_large TEXT,
    banner_image TEXT,
    about        TEXT,
    site_url     TEXT,
    cached_at    INTEGER NOT NULL,
    full_json    TEXT
);
```

**Migration 005** — Local progress tracking:

```sql
CREATE TABLE local_progress (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id         INTEGER NOT NULL UNIQUE,
    entry_id         INTEGER,
    media_type       TEXT NOT NULL,
    status           TEXT NOT NULL,
    progress         INTEGER NOT NULL DEFAULT 0,
    progress_volumes INTEGER DEFAULT 0,
    score            REAL DEFAULT 0,
    notes            TEXT,
    private          INTEGER DEFAULT 0,
    repeat           INTEGER DEFAULT 0,
    started_year     INTEGER,
    started_month    INTEGER,
    started_day      INTEGER,
    finished_year    INTEGER,
    finished_month   INTEGER,
    finished_day     INTEGER,
    updated_at       INTEGER NOT NULL,
    created_at       INTEGER NOT NULL,
    synced_at        INTEGER
);
CREATE INDEX idx_local_progress_status ON local_progress(status);
CREATE INDEX idx_local_progress_type   ON local_progress(media_type);
```

**Migration 006** — Notifications cache:

```sql
CREATE TABLE notifications_cache (
    id              INTEGER PRIMARY KEY,
    notification_id INTEGER NOT NULL UNIQUE,
    type            TEXT NOT NULL,
    is_read         INTEGER DEFAULT 0,
    created_at      INTEGER NOT NULL,
    cached_at       INTEGER NOT NULL,
    full_json       TEXT NOT NULL
);
CREATE INDEX idx_notifications_unread ON notifications_cache(is_read);
CREATE INDEX idx_notifications_time   ON notifications_cache(created_at DESC);
```

**Migration 007** — Activity feed cache:

```sql
CREATE TABLE activities_cache (
    id          INTEGER PRIMARY KEY,
    activity_id INTEGER NOT NULL UNIQUE,
    type        TEXT NOT NULL,
    user_id     INTEGER,
    created_at  INTEGER NOT NULL,
    cached_at   INTEGER NOT NULL,
    full_json   TEXT NOT NULL
);
CREATE INDEX idx_activities_user   ON activities_cache(user_id);
CREATE INDEX idx_activities_time   ON activities_cache(created_at DESC);
```

**Migration 008** — Reviews cache:

```sql
CREATE TABLE reviews_cache (
    id        INTEGER PRIMARY KEY,
    review_id INTEGER NOT NULL UNIQUE,
    media_id  INTEGER NOT NULL,
    user_id   INTEGER NOT NULL,
    score     INTEGER,
    rating    INTEGER,
    ratingAmount INTEGER,
    created_at INTEGER NOT NULL,
    cached_at  INTEGER NOT NULL,
    full_json  TEXT NOT NULL
);
CREATE INDEX idx_reviews_media ON reviews_cache(media_id);
CREATE INDEX idx_reviews_user  ON reviews_cache(user_id);
```

### 1.3 Replace `Arc<Mutex<Connection>>` with R2D2 connection pool + WAL mode

**Current problem**: A `Mutex<Connection>` means only one thread can access the DB at any time. With the UI making many queries simultaneously, this creates a bottleneck.

**Solution**:

```rust
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self> {
        let manager = SqliteConnectionManager::file(&path)
            .with_init(|conn| {
                conn.execute_batch("
                    PRAGMA journal_mode=WAL;
                    PRAGMA synchronous=NORMAL;
                    PRAGMA foreign_keys=ON;
                    PRAGMA cache_size=-20000;
                    PRAGMA temp_store=MEMORY;
                ")?;
                Ok(())
            });
        let pool = Pool::builder()
            .max_size(8)
            .build(manager)?;
        let conn = pool.get()?;
        migrations::runner().run(&conn)?;
        Ok(Self { pool })
    }

    pub fn get(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.pool.get().expect("Failed to get DB connection from pool")
    }
}
```

### 1.4 Fix log directory

**Current**: `/tmp/zafkiel` — deleted on every reboot.

**Fix**: Use `app.path().app_log_dir()` which resolves to `~/.local/share/zafkiel/logs/` on Linux.

### 1.5 Simplify `AniListService`

**Current**: `Arc<RwLock<AniListClient>>` — the outer `RwLock` is redundant because `AniListClient` is already `Clone` with internal `Arc<ClientInner>`.

**Fix**: Hold `AniListClient` directly. Token updates recreate the client. Removes async lock contention on every API call.

---

## Part 2 — Backend: New Tauri Commands

### 2.1 MediaList Commands

**New file**: `src-tauri/src/commands/api/medialist.rs`

| Command                                                                                         | Auth Required | Description                     |
| ----------------------------------------------------------------------------------------------- | ------------- | ------------------------------- |
| `get_my_anime_list(status?)`                                                                    | ✅            | Authenticated user's anime list |
| `get_my_manga_list(status?)`                                                                    | ✅            | Authenticated user's manga list |
| `get_user_anime_list(username, status?)`                                                        | ❌            | Any user's public anime list    |
| `get_user_manga_list(username, status?)`                                                        | ❌            | Any user's public manga list    |
| `save_media_list_entry(media_id, status, progress?, score?, notes?, started_at?, finished_at?)` | ✅            | Create or update a list entry   |
| `delete_media_list_entry(entry_id)`                                                             | ✅            | Remove from list entirely       |
| `update_list_progress(entry_id, progress)`                                                      | ✅            | Quick progress update           |
| `update_list_score(entry_id, score)`                                                            | ✅            | Quick score update              |
| `update_list_status(entry_id, status)`                                                          | ✅            | Quick status change             |

### 2.2 Activity Feed Commands

**New file**: `src-tauri/src/commands/api/activity.rs`

| Command                                               | Auth Required | Description                  |
| ----------------------------------------------------- | ------------- | ---------------------------- |
| `get_activity_feed(page?, per_page?)`                 | ❌            | Global activity feed         |
| `get_following_activity(page?, per_page?)`            | ✅            | Activity from followed users |
| `get_user_activities(user_id, page?, per_page?)`      | ❌            | Specific user's activities   |
| `get_activity_by_id(id)`                              | ❌            | Single activity detail       |
| `get_activity_replies(activity_id, page?, per_page?)` | ❌            | Replies to activity          |
| `create_text_activity(text)`                          | ✅            | Post a text status update    |
| `create_message_activity(recipient_id, message)`      | ✅            | Send a message to user       |
| `reply_to_activity(activity_id, text)`                | ✅            | Reply to any activity        |
| `delete_activity(id)`                                 | ✅            | Delete own activity          |
| `delete_activity_reply(id)`                           | ✅            | Delete own reply             |
| `toggle_activity_pin(id, pinned)`                     | ✅            | Pin/unpin activity           |
| `toggle_activity_subscription(activity_id)`           | ✅            | Subscribe to activity        |

### 2.3 Forum Thread Commands

**New file**: `src-tauri/src/commands/api/forum.rs`

| Command                                                  | Auth Required | Description                 |
| -------------------------------------------------------- | ------------- | --------------------------- |
| `get_recent_forum_threads(page?, per_page?)`             | ❌            | Recent forum threads        |
| `get_popular_forum_threads(page?, per_page?)`            | ❌            | Popular/trending threads    |
| `search_forum_threads(query, page?, per_page?)`          | ❌            | Search threads by keyword   |
| `get_forum_thread(thread_id)`                            | ❌            | Full thread with details    |
| `get_thread_comments(thread_id, page?, per_page?)`       | ❌            | Thread comment list         |
| `get_threads_by_category(category_id, page?, per_page?)` | ❌            | Filter by category          |
| `get_threads_by_user(user_id, page?, per_page?)`         | ❌            | User's threads              |
| `get_subscribed_threads(page?, per_page?)`               | ✅            | My subscribed threads       |
| `create_forum_thread(title, body, categories?)`          | ✅            | Create new thread           |
| `reply_to_forum_thread(thread_id, text)`                 | ✅            | Post a comment              |
| `reply_to_forum_comment(comment_id, text)`               | ✅            | Reply to a comment          |
| `edit_forum_comment(comment_id, text)`                   | ✅            | Edit own comment            |
| `delete_forum_thread(thread_id)`                         | ✅            | Delete own thread           |
| `delete_forum_comment(comment_id)`                       | ✅            | Delete own comment          |
| `subscribe_to_thread(thread_id)`                         | ✅            | Subscribe for notifications |
| `unsubscribe_from_thread(thread_id)`                     | ✅            | Unsubscribe                 |

### 2.4 Notification Commands

**New file**: `src-tauri/src/commands/api/notification.rs`

| Command                                  | Auth Required | Description                       |
| ---------------------------------------- | ------------- | --------------------------------- |
| `get_notifications(page?, per_page?)`    | ✅            | Fetch all notifications           |
| `get_notifications_by_type(type, page?)` | ✅            | Filter by notification type       |
| `get_and_mark_notifications_read()`      | ✅            | Fetch + mark all read             |
| `get_unread_notification_count()`        | ✅            | Just the unread count (for badge) |

### 2.5 Review Commands

**New file**: `src-tauri/src/commands/api/review.rs`

| Command                                                 | Auth Required | Description                |
| ------------------------------------------------------- | ------------- | -------------------------- |
| `get_media_reviews(media_id, page?, per_page?)`         | ❌            | Reviews for an anime/manga |
| `get_user_reviews(user_id, page?, per_page?)`           | ❌            | User's written reviews     |
| `get_review_by_id(id)`                                  | ❌            | Single review detail       |
| `get_recent_reviews(page?, per_page?)`                  | ❌            | Recently posted reviews    |
| `save_review(media_id, body, summary, score, private?)` | ✅            | Create/update review       |
| `delete_review(review_id)`                              | ✅            | Delete own review          |
| `rate_review(review_id, rating)`                        | ✅            | Rate helpful/not-helpful   |

### 2.6 Recommendation Commands

**New file**: `src-tauri/src/commands/api/recommendation.rs`

| Command                                                          | Auth Required | Description                      |
| ---------------------------------------------------------------- | ------------- | -------------------------------- |
| `get_media_recommendations(media_id, page?, per_page?)`          | ❌            | Recommendations for a media      |
| `get_recent_recommendations(page?, per_page?)`                   | ❌            | Recent site-wide recommendations |
| `rate_recommendation(media_id, recommendation_media_id, rating)` | ✅            | Upvote/downvote                  |

### 2.7 Airing Schedule Commands

Extend existing or new file `src-tauri/src/commands/api/airing.rs`:

| Command                                 | Auth Required | Description                    |
| --------------------------------------- | ------------- | ------------------------------ |
| `get_airing_schedule(page?, per_page?)` | ❌            | Upcoming airing schedule       |
| `get_recent_airing(page?, per_page?)`   | ❌            | Recently aired episodes        |
| `get_airing_by_media_id(media_id)`      | ❌            | Airing info for specific anime |

### 2.8 User Social Commands

Extend user commands or new file `src-tauri/src/commands/api/social.rs`:

| Command                                         | Auth Required | Description                                     |
| ----------------------------------------------- | ------------- | ----------------------------------------------- |
| `get_user_followers(user_id, page?, per_page?)` | ❌            | User's followers                                |
| `get_user_following(user_id, page?, per_page?)` | ❌            | Users they follow                               |
| `get_user_favorites(user_id)`                   | ❌            | Favorited anime/manga/chars/staff               |
| `toggle_favourite(type, id)`                    | ✅            | Toggle favorite (anime/manga/char/staff/studio) |
| `follow_user(user_id)`                          | ✅            | Follow a user                                   |
| `unfollow_user(user_id)`                        | ✅            | Unfollow a user                                 |
| `get_user_stats(user_id)`                       | ❌            | Full statistics including genre/tag breakdown   |

### 2.9 DB Command Fixes

| Command                     | Old State                                 | New State                             |
| --------------------------- | ----------------------------------------- | ------------------------------------- |
| `cache_media`               | Inserts unstructured JSON (table missing) | Upsert into normalized `cached_media` |
| `cache_user`                | Inserts unstructured JSON (table missing) | Upsert into normalized `cached_users` |
| `add_to_recently_viewed`    | No dedup (table missing)                  | UPSERT with title/cover params        |
| `update_local_progress`     | Fails — table missing                     | Works with proper params              |
| `upsert_notification_cache` | **New**                                   | Cache notification objects            |
| `upsert_activity_cache`     | **New**                                   | Cache activity objects                |
| `get_cached_media_list`     | **New**                                   | Read `local_progress` entries         |
| `sync_media_list_entry`     | **New**                                   | Write entry after AniList sync        |
| `get_unsynced_progress`     | **New**                                   | Get entries needing AniList sync      |

---

## Part 3 — Frontend: TypeScript Type Additions

### 3.1 New types in `src/lib/types/anilist.ts`

```typescript
// MediaList (user's list entry)
export interface MediaList {
  id: number;
  mediaId: number;
  status: MediaListStatus;
  score?: number;
  progress?: number;
  progressVolumes?: number;
  repeat?: number;
  priority?: number;
  private?: boolean;
  hiddenFromStatusLists?: boolean;
  notes?: string;
  startedAt?: MediaDate;
  completedAt?: MediaDate;
  updatedAt?: number;
  createdAt?: number;
  media?: Media;
  user?: User;
}

// Activities
export type ActivityType = 'TEXT' | 'ANIME_LIST' | 'MANGA_LIST' | 'MESSAGE_ACTIVITY';

export interface TextActivity { /* ... */ }
export interface ListActivity { /* ... */ }
export interface MessageActivity { /* ... */ }
export type ActivityUnion = TextActivity | ListActivity | MessageActivity;
export interface ActivityReply { /* ... */ }

// Forum
export interface Thread { /* ... */ }
export interface ThreadCategory { id: number; name?: string; }
export interface ThreadComment { /* ... */ }

// Review
export interface Review { /* ... */ }

// Recommendation
export interface Recommendation { /* ... */ }

// Airing
export interface AiringSchedule { /* ... */ }

// Notifications
export type NotificationType = 'ACTIVITY_MESSAGE' | 'ACTIVITY_REPLY' | 'FOLLOWING' | 'AIRING' | ...;
export interface Notification { /* ... */ }

// Favorites
export interface UserFavourites { /* ... */ }

// Extended Statistics
export interface UserGenreStat { /* ... */ }
export interface UserTagStat { /* ... */ }
export interface UserStatisticsFull { /* ... */ }
```

### 3.2 New TanStack Query hooks in `useAnilist.svelte.ts`

New query key factories:

- `medialist.*` — `myAnime`, `myManga`, `userAnime`, `userManga`
- `activity.*` — `feed`, `following`, `user`, `byId`, `replies`
- `forum.*` — `recent`, `popular`, `search`, `thread`, `comments`
- `notification.*` — `list`, `unreadCount`
- `review.*` — `forMedia`, `forUser`, `byId`
- `recommendation.*` — `forMedia`
- `airing.*` — `schedule`, `recent`, `forMedia`
- `social.*` — `followers`, `following`, `favorites`

New hooks (queries):

- `useMyAnimeList(status?)` / `useMyMangaList(status?)`
- `useUserAnimeList(username, status?)` / `useUserMangaList(username, status?)`
- `useActivityFeed(params?)` / `useFollowingActivity(params?)`
- `useUserActivities(userId, params?)`
- `useActivityReplies(activityId)`
- `useForumThreads(params?)` / `useForumThread(threadId)` / `useThreadComments(threadId)`
- `useNotifications(params?)` / `useUnreadNotificationCount()` (polled every 60s)
- `useMediaReviews(mediaId, params?)` / `useUserReviews(userId, params?)`
- `useMediaRecommendations(mediaId, params?)`
- `useAiringSchedule(params?)` / `useRecentAiring(params?)` / `useMediaAiring(mediaId)`
- `useUserFollowers(userId)` / `useUserFollowing(userId)` / `useUserFavorites(userId)`

New hooks (mutations):

- `useSaveMediaListEntry()` — save/update list entry
- `useDeleteMediaListEntry()` — delete list entry
- `useUpdateListProgress()` — quick progress +1 (optimistic)
- `useUpdateListScore()` — quick score update
- `useUpdateListStatus()` — quick status change
- `useCreateTextActivity()` / `useCreateMessage()` / `useReplyToActivity()`
- `useDeleteActivity()` / `useDeleteActivityReply()`
- `useCreateForumThread()` / `useReplyToThread()` / `useReplyToComment()`
- `useEditForumComment()` / `useDeleteForumThread()` / `useDeleteForumComment()`
- `useSubscribeToThread()`
- `useMarkNotificationsRead()`
- `useSaveReview()` / `useDeleteReview()` / `useRateReview()`
- `useRateRecommendation()`
- `useToggleFavorite()` — toggle any entity type
- `useFollowUser()` / `useUnfollowUser()`

**Fix existing stubs**:

- `useUpdateProgress` → call `update_list_progress` command
- `useAddToList` → call `save_media_list_entry` command
- `useRemoveFromList` → call `delete_media_list_entry` command

---

## Part 4 — Frontend: New Components

### 4.1 `ActivityCard.svelte`

Polymorphic component rendering any `ActivityUnion` type:

- **ListActivity**: User avatar + name, activity text ("watched episode X"), cover thumbnail right, like/reply counts, time ago
- **TextActivity**: Avatar + name, markdown body, like/reply buttons, time ago
- **MessageActivity**: "Messenger → Recipient" header, message body
- Design: `bg-card border rounded-lg p-4`, left accent border by type, type-specific icons

### 4.2 `ReviewCard.svelte`

- User avatar, name, score badge (green ≥70, yellow 50-69, red <50)
- Summary in bold, truncated body with "Read more"
- Helpful/unhelpful bar (proportional)
- Rate buttons (auth-gated), time from creation

### 4.3 `NotificationItem.svelte`

- Type icon (solar:\* duotone matching notification type)
- Contextual message built from notification fields
- Time ago on right
- Unread indicator: left accent border + brighter background

### 4.4 `ThreadCard.svelte`

Forum thread list card:

- Title + category badges
- Author + time
- Reply/view/like counts
- Pinned/locked badges
- Last reply info

### 4.5 `ThreadCommentCard.svelte`

- Avatar + name, markdown body, like + reply + edit/delete (own)
- Nested children up to 2 levels deep, collapsible
- "Replying to @user" header for nested

### 4.6 `MediaListRow.svelte`

Table row for list view:

- Cover thumbnail, title link
- Inline-editable progress `X / Y` with +1 button
- Status badge, score (inline editable)
- Format, air date, quick actions

### 4.7 `MediaListGrid.svelte`

Grid view card:

- Cover (tall aspect ratio)
- Hover overlays: title, progress, score, status

### 4.8 `ScoreDisplay.svelte`

Reusable score display, color-coded:

- Green ≥70, Amber 50-69, Red <50, Gray N/A
- Supports: number badge, circle/ring, stars

### 4.9 `StatusBadge.svelte`

List status badge with per-status colors:

- CURRENT → Blue "Watching/Reading"
- COMPLETED → Green
- PLANNING → Purple
- DROPPED → Red
- PAUSED → Yellow
- REPEATING → Cyan

### 4.10 `AiringCard.svelte`

Schedule page card:

- Cover (small), anime title, episode number
- Live countdown (days/hours/minutes)

### 4.11 `MarkdownRenderer.svelte`

AniList markdown + HTML renderer:

- `__bold__`, `~~strikethrough~~`, `~small~`, `[Spoiler: text]` collapsible
- Sanitized HTML (no script/iframe)
- Inline images

### 4.12 `UserCard.svelte`

Compact user card:

- Avatar (circle), name + donation/mod badges
- Follow button, click → profile

### 4.13 `InlineProgressEditor.svelte`

- Shows `current / total`
- Click → editable number input
- +1 button, checkmark to complete on last episode
- Optimistic update + sync on blur/enter

### 4.14 `AiringCountdown.svelte`

Shared countdown: `Xd Xh Xm` until next episode, updates every second.

### 4.15 Updated `MediaCard.svelte`

Add:

- List status overlay (bottom-left badge): "Watching ep 5"
- Quick +1 episode on hover (for CURRENT entries)
- "Add to List" button for non-listed media
- Favorite heart indicator (yellow when favorited)

### 4.16 Updated `TitleBar.svelte`

- Notification bell icon with unread count badge (polled 60s, auth-gated)

### 4.17 Updated `ProfileDropdown.svelte`

- "My Activity" → `/social?filter=mine`
- "Notifications" with badge
- "My List" → `/list`

### 4.18 Updated `ActionButtonStrip.svelte`

Wire actual API calls:

- "Add to List" → `useSaveMediaListEntry()` (PLANNING)
- "Mark Watching" → (CURRENT)
- "Mark Completed" → (COMPLETED)
- "Favorite" → `useToggleFavorite()` (toggle)
- Loading states, error/success toasts via `svelte-sonner`

---

## Part 5 — Frontend: New Pages

### 5.1 `/list` and `/list/[username]` — Media List Page

**Layout**:

- Status tabs: All | Watching | Completed | Paused | Dropped | Planning | Repeating
- Search input + sort dropdown + view toggle (table/grid)
- Table view: `MediaListRow.svelte`
- Grid view: `MediaListGrid.svelte`

**Behavior**:

- Own list: inline editing
- Other user: read-only
- URL params: `/list/username?status=CURRENT&sort=UPDATED_AT_DESC`
- Virtual scrolling for large lists

### 5.2 `/social` — Activity Feed Page

**Layout**:

- Two-column: feed 70% + sidebar 30%
- Tabs: "Global" | "Following"
- Post box at top (auth)
- `ActivityCard.svelte` infinite scroll
- Sidebar: stats, trending

### 5.3 `/notifications` — Notifications Page

**Layout**:

- Max-w-3xl centered, "Mark All Read" button
- Filter chips: All | Airing | Messages | Replies | Social | Media Updates
- `NotificationItem.svelte` list
- Auto-mark-read after 5s (configurable)

### 5.4 `/forum` — Forum Landing Page

**Layout**:

- Left sidebar: categories, "Recent/Popular/Subscribed" filters
- `ThreadCard.svelte` main list
- Search bar, "New Thread" button (auth)

### 5.5 `/forum/[id]` — Forum Thread Detail

**Layout**:

- Breadcrumb, original post card, meta (views/likes/subscribe)
- `ThreadCommentCard.svelte` paginated list
- Reply box at bottom (auth), markdown preview toggle

### 5.6 `/schedule` — Airing Schedule Page

**Layout**:

- Week view: 7 columns (Mon-Sun)
- Today highlighted
- `AiringCard.svelte` with live `AiringCountdown`
- Toggle: "All" vs "My List Only" (auth)

### 5.7 Enhanced User Profile — `/user/[id]`

New tabs (in addition to existing Overview + Statistics):

- **Anime List**: Mini-list with status filter tabs, link to full `/list/[username]?type=anime`
- **Manga List**: Same for manga
- **Activities**: Last 20 from `useUserActivities(userId)`, "View All" link
- **Reviews**: `useUserReviews(userId)` via `ReviewCard.svelte`
- **Favorites**: `useUserFavorites(userId)` — segmented: Anime, Manga, Characters, Staff, Studios
- **Social**: Followers/following counts with `UserCard.svelte` lists, Follow/Unfollow button (auth)

Header enhancements:

- Follow/Unfollow button (own profile hidden)
- Follower/following count chips
- Enhanced stats: score distribution bar chart, top genres/tags breakdown, format/status pie bars

### 5.8 Enhanced Anime/Manga Detail — `/anime/[id]`

Additions:

- **Reviews tab**: Connect real `useMediaReviews(animeId)` + `ReviewCard.svelte`
- **Recommendations tab** (new): `useMediaRecommendations(animeId)` grid with rate buttons
- **List Entry panel** (sidebar): If authenticated + in list — status/progress/score/notes editable via `InlineProgressEditor.svelte`
- **Add to List button**: Prominent if not in list
- **Favorite button**: Wired to `useToggleFavorite()`
- **Airing info**: `AiringCountdown` if currently airing
- **External links**: Show streaming sites from `externalLinks` field

### 5.9 Enhanced Dashboard — `/`

Replace hardcoded stats:

- Stats from `useCurrentUser()` → `user.statistics`
- "Continue Watching" strip: `useMyAnimeList('CURRENT')` top 5 by updated
- "Airing Today" section: `useAiringSchedule()` filtered to today
- "Recent Activity" preview: last 3 from `useFollowingActivity()`
- "Seasonal Anime" section: `useSeasonalAnime()` small grid

---

## Part 6 — Navigation & Routing

### New Route Files

```
src/routes/(app)/list/+page.svelte              -- redirect to /list/[me]
src/routes/(app)/list/[username]/+page.svelte   -- media list view
src/routes/(app)/list/[username]/+page.ts       -- preload logic
src/routes/(app)/social/+page.svelte            -- activity feed
src/routes/(app)/notifications/+page.svelte     -- notifications
src/routes/(app)/forum/+page.svelte             -- forum listing
src/routes/(app)/forum/[id]/+page.svelte        -- thread detail
src/routes/(app)/forum/create/+page.svelte      -- create thread
src/routes/(app)/schedule/+page.svelte          -- airing schedule
src/routes/(app)/profile/+page.svelte           -- redirect to own profile
```

### Sidebar/Navigation Updates

Add navigation items (with solar:\* icons):

- **My List** (`/list`) — `solar:list-bold`
- **Schedule** (`/schedule`) — `solar:calendar-bold`
- **Social** (`/social`) — `solar:users-group-two-rounded-bold`
- Notification badge in TitleBar

Order: Home → Browse Anime → Browse Manga → My List → Schedule → Social → Search → Settings

---

## Part 7 — Code Quality, Performance & Architecture

### 7.1 Rust Backend Performance

**R2D2 pool** (covered in Part 1.3) — async DB calls via `tokio::task::spawn_blocking`

**Rate limiting**: Add `Arc<tokio::sync::Semaphore>` (90 permits/minute) to prevent 429 errors:

```rust
// Acquire before every AniList request
let _permit = rate_limiter.acquire().await;
```

**Remove `block_on` in setup**: Move torrent session init to proper async pattern.

### 7.2 Frontend Performance

**Virtual scrolling**: Use `@tanstack/svelte-virtual` for `MediaListRow` — only renders visible rows.

**Infinite scroll**: Replace "load more" with `IntersectionObserver` → `createInfiniteQuery` for feeds.

**Skeleton loaders**:

- `MediaCardSkeleton.svelte`
- `ActivityCardSkeleton.svelte`
- `MediaListRowSkeleton.svelte`

**Debounced search**: 300ms debounce, `enabled: debouncedQuery.length > 2` gate.

**`createInfiniteQuery`**: Activity, forum, notifications, list pages use `fetchNextPage`.

### 7.3 Database Performance

**Prepared statement caching** with rusqlite's `cached_statement`.

**Background cache cleanup task** (runs hourly):

- `cached_media` rows older than 7 days (by `last_accessed`)
- `cached_images` rows older than 30 days
- `activities_cache` rows older than 3 days
- `notifications_cache` rows older than 7 days

### 7.4 Error Handling

**Rust `AppError` enum**:

```rust
#[derive(thiserror::Error, Debug, serde::Serialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("AniList API error: {0}")]
    AniList(String),
    #[error("Authentication required")]
    Unauthenticated,
    #[error("Not found: {0}")]
    NotFound(String),
}
```

All commands return `Result<T, AppError>` for structured frontend error handling.

### 7.5 New Settings Panels

**List Settings** (`ListSettings.svelte`):

- Default view: Table / Grid
- Default sort, score format (10pt / 100pt / Stars / Smiley), show private, adult content

**Notification Settings** (`NotificationSettings.svelte`):

- Per-type toggles, polling interval, auto-mark-read

**Profile Edit** (extend `AccountSettings.svelte`):

- Edit bio/about textarea

---

## Part 8 — Storybook & Testing

### Stories to Create

- `ActivityCard.stories.ts` — text/list/message types
- `ReviewCard.stories.ts` — scored/anonymous variants
- `NotificationItem.stories.ts` — multiple types
- `ThreadCard.stories.ts` — normal/pinned/locked
- `MediaListRow.stories.ts` — editable/readonly
- `AiringCard.stories.ts` — airing/aired
- `ScoreDisplay.stories.ts` — all ranges + formats
- `UserCard.stories.ts` — follow/unfollow state

### Unit Tests

- `formatAniListMarkdown.test.ts`
- `scoreToColor.test.ts`
- `activityTextFormatter.test.ts`
- `countdownFormatter.test.ts`

### E2E Tests

- `anime-list.test.ts`
- `forum.test.ts`
- `notifications.test.ts`
- `activity-feed.test.ts`

---

## Implementation Order (Phase Sequence)

| Phase       | Scope                                                                                                                     | Status  |
| ----------- | ------------------------------------------------------------------------------------------------------------------------- | ------- |
| **Phase A** | DB infrastructure: migrations, R2D2 pool, schema fix, log path fix, AniListService simplification                         | ⬜ TODO |
| **Phase B** | MediaList backend commands + frontend types/hooks + fix stubs + list entry panel on detail page                           | ⬜ TODO |
| **Phase C** | User profile enhancements + `/list/[username]` page                                                                       | ⬜ TODO |
| **Phase D** | Activity + notification backend commands + `ActivityCard` component + `/social` + `/notifications` pages + TitleBar badge | ⬜ TODO |
| **Phase E** | Reviews + recommendations + airing backend + components + tabs on detail page + `/schedule` page                          | ⬜ TODO |
| **Phase F** | Forum backend commands + `ThreadCard`/`ThreadCommentCard` + `/forum` + `/forum/[id]` pages                                | ⬜ TODO |
| **Phase G** | Dashboard real data + virtual scrolling + infinite scroll + skeleton loaders + Storybook + tests                          | ⬜ TODO |

---

## Visual Design Consistency

All new pages follow the existing design language:

- **Theming**: `bg-background`, `text-foreground`, `text-primary`, `bg-card border rounded-lg`
- **Tabs**: Borderless with `border-b-2 border-primary` underline on active tab (matching anime detail + user profile pages)
- **Loading states**: `solar:refresh-circle-line-duotone` + `animate-spin` (matches existing)
- **Icons**: `@iconify/svelte` with `solar:*` duotone (matches TitleBar, Sidebar)
- **Colors**: All CSS variables — no hardcoded colors
- **Typography**: Tailwind scale + `@tailwindcss/typography` for markdown content
- **Responsive**: `container mx-auto max-w-7xl px-4` + responsive grid
- **Transitions**: `fly`, `fade`, `scale` from Svelte (no custom CSS transitions)
- **Toasts**: `svelte-sonner` for all mutation feedback
