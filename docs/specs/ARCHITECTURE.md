# PHASE 1: Architecture Specification

**Status**: Part of Phase 1 - Foundation
**Reference**: [PHASE-1-ARCHITECTURE]

## Overview

This document defines the system architecture for Unreal RSS Phase 1. It establishes the module structure, data flow, and design patterns for the complete MVP.

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────┐         ┌──────────────────────┐   │
│  │   React Frontend     │         │   Rust Backend       │   │
│  │  (TypeScript)        │◄────►   │   (Tokio Async)      │   │
│  │                      │  IPC    │                      │   │
│  │ - UI Components      │         │ - Feed Fetching      │   │
│  │ - State Management   │         │ - Feed Parsing       │   │
│  │ - User Events        │         │ - DB Operations      │   │
│  │                      │         │ - Tauri Commands     │   │
│  └──────────────────────┘         └────────┬─────────────┘   │
│                                             │                 │
│                                   ┌─────────▼──────────┐      │
│                                   │  SQLite Database   │      │
│                                   │                    │      │
│                                   │ - Feeds Table      │      │
│                                   │ - Articles Table   │      │
│                                   │ - Settings Table   │      │
│                                   └────────────────────┘      │
│                                                               │
│  Network Layer (HTTP requests)                               │
│  ├─ Fetch RSS/Atom feeds                                     │
│  ├─ Handle redirects and errors                              │
│  └─ Respect rate limiting                                    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Module Structure

#### Backend (Rust)

```
src-tauri/src/
├── main.rs              # Entry point, Tauri setup, command registration
├── lib.rs               # Public API exports
├── feed/
│   ├── mod.rs           # Feed module
│   ├── fetcher.rs       # HTTP fetching (reqwest)
│   ├── parser.rs        # RSS/Atom parsing
│   └── models.rs        # Feed, Article, FeedError types
├── db/
│   ├── mod.rs           # Database module
│   ├── schema.rs        # Schema creation/migration
│   ├── queries.rs       # SQL queries (generic)
│   ├── feeds.rs         # Feed CRUD operations
│   ├── articles.rs      # Article CRUD operations
│   └── models.rs        # DB types and structs
├── api/
│   ├── mod.rs           # API module (all Tauri commands)
│   ├── feeds.rs         # Feed commands (add, delete, list)
│   ├── articles.rs      # Article commands (read, star)
│   └── utils.rs         # Common API utilities
├── error/
│   └── mod.rs           # Error types and handling
└── config/
    └── mod.rs           # App configuration and constants
```

#### Frontend (React/TypeScript)

```
src/
├── main.tsx             # Entry point
├── App.tsx              # Root component
├── components/
│   ├── FeedList.tsx     # Sidebar feed list
│   ├── ArticleList.tsx  # Article grid/list
│   ├── ArticleView.tsx  # Article reader
│   ├── AddFeedForm.tsx  # Add feed dialog
│   └── Loading.tsx      # Loading states
├── hooks/
│   ├── useFeed.ts       # Feed fetching/management
│   ├── useArticles.ts   # Article fetching/management
│   └── useSync.ts       # Data sync hooks
├── types/
│   └── index.ts         # TypeScript interfaces
├── utils/
│   ├── api.ts           # Tauri IPC calls
│   ├── errors.ts        # Error handling
│   └── formatting.ts    # Text formatting
├── styles/
│   └── globals.css      # Global styles + Tailwind
└── __tests__/
    ├── FeedList.test.tsx
    ├── ArticleView.test.tsx
    └── hooks.test.ts
```

## Data Models

### Feed Model

```rust
pub struct Feed {
    pub id: String,              // UUID
    pub url: String,             // RSS feed URL
    pub title: String,           // Feed title
    pub description: Option<String>,
    pub last_updated: Option<i64>, // Unix timestamp
    pub created_at: i64,         // Unix timestamp
}
```

### Article Model

```rust
pub struct Article {
    pub id: String,              // UUID
    pub feed_id: String,         // Foreign key to Feed
    pub title: String,
    pub description: Option<String>, // Summary
    pub content: Option<String>, // Full HTML content
    pub url: String,             // Article link
    pub pub_date: Option<i64>,   // Unix timestamp
    pub created_at: i64,
    pub is_read: bool,
    pub is_starred: bool,
}
```

### Settings Model

```rust
pub struct Settings {
    pub key: String,    // Setting key (e.g., "theme")
    pub value: String,  // Setting value (JSON serialized)
}

// Examples:
// { "key": "theme", "value": "dark" }
// { "key": "refresh_interval", "value": "300" }
// { "key": "font_size", "value": "16" }
```

## Data Flow

### Add Feed Flow

```
User Input (FeedList component)
    ↓
invoke('add_feed', { url })  [Tauri IPC]
    ↓
Backend: add_feed command (api/feeds.rs)
    ├─ Validate URL
    ├─ Fetch feed (feed/fetcher.rs)
    ├─ Parse feed (feed/parser.rs)
    ├─ Generate feed ID (UUID)
    ├─ Insert to DB (db/feeds.rs)
    ├─ Fetch articles for feed
    ├─ Insert articles to DB (db/articles.rs)
    └─ Return Feed + Articles
    ↓
Frontend: Update state, show articles
    ↓
User sees new feed in list
```

### Fetch Articles Flow

```
invoke('get_articles', { feed_id })  [Tauri IPC]
    ↓
Backend: get_articles command
    ├─ Validate feed_id
    ├─ Query articles from DB (db/articles.rs)
    ├─ Order by pub_date DESC
    └─ Return Vec<Article>
    ↓
Frontend: Display in ArticleList
```

### Mark as Read Flow

```
User clicks article
    ↓
invoke('mark_as_read', { article_id })  [Tauri IPC]
    ↓
Backend: mark_as_read command
    ├─ Validate article_id
    ├─ Update article in DB (is_read = true)
    ├─ Return success
    └─ (Re-render unread counts)
    ↓
Frontend: Update UI (visual feedback)
```

## Design Patterns

### Error Handling

All fallible operations use `Result<T, E>`:

```rust
// Good
pub fn fetch_feed(url: &str) -> Result<Feed, FeedError> {
    // ...
}

// Never use unwrap() in production code
```

### Database Connections

Shared DB connection via Tauri state:

```rust
pub struct AppState {
    db: Arc<Mutex<Connection>>,
}

// Acquired in commands:
let db = state.db.lock().unwrap();
```

### Async Operations

All I/O operations are async:

```rust
#[tauri::command]
pub async fn add_feed(
    url: String,
    state: State<'_, AppState>,
) -> Result<Feed, String> {
    // Use .await for async operations
}
```

### Frontend State Management

Use React hooks for local state, not global (unless needed):

```tsx
// Per-component state
const [selectedFeed, setSelectedFeed] = useState<Feed | null>(null);
const [articles, setArticles] = useState<Article[]>([]);
const [loading, setLoading] = useState(false);
```

## API Boundaries (Tauri Commands)

All communication between frontend and backend goes through Tauri commands. See API.md for full specification.

**Command patterns:**
- All commands are `async`
- All inputs validated at command boundary
- All outputs are JSON-serializable
- All errors return clear messages

## Testing Strategy

### Unit Tests (Backend)

Located in `src-tauri/src/` with `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rss_feed() {
        // ...
    }
}
```

### Integration Tests (Backend)

Located in `src-tauri/tests/`:

```rust
// tests/feed_integration.rs
#[tokio::test]
async fn test_add_feed_workflow() {
    // Full end-to-end test
}
```

### Component Tests (Frontend)

Located in `src/__tests__/`:

```tsx
// FeedList.test.tsx
describe('FeedList', () => {
    it('renders feeds', () => {
        // Component test
    });
});
```

## Performance Considerations

### Targets (Phase 1)

- App startup: <2 seconds
- Feed fetch: <10 seconds for typical feed
- Database query: <100ms
- UI render: <500ms

### Optimization strategies

1. **Async operations** - Never block UI thread
2. **Database indices** - Index frequently queried columns
3. **Caching** - Cache feeds between sessions
4. **Lazy loading** - Load articles on demand, not all at once
5. **Pagination** - Limit articles per page

## Security Considerations

### Phase 1 (MVP)

- [ ] Validate all URLs (no code injection)
- [ ] Sanitize feed titles/content (prevent XSS)
- [ ] Use parameterized SQL queries (prevent SQL injection)
- [ ] No sensitive data in logs
- [ ] No hardcoded secrets

### Phase 2+ (Backend sync)

- [ ] Authentication/authorization
- [ ] HTTPS/TLS for all traffic
- [ ] User data encryption
- [ ] API rate limiting

## Configuration & Constants

### Database

```rust
const DB_PATH: &str = "./app_data/feeds.db";
const SCHEMA_VERSION: i32 = 1;
```

### Networking

```rust
const REQUEST_TIMEOUT_SECS: u64 = 10;
const MAX_FEED_SIZE_BYTES: usize = 10_000_000; // 10MB
const MAX_ARTICLE_CONTENT_BYTES: usize = 1_000_000; // 1MB
```

### UI

```typescript
const ITEMS_PER_PAGE = 50;
const REFRESH_INTERVAL_MS = 5 * 60 * 1000; // 5 minutes
```

## Dependency Management

### Approved Rust Crates (Phase 1)

```toml
tauri = "1.4"
tokio = "1"
reqwest = "0.11"
rss = "0.12"
atom_syndication = "0.12"
serde = "1.0"
serde_json = "1.0"
rusqlite = "0.30"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = "0.4"
log = "0.4"
```

### Approved npm Packages (Phase 1)

```json
{
  "react": "^18.0",
  "react-dom": "^18.0",
  "typescript": "^5.0",
  "@tauri-apps/api": "^1.4",
  "tailwindcss": "^3.0",
  "eslint": "^8.0",
  "prettier": "^3.0"
}
```

## No Breaking Changes

Phase 1 architecture is designed to support:
- [ ] Phase 2 content extraction
- [ ] Phase 3 search indexing
- [ ] Phase 4 backend API

All architectural decisions are backwards-compatible and extensible.

---

**Status**: Specification Ready
**Next**: DATABASE.md (schema design)
