# PHASE 1: Database Specification

**Status**: Part of Phase 1 - Foundation
**Reference**: [PHASE-1-DATABASE]

## Overview

This document defines the SQLite database schema for Unreal RSS Phase 1, including table design, relationships, indices, and migration strategy.

## Database Design Principles

1. **Schema Versioning** - Version tracked for future migrations
2. **Normalization** - Avoid data duplication
3. **Referential Integrity** - Foreign key constraints enforced
4. **Performance** - Indices for common queries
5. **Backwards Compatibility** - All changes are additive
6. **Offline First** - All data stored locally

## Schema

### Version: 1

```sql
-- Version 1 Schema
-- Created: Phase 1 - Foundation
```

### Table: feeds

Primary table for RSS feeds.

```sql
CREATE TABLE feeds (
    id TEXT PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    last_updated INTEGER,
    created_at INTEGER NOT NULL,
    CHECK (url != ''),
    CHECK (length(title) > 0)
);

CREATE INDEX idx_feeds_created_at ON feeds(created_at);
CREATE INDEX idx_feeds_updated_at ON feeds(last_updated);
```

**Columns:**

| Column | Type | Nullable | Description |
|--------|------|----------|-------------|
| id | TEXT | No | UUID v4, primary key |
| url | TEXT | No | RSS feed URL, must be unique |
| title | TEXT | No | Feed title from RSS |
| description | TEXT | Yes | Feed description |
| last_updated | INTEGER | Yes | Unix timestamp of last fetch |
| created_at | INTEGER | No | Unix timestamp when added |

**Constraints:**
- `id` PRIMARY KEY - Unique feed identifier
- `url` UNIQUE - No duplicate feeds
- `url` NOT NULL, NOT EMPTY - Valid URL required
- `title` NOT NULL, NOT EMPTY - Title required
- Foreign key constraints enabled

**Indices:**
- `idx_feeds_created_at` - For sorting by addition order
- `idx_feeds_updated_at` - For finding stale feeds

### Table: articles

Articles from feeds.

```sql
CREATE TABLE articles (
    id TEXT PRIMARY KEY,
    feed_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    content TEXT,
    url TEXT,
    pub_date INTEGER,
    created_at INTEGER NOT NULL,
    is_read BOOLEAN DEFAULT 0,
    is_starred BOOLEAN DEFAULT 0,
    FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE,
    CONSTRAINT unique_article UNIQUE (feed_id, url),
    CHECK (length(title) > 0)
);

CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_is_read ON articles(is_read);
CREATE INDEX idx_articles_is_starred ON articles(is_starred);
CREATE INDEX idx_articles_pub_date ON articles(pub_date DESC);
CREATE INDEX idx_articles_created_at ON articles(created_at DESC);
```

**Columns:**

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | TEXT | No | - | UUID v4, primary key |
| feed_id | TEXT | No | - | Foreign key to feeds.id |
| title | TEXT | No | - | Article title |
| description | TEXT | Yes | - | Summary/excerpt |
| content | TEXT | Yes | - | Full HTML (for Phase 2+) |
| url | TEXT | Yes | - | Original article URL |
| pub_date | INTEGER | Yes | - | Unix timestamp (article published) |
| created_at | INTEGER | No | - | Unix timestamp (when fetched) |
| is_read | BOOLEAN | No | 0 | Read status |
| is_starred | BOOLEAN | No | 0 | Starred/favorited |

**Constraints:**
- `id` PRIMARY KEY - Unique article identifier
- `feed_id` FOREIGN KEY - Enforces feed exists
- `ON DELETE CASCADE` - Delete articles when feed deleted
- `(feed_id, url)` UNIQUE - No duplicate articles per feed
- `title` NOT NULL, NOT EMPTY - Title required

**Indices:**
- `idx_articles_feed_id` - For fetching articles by feed
- `idx_articles_is_read` - For unread count queries
- `idx_articles_is_starred` - For starred articles list
- `idx_articles_pub_date` - For sorting by publication date
- `idx_articles_created_at` - For sorting by fetch date

### Table: settings

User preferences and app configuration.

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_settings_key ON settings(key);
```

**Columns:**

| Column | Type | Nullable | Description |
|--------|------|----------|-------------|
| key | TEXT | No | Setting key (e.g., "theme") |
| value | TEXT | No | JSON-serialized value |
| created_at | INTEGER | No | Unix timestamp when created |
| updated_at | INTEGER | No | Unix timestamp when last updated |

**Constraints:**
- `key` PRIMARY KEY - Unique setting identifier

**Indices:**
- `idx_settings_key` - For quick lookups

**Example settings:**

```json
// Theme preference
{ "key": "theme", "value": "\"dark\"" }

// Font configuration
{ "key": "font_family", "value": "\"system-ui\"" }
{ "key": "font_size", "value": "16" }

// Refresh interval (seconds)
{ "key": "refresh_interval", "value": "300" }

// Last sync timestamp
{ "key": "last_sync", "value": "1698345600" }
```

## Schema Diagram

```
┌────────────────┐
│    feeds       │
├────────────────┤
│ id (PK)        │
│ url (UNIQUE)   │
│ title          │
│ description    │
│ last_updated   │
│ created_at     │
└────────────────┘
         │
         │ 1:N
         │
┌────────────────────┐
│    articles        │
├────────────────────┤
│ id (PK)            │
│ feed_id (FK)       │
│ title              │
│ description        │
│ content            │
│ url                │
│ pub_date           │
│ created_at         │
│ is_read            │
│ is_starred         │
└────────────────────┘

┌────────────────┐
│   settings     │
├────────────────┤
│ key (PK)       │
│ value          │
│ created_at     │
│ updated_at     │
└────────────────┘
```

## Database Operations

### Insert Feed

```rust
pub fn insert_feed(
    db: &Connection,
    feed: &Feed,
) -> Result<(), rusqlite::Error> {
    db.execute(
        "INSERT INTO feeds (id, url, title, description, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![feed.id, feed.url, feed.title, feed.description, feed.created_at],
    )?;
    Ok(())
}
```

### Insert Articles (Bulk)

```rust
pub fn insert_articles(
    db: &Connection,
    articles: &[Article],
) -> Result<(), rusqlite::Error> {
    let tx = db.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT OR IGNORE INTO articles
             (id, feed_id, title, description, url, pub_date, created_at, is_read, is_starred)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
        )?;

        for article in articles {
            stmt.execute(params![
                article.id,
                article.feed_id,
                article.title,
                article.description,
                article.url,
                article.pub_date,
                article.created_at,
                article.is_read,
                article.is_starred,
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}
```

### Get All Feeds

```rust
pub fn get_all_feeds(db: &Connection) -> Result<Vec<Feed>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT id, url, title, description, last_updated, created_at
         FROM feeds
         ORDER BY created_at DESC"
    )?;

    let feeds = stmt.query_map([], |row| {
        Ok(Feed {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            last_updated: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    feeds.collect()
}
```

### Get Articles by Feed

```rust
pub fn get_articles_by_feed(
    db: &Connection,
    feed_id: &str,
) -> Result<Vec<Article>, rusqlite::Error> {
    let mut stmt = db.prepare(
        "SELECT id, feed_id, title, description, content, url, pub_date, created_at, is_read, is_starred
         FROM articles
         WHERE feed_id = ?1
         ORDER BY pub_date DESC, created_at DESC
         LIMIT 100"
    )?;

    let articles = stmt.query_map(params![feed_id], |row| {
        Ok(Article {
            id: row.get(0)?,
            feed_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            content: row.get(4)?,
            url: row.get(5)?,
            pub_date: row.get(6)?,
            created_at: row.get(7)?,
            is_read: row.get(8)?,
            is_starred: row.get(9)?,
        })
    })?;

    articles.collect()
}
```

### Mark Article as Read

```rust
pub fn mark_as_read(
    db: &Connection,
    article_id: &str,
) -> Result<(), rusqlite::Error> {
    db.execute(
        "UPDATE articles SET is_read = 1 WHERE id = ?1",
        params![article_id],
    )?;
    Ok(())
}
```

### Get Unread Count

```rust
pub fn get_unread_count(
    db: &Connection,
    feed_id: &str,
) -> Result<i32, rusqlite::Error> {
    db.query_row(
        "SELECT COUNT(*) FROM articles WHERE feed_id = ?1 AND is_read = 0",
        params![feed_id],
        |row| row.get(0),
    )
}
```

## Database Initialization

### Schema Creation

```rust
pub fn init_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let db = Connection::open(db_path)?;

    // Enable foreign keys
    db.execute("PRAGMA foreign_keys = ON", [])?;

    // Create schema
    db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS feeds (
            id TEXT PRIMARY KEY,
            url TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            last_updated INTEGER,
            created_at INTEGER NOT NULL,
            CHECK (url != ''),
            CHECK (length(title) > 0)
        );

        CREATE INDEX IF NOT EXISTS idx_feeds_created_at ON feeds(created_at);
        CREATE INDEX IF NOT EXISTS idx_feeds_updated_at ON feeds(last_updated);

        CREATE TABLE IF NOT EXISTS articles (
            id TEXT PRIMARY KEY,
            feed_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            content TEXT,
            url TEXT,
            pub_date INTEGER,
            created_at INTEGER NOT NULL,
            is_read BOOLEAN DEFAULT 0,
            is_starred BOOLEAN DEFAULT 0,
            FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE,
            CONSTRAINT unique_article UNIQUE (feed_id, url),
            CHECK (length(title) > 0)
        );

        CREATE INDEX IF NOT EXISTS idx_articles_feed_id ON articles(feed_id);
        CREATE INDEX IF NOT EXISTS idx_articles_is_read ON articles(is_read);
        CREATE INDEX IF NOT EXISTS idx_articles_is_starred ON articles(is_starred);
        CREATE INDEX IF NOT EXISTS idx_articles_pub_date ON articles(pub_date DESC);
        CREATE INDEX IF NOT EXISTS idx_articles_created_at ON articles(created_at DESC);

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);
        "
    )?;

    Ok(db)
}
```

## Migrations

### Migration Strategy

1. All migrations stored in `db/migrations/`
2. Migrations are numbered: `001_initial.sql`, `002_feature.sql`, etc.
3. Each migration has UP and DOWN components
4. Schema version tracked in code
5. Applied on startup if needed

### Phase 1 Migrations

**001_initial.sql** - Initial schema (described above)

### Future Migrations (Phase 2+)

**002_add_content_extraction.sql** - Add extracted_content column for Phase 2

```sql
-- UP
ALTER TABLE articles ADD COLUMN extracted_content TEXT;

-- DOWN
ALTER TABLE articles DROP COLUMN extracted_content;
```

## Performance Optimization

### Indices Strategy

**Feed queries:**
- `idx_feeds_created_at` - Sort newest feeds first
- `idx_feeds_updated_at` - Find stale feeds to refresh

**Article queries:**
- `idx_articles_feed_id` - Essential for filtering by feed
- `idx_articles_is_read` - For unread counts
- `idx_articles_is_starred` - For favorites
- `idx_articles_pub_date` - Sort by publication date
- `idx_articles_created_at` - Sort by fetch date

### Query Optimization Tips

1. Use `LIMIT` to paginate articles (not all at once)
2. Always filter by feed_id first
3. Use composite indices for multi-column filters
4. ANALYZE table periodically for query planner

## Backup & Recovery

### Backup Strategy (Phase 1)

1. Database file stored in app_data directory
2. User can export to JSON
3. No automatic cloud backup (Phase 2+)

### Restoration (Phase 1)

1. Manual restore from JSON export
2. Reimport feeds and articles

## Testing

### Database Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get_feed() {
        // Create in-memory database
        let db = Connection::open_in_memory().unwrap();
        init_db_schema(&db).unwrap();

        // Insert feed
        let feed = Feed { /* ... */ };
        insert_feed(&db, &feed).unwrap();

        // Get feed
        let feeds = get_all_feeds(&db).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].url, "https://example.com/feed");
    }
}
```

## Constraints & Validations

### URL Validation

- Must be valid HTTP/HTTPS URL
- Max length: 2048 characters
- No localhost for Phase 1

### Title Validation

- Not empty
- Max length: 500 characters

### Content Validation

- Article content max: 1MB
- No HTML injection (sanitize on display)

---

**Status**: Schema Complete
**Next**: API.md (Tauri command definitions)
**Validated by**: PHASE-1-FOUNDATION.md acceptance criteria
