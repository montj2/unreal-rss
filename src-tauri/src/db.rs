use chrono::Utc;
use rusqlite::{params, Connection, Result as SqlResult};
use std::path::Path;
use uuid::Uuid;

/// Initialize the database with schema
pub fn init(db_path: &Path) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Create feeds table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feeds (
            id TEXT PRIMARY KEY,
            url TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            last_updated INTEGER,
            created_at INTEGER NOT NULL,
            CHECK (url != '')
        );",
        [],
    )?;

    // Create articles table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (
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
            FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
        );",
        [],
    )?;

    // Create settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
        [],
    )?;

    // Create indices for common queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_feed_id ON articles(feed_id);",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_is_read ON articles(is_read);",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_articles_is_starred ON articles(is_starred);",
        [],
    )?;

    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feed {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub last_updated: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub id: String,
    pub feed_id: String,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub url: Option<String>,
    pub pub_date: Option<i64>,
    pub created_at: i64,
    pub is_read: bool,
    pub is_starred: bool,
}

/// Add a new feed to the database
pub fn add_feed(
    db_path: &Path,
    url: &str,
    title: &str,
    description: Option<&str>,
) -> SqlResult<Feed> {
    let conn = Connection::open(db_path)?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    conn.execute(
        "INSERT INTO feeds (id, url, title, description, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, url, title, description, now],
    )?;

    Ok(Feed {
        id,
        url: url.to_string(),
        title: title.to_string(),
        description: description.map(String::from),
        last_updated: None,
        created_at: now,
    })
}

/// Get all feeds
pub fn get_feeds(db_path: &Path) -> SqlResult<Vec<Feed>> {
    let conn = Connection::open(db_path)?;
    let mut stmt =
        conn.prepare("SELECT id, url, title, description, last_updated, created_at FROM feeds")?;

    let feeds = stmt
        .query_map([], |row| {
            Ok(Feed {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                last_updated: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(feeds)
}

/// Delete a feed
pub fn delete_feed(db_path: &Path, feed_id: &str) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM feeds WHERE id = ?1", params![feed_id])?;
    Ok(())
}

/// Get all articles for a feed
pub fn get_articles(db_path: &Path, feed_id: &str) -> SqlResult<Vec<Article>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "SELECT id, feed_id, title, description, content, url, pub_date, created_at, is_read, is_starred FROM articles WHERE feed_id = ?1 ORDER BY pub_date DESC"
    )?;

    let articles = stmt
        .query_map(params![feed_id], |row| {
            Ok(Article {
                id: row.get(0)?,
                feed_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                content: row.get(4)?,
                url: row.get(5)?,
                pub_date: row.get(6)?,
                created_at: row.get(7)?,
                is_read: row.get::<_, i32>(8)? != 0,
                is_starred: row.get::<_, i32>(9)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(articles)
}

/// Get a single article
pub fn get_article(db_path: &Path, article_id: &str) -> SqlResult<Option<Article>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "SELECT id, feed_id, title, description, content, url, pub_date, created_at, is_read, is_starred FROM articles WHERE id = ?1"
    )?;

    let article = stmt
        .query_row(params![article_id], |row| {
            Ok(Article {
                id: row.get(0)?,
                feed_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                content: row.get(4)?,
                url: row.get(5)?,
                pub_date: row.get(6)?,
                created_at: row.get(7)?,
                is_read: row.get::<_, i32>(8)? != 0,
                is_starred: row.get::<_, i32>(9)? != 0,
            })
        })
        .optional()?;

    Ok(article)
}

/// Mark article as read
pub fn mark_as_read(db_path: &Path, article_id: &str) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "UPDATE articles SET is_read = 1 WHERE id = ?1",
        params![article_id],
    )?;
    Ok(())
}

/// Mark article as unread
pub fn mark_as_unread(db_path: &Path, article_id: &str) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "UPDATE articles SET is_read = 0 WHERE id = ?1",
        params![article_id],
    )?;
    Ok(())
}

/// Star article
pub fn star_article(db_path: &Path, article_id: &str, starred: bool) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "UPDATE articles SET is_starred = ?1 WHERE id = ?2",
        params![if starred { 1 } else { 0 }, article_id],
    )?;
    Ok(())
}

/// Add article to database
pub fn add_article(db_path: &Path, article: &Article) -> SqlResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "INSERT OR IGNORE INTO articles (id, feed_id, title, description, content, url, pub_date, created_at, is_read, is_starred) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            article.id,
            article.feed_id,
            article.title,
            article.description,
            article.content,
            article.url,
            article.pub_date,
            article.created_at,
            if article.is_read { 1 } else { 0 },
            if article.is_starred { 1 } else { 0 },
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_tables() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let result = init(&db_path);
        assert!(result.is_ok());
        assert!(db_path.exists());
    }

    #[test]
    fn test_add_and_get_feeds() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        init(&db_path).unwrap();

        let feed = add_feed(
            &db_path,
            "https://example.com/feed",
            "Example Feed",
            Some("A test feed"),
        )
        .unwrap();
        assert_eq!(feed.title, "Example Feed");
        assert_eq!(feed.url, "https://example.com/feed");

        let feeds = get_feeds(&db_path).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Example Feed");
    }
}
