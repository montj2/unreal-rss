use crate::db;
use crate::feed;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
}

fn get_db_path() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    data_dir.join("unreal-rss").join("unreal-rss.db")
}

/// Add a new feed
#[tauri::command]
pub async fn add_feed(url: String) -> Result<db::Feed, String> {
    // Validate URL
    feed::validate_url(&url).map_err(|e| e.to_string())?;

    // Fetch and parse feed
    let articles = feed::fetch_feed(&url).await.map_err(|e| e.to_string())?;

    if articles.is_empty() {
        return Err("Feed contains no articles".to_string());
    }

    // Extract feed title and description from first article (placeholder)
    let title = articles
        .first()
        .map(|a| a.title.clone())
        .unwrap_or_else(|| "Untitled Feed".to_string());

    // Add feed to database
    let db_path = get_db_path();
    let feed = db::add_feed(&db_path, &url, &title, None).map_err(|e| e.to_string())?;

    // Add articles to database
    for article in articles {
        db::add_article(&db_path, &article).map_err(|e| e.to_string())?;
    }

    Ok(feed)
}

/// Get all feeds
#[tauri::command]
pub fn get_feeds() -> Result<Vec<db::Feed>, String> {
    let db_path = get_db_path();
    db::get_feeds(&db_path).map_err(|e| e.to_string())
}

/// Delete a feed
#[tauri::command]
pub fn delete_feed(feed_id: String) -> Result<(), String> {
    if feed_id.is_empty() {
        return Err("feed_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::delete_feed(&db_path, &feed_id).map_err(|e| e.to_string())
}

/// Get articles for a feed
#[tauri::command]
pub fn get_articles(feed_id: String) -> Result<Vec<db::Article>, String> {
    if feed_id.is_empty() {
        return Err("feed_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::get_articles(&db_path, &feed_id).map_err(|e| e.to_string())
}

/// Get a single article
#[tauri::command]
pub fn get_article(article_id: String) -> Result<Option<db::Article>, String> {
    if article_id.is_empty() {
        return Err("article_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::get_article(&db_path, &article_id).map_err(|e| e.to_string())
}

/// Mark article as read
#[tauri::command]
pub fn mark_as_read(article_id: String) -> Result<(), String> {
    if article_id.is_empty() {
        return Err("article_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::mark_as_read(&db_path, &article_id).map_err(|e| e.to_string())
}

/// Mark article as unread
#[tauri::command]
pub fn mark_as_unread(article_id: String) -> Result<(), String> {
    if article_id.is_empty() {
        return Err("article_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::mark_as_unread(&db_path, &article_id).map_err(|e| e.to_string())
}

/// Star article
#[tauri::command]
pub fn star_article(article_id: String, starred: bool) -> Result<(), String> {
    if article_id.is_empty() {
        return Err("article_id cannot be empty".to_string());
    }

    let db_path = get_db_path();
    db::star_article(&db_path, &article_id, starred).map_err(|e| e.to_string())
}
