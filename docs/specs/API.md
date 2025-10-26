# PHASE 1: API Specification

**Status**: Part of Phase 1 - Foundation
**Reference**: [PHASE-1-API]

## Overview

This document defines all Tauri IPC commands (API endpoints) for Unreal RSS Phase 1. All commands follow a consistent pattern and are fully type-safe.

## Design Principles

1. **Async First** - All commands are async
2. **Input Validation** - Validate all arguments at boundary
3. **Type Safety** - Clear input/output types
4. **Error Handling** - Clear error messages
5. **No Leaking Internals** - Abstract internal errors
6. **Consistent Naming** - snake_case for function names

## API Patterns

### Standard Command Pattern

```rust
#[tauri::command]
pub async fn command_name(
    arg1: String,
    arg2: i32,
    state: State<'_, AppState>,
) -> Result<ReturnType, String> {
    // 1. Validate inputs
    if arg1.is_empty() {
        return Err("arg1 cannot be empty".to_string());
    }

    // 2. Acquire database connection
    let db = state.db.lock().map_err(|e| e.to_string())?;

    // 3. Perform operation
    let result = db.query_row(...)?
        .map_err(|e| format!("Database error: {}", e))?;

    // 4. Return result
    Ok(result)
}
```

### Error Response Pattern

```rust
// Good - Context provided
Err("Failed to fetch feed 'https://example.com': Connection timeout".to_string())

// Bad - No context
Err("error".to_string())
```

### Type Safety Pattern

```typescript
// Frontend type-safe call
const result = await invoke<Feed>('add_feed', { url: feedUrl })
    .catch(error => console.error('Add feed failed:', error));
```

## Feed Commands

### add_feed

Add a new RSS feed.

**Command:**
```rust
#[tauri::command]
pub async fn add_feed(
    url: String,
    state: State<'_, AppState>,
) -> Result<Feed, String>
```

**Input:**
```typescript
{
  url: string  // RSS feed URL (HTTP/HTTPS)
}
```

**Output (Success):**
```typescript
{
  id: string,
  url: string,
  title: string,
  description?: string,
  last_updated?: number,
  created_at: number
}
```

**Output (Error):**
```typescript
"Feed URL must start with http:// or https://"
"Invalid feed URL: https://example.com"
"Feed already exists: https://example.com"
"Failed to fetch feed: Connection timeout"
"Failed to parse feed: Invalid RSS format"
```

**Validation:**
- [ ] URL not empty
- [ ] URL is valid HTTP/HTTPS
- [ ] URL is not already in database
- [ ] Feed is accessible and valid
- [ ] Feed title not empty

**Side Effects:**
- Inserts feed into feeds table
- Fetches and inserts articles into articles table
- Updates UI with new feed and articles

### get_feeds

Get all feeds.

**Command:**
```rust
#[tauri::command]
pub async fn get_feeds(
    state: State<'_, AppState>,
) -> Result<Vec<FeedWithUnread>, String>
```

**Input:**
```typescript
// No arguments
```

**Output (Success):**
```typescript
[
  {
    id: string,
    url: string,
    title: string,
    description?: string,
    last_updated?: number,
    created_at: number,
    unread_count: number  // Computed from articles
  },
  // ... more feeds
]
```

**Output (Error):**
```typescript
"Database error: cannot access database"
```

**Performance:**
- O(n) where n = number of feeds
- Includes unread count for each feed
- Sorted by creation date (newest first)

### delete_feed

Delete a feed and all its articles.

**Command:**
```rust
#[tauri::command]
pub async fn delete_feed(
    feed_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  feed_id: string  // UUID of feed to delete
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"feed_id cannot be empty"
"Feed not found: {feed_id}"
"Database error: cannot delete feed"
```

**Validation:**
- [ ] feed_id not empty
- [ ] feed_id exists in database

**Side Effects:**
- Deletes feed from feeds table
- Cascades delete to all articles (due to FK)
- Updates UI to remove feed

### get_feed_details

Get detailed information about a specific feed.

**Command:**
```rust
#[tauri::command]
pub async fn get_feed_details(
    feed_id: String,
    state: State<'_, AppState>,
) -> Result<FeedDetails, String>
```

**Input:**
```typescript
{
  feed_id: string
}
```

**Output (Success):**
```typescript
{
  id: string,
  url: string,
  title: string,
  description?: string,
  last_updated?: number,
  created_at: number,
  total_articles: number,
  unread_count: number,
  starred_count: number
}
```

**Output (Error):**
```typescript
"feed_id cannot be empty"
"Feed not found"
```

## Article Commands

### get_articles

Get articles for a specific feed.

**Command:**
```rust
#[tauri::command]
pub async fn get_articles(
    feed_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Article>, String>
```

**Input:**
```typescript
{
  feed_id: string,
  limit?: number,        // Default: 50, Max: 1000
  offset?: number        // Default: 0
}
```

**Output (Success):**
```typescript
[
  {
    id: string,
    feed_id: string,
    title: string,
    description?: string,
    content?: string,
    url?: string,
    pub_date?: number,
    created_at: number,
    is_read: boolean,
    is_starred: boolean
  },
  // ... more articles (up to limit)
]
```

**Output (Error):**
```typescript
"feed_id cannot be empty"
"Feed not found"
"Database error"
```

**Validation:**
- [ ] feed_id not empty
- [ ] feed_id exists
- [ ] limit: 1-1000
- [ ] offset: >= 0

**Performance:**
- Paginated (limit/offset)
- Sorted by pub_date DESC
- Should return in <100ms

### get_article

Get a single article by ID.

**Command:**
```rust
#[tauri::command]
pub async fn get_article(
    article_id: String,
    state: State<'_, AppState>,
) -> Result<Article, String>
```

**Input:**
```typescript
{
  article_id: string
}
```

**Output (Success):**
```typescript
{
  id: string,
  feed_id: string,
  title: string,
  description?: string,
  content?: string,
  url?: string,
  pub_date?: number,
  created_at: number,
  is_read: boolean,
  is_starred: boolean
}
```

**Output (Error):**
```typescript
"article_id cannot be empty"
"Article not found"
```

**Note:** This command can also mark the article as read (Phase 2 optimization).

### mark_as_read

Mark an article as read.

**Command:**
```rust
#[tauri::command]
pub async fn mark_as_read(
    article_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  article_id: string
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"article_id cannot be empty"
"Article not found"
"Database error"
```

**Side Effects:**
- Updates article is_read = true
- UI reflects unread count change

### mark_as_unread

Mark an article as unread.

**Command:**
```rust
#[tauri::command]
pub async fn mark_as_unread(
    article_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  article_id: string
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"article_id cannot be empty"
"Article not found"
"Database error"
```

### star_article

Star (favorite) an article.

**Command:**
```rust
#[tauri::command]
pub async fn star_article(
    article_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  article_id: string
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"article_id cannot be empty"
"Article not found"
"Database error"
```

**Behavior:**
- If article already starred, unstar it (toggle)
- Or add `is_starred` parameter for explicit control

### unstar_article

Remove star from an article.

**Command:**
```rust
#[tauri::command]
pub async fn unstar_article(
    article_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  article_id: string
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"article_id cannot be empty"
"Article not found"
"Database error"
```

## Settings Commands

### get_setting

Get a user preference.

**Command:**
```rust
#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<String, String>
```

**Input:**
```typescript
{
  key: string  // Setting key (e.g., "theme")
}
```

**Output (Success):**
```typescript
"dark"  // JSON-serialized value
```

**Output (Error):**
```typescript
"key cannot be empty"
"Setting not found"
"Database error"
```

**Common Settings:**
- `theme` - "light" | "dark" | "auto"
- `font_family` - "system-ui" | "serif" | etc.
- `font_size` - number (pixels)
- `refresh_interval` - number (seconds)

### set_setting

Set a user preference.

**Command:**
```rust
#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Input:**
```typescript
{
  key: string,      // Setting key
  value: string     // JSON-serialized value
}
```

**Output (Success):**
```typescript
// No output (empty success)
```

**Output (Error):**
```typescript
"key cannot be empty"
"value cannot be empty"
"Invalid setting key"
"Database error"
```

**Validation:**
- [ ] key not empty
- [ ] value not empty
- [ ] key is known setting or allow custom

### get_all_settings

Get all user settings.

**Command:**
```rust
#[tauri::command]
pub async fn get_all_settings(
    state: State<'_, AppState>,
) -> Result<Map<String, String>, String>
```

**Output (Success):**
```typescript
{
  "theme": "dark",
  "font_size": "16",
  "refresh_interval": "300"
}
```

**Output (Error):**
```typescript
"Database error"
```

## Utility Commands

### get_app_version

Get application version.

**Command:**
```rust
#[tauri::command]
pub fn get_app_version() -> String
```

**Output:**
```typescript
"0.1.0"
```

### get_db_stats

Get database statistics.

**Command:**
```rust
#[tauri::command]
pub async fn get_db_stats(
    state: State<'_, AppState>,
) -> Result<DbStats, String>
```

**Output (Success):**
```typescript
{
  total_feeds: number,
  total_articles: number,
  unread_articles: number,
  starred_articles: number,
  db_size_bytes: number
}
```

## Command Registration

All commands must be registered in `main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Feed commands
            add_feed,
            get_feeds,
            delete_feed,
            get_feed_details,
            // Article commands
            get_articles,
            get_article,
            mark_as_read,
            mark_as_unread,
            star_article,
            unstar_article,
            // Settings commands
            get_setting,
            set_setting,
            get_all_settings,
            // Utility
            get_app_version,
            get_db_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Frontend Integration

### Type-Safe Invocation

```typescript
import { invoke } from '@tauri-apps/api/tauri';

// Properly typed
const feed = await invoke<Feed>('add_feed', { url: 'https://example.com/feed' })
    .catch(error => {
        console.error('Failed to add feed:', error);
        return null;
    });

// Always handle errors
if (feed) {
    console.log('Feed added:', feed.title);
}
```

### Batch Operations

```typescript
// Get all feeds, then get their articles
const feeds = await invoke<Feed[]>('get_feeds');
for (const feed of feeds) {
    const articles = await invoke<Article[]>('get_articles', {
        feed_id: feed.id,
        limit: 50
    });
    // Process articles
}
```

## Error Handling

### Common Error Patterns

**Input Validation:**
```
"feed_id cannot be empty"
"Invalid feed URL: {url}"
"limit must be between 1 and 1000, got {limit}"
```

**Resource Not Found:**
```
"Feed not found: {feed_id}"
"Article not found: {article_id}"
```

**Database Errors:**
```
"Database error: {error_message}"
```

**Business Logic:**
```
"Feed already exists: {url}"
"Cannot delete non-empty feed"
```

## Rate Limiting (Phase 1)

No rate limiting in Phase 1 (single-user desktop app).

Phase 2+ will add rate limiting for backend API.

## Versioning

All commands are for **API Version 1**.

Breaking changes in Phase 2+ will be handled by:
1. Adding new commands (e.g., `add_feed_v2`)
2. Deprecating old commands
3. Documenting migration path

---

**Status**: Specification Complete
**Next**: UI-DESIGN.md (User flows and components)
**Tested by**: Integration tests in `src-tauri/tests/`
