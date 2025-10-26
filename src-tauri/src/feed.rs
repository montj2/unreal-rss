use crate::db::Article;
use chrono::Utc;
use rss::Channel as RssChannel;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum FeedError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Timeout: feed took too long to fetch")]
    Timeout,
}

/// Validate feed URL
pub fn validate_url(url: &str) -> Result<(), FeedError> {
    url::Url::parse(url).map_err(|e| FeedError::InvalidUrl(e.to_string()))?;
    Ok(())
}

/// Fetch and parse a feed
pub async fn fetch_feed(url: &str) -> Result<Vec<Article>, FeedError> {
    validate_url(url)?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| FeedError::NetworkError(e.to_string()))?;

    let response = client.get(url).send().await.map_err(|e| {
        if e.is_timeout() {
            FeedError::Timeout
        } else {
            FeedError::NetworkError(e.to_string())
        }
    })?;

    let content = response
        .text()
        .await
        .map_err(|e| FeedError::NetworkError(e.to_string()))?;

    parse_feed(&content, url)
}

/// Parse RSS/Atom feed content
pub fn parse_feed(content: &str, feed_url: &str) -> Result<Vec<Article>, FeedError> {
    // Try parsing as RSS 2.0
    if let Ok(channel) = RssChannel::read_from(content.as_bytes()) {
        return parse_rss_channel(&channel, feed_url);
    }

    // Try parsing as Atom
    if let Ok(feed) = atom_syndication::Feed::read_from(content.as_bytes()) {
        return parse_atom_feed(&feed, feed_url);
    }

    Err(FeedError::ParseError(
        "Could not parse as RSS 2.0 or Atom 1.0".to_string(),
    ))
}

/// Parse RSS 2.0 channel
fn parse_rss_channel(channel: &RssChannel, feed_url: &str) -> Result<Vec<Article>, FeedError> {
    let mut articles = Vec::new();
    let now = Utc::now().timestamp();

    for item in &channel.items {
        let title = item.title.clone().unwrap_or_default();
        let description = item.description.clone();
        let content = None; // RSS doesn't have content field
        let url = item.link.clone();

        let pub_date = item.pub_date.as_ref().and_then(|d| {
            chrono::DateTime::parse_from_rfc2822(d)
                .ok()
                .map(|dt| dt.timestamp())
        });

        articles.push(Article {
            id: Uuid::new_v4().to_string(),
            feed_id: hash_url(feed_url),
            title,
            description,
            content,
            url,
            pub_date,
            created_at: now,
            is_read: false,
            is_starred: false,
        });
    }

    Ok(articles)
}

/// Parse Atom feed
fn parse_atom_feed(
    feed: &atom_syndication::Feed,
    feed_url: &str,
) -> Result<Vec<Article>, FeedError> {
    let mut articles = Vec::new();
    let now = Utc::now().timestamp();

    for entry in &feed.entries {
        let title = entry.title.to_string();
        let description = entry.summary.as_ref().map(|s| s.to_string());
        let content = entry.content.as_ref().and_then(|c| c.value.clone());
        let url = entry.links.first().map(|l| l.href.clone());

        let pub_date = entry
            .published
            .map(|d| d.timestamp())
            .or_else(|| entry.updated.map(|d| d.timestamp()));

        articles.push(Article {
            id: Uuid::new_v4().to_string(),
            feed_id: hash_url(feed_url),
            title,
            description,
            content,
            url,
            pub_date,
            created_at: now,
            is_read: false,
            is_starred: false,
        });
    }

    Ok(articles)
}

/// Simple hash function to generate consistent feed IDs from URLs
fn hash_url(url: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    format!("feed_{}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_url() {
        assert!(validate_url("https://example.com/feed").is_ok());
        assert!(validate_url("http://example.com/feed").is_ok());
    }

    #[test]
    fn test_validate_invalid_url() {
        assert!(validate_url("not a url").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_parse_invalid_feed() {
        let result = parse_feed("<invalid>", "https://example.com/feed");
        assert!(result.is_err());
    }
}
