# PHASE 1: Foundation

**Status**: Not Started
**Duration**: 2 weeks
**Reference**: [PHASE-1-FOUNDATION]

## Overview

Phase 1 establishes the core foundation for Unreal RSS. This includes:
- Setting up the Tauri project scaffold with Rust backend and React frontend
- Implementing basic feed fetching, parsing, and caching
- Designing and implementing SQLite database schema
- Building a minimal but functional feed list and article view UI
- Establishing all guardrails, pre-commit hooks, and CI/CD foundations

**Goal**: A working RSS reader that can add feeds, fetch articles, and display them. All tooling and guardrails in place for future development.

## Objectives

- [ ] Tauri project scaffolded with proper structure
- [ ] Feed fetching and RSS/Atom parsing working
- [ ] SQLite database with complete schema
- [ ] Feed list and article view UI components
- [ ] Pre-commit hooks and testing framework configured
- [ ] 80%+ test coverage for all new code
- [ ] Zero clippy warnings in Rust code
- [ ] All documentation up to date
- [ ] First working version ready for Phase 2

## Acceptance Criteria

### Tauri Project Setup
- [ ] Tauri 1.4+ project initializes correctly
- [ ] Rust backend compiles without warnings
- [ ] React + TypeScript frontend builds successfully
- [ ] Dev server runs with `npm run tauri dev`
- [ ] Production build succeeds with `npm run tauri build`
- [ ] Project structure matches docs/PROJECT-OVERVIEW.md

### Feed Fetching & Parsing
- [ ] Fetches RSS 2.0 feeds from HTTP/HTTPS URLs
- [ ] Parses Atom 1.0 feeds correctly
- [ ] Handles feed timeouts (>5s) gracefully
- [ ] Validates feed URLs (basic URL validation)
- [ ] Extracts title, description, URL, pub_date from feed items
- [ ] Caches feed content for offline access
- [ ] Returns error messages clearly (network error vs parse error)

### Database Schema
- [ ] SQLite database initializes on first run
- [ ] Tables created: feeds, articles, settings
- [ ] Feeds table includes: id, url, title, description, last_updated, created_at
- [ ] Articles table includes: id, feed_id, title, description, content, url, pub_date, created_at, is_read, is_starred
- [ ] All tables have proper indices
- [ ] Settings table stores user preferences (theme, font, etc.)
- [ ] Schema version tracked for future migrations

### UI - Feed Management
- [ ] Add feed button/form visible
- [ ] Can input feed URL and add feed
- [ ] Feeds display in a list (sidebar)
- [ ] Feed title shows with unread count
- [ ] Delete feed button with confirmation
- [ ] Visual feedback for loading state
- [ ] Error messages display clearly (invalid URL, network error, etc.)

### UI - Article View
- [ ] Clicking feed shows articles in main pane
- [ ] Articles list shows title, date, unread indicator
- [ ] Clicking article shows full content
- [ ] Article displays: title, source, date, content
- [ ] Can mark article as read/unread
- [ ] Can star/unstar articles
- [ ] Keyboard navigation works (arrow keys, Enter)

### Tauri IPC/API
- [ ] `add_feed(url: String) -> Result<Feed>`
- [ ] `get_feeds() -> Result<Vec<Feed>>`
- [ ] `delete_feed(feed_id: String) -> Result<()>`
- [ ] `get_articles(feed_id: String) -> Result<Vec<Article>>`
- [ ] `get_article(article_id: String) -> Result<Article>`
- [ ] `mark_as_read(article_id: String) -> Result<()`
- [ ] `mark_as_unread(article_id: String) -> Result<()>`
- [ ] `star_article(article_id: String) -> Result<()>`
- [ ] All commands validate input and return clear error messages

### Testing
- [ ] Feed parser tests (RSS, Atom, invalid feeds)
- [ ] Database tests (CRUD operations, schema)
- [ ] Tauri command tests (all API endpoints)
- [ ] React component tests (add feed form, article list, article view)
- [ ] Integration tests (add feed → fetch articles → view)
- [ ] >80% code coverage for new code
- [ ] All tests pass: `cargo test && npm run test`

### Code Quality
- [ ] `cargo fmt` passes (zero formatting issues)
- [ ] `cargo clippy` passes (zero warnings)
- [ ] `npm run lint` passes (zero ESLint errors)
- [ ] `npm run format` passes (Prettier formatting)
- [ ] `npm run type-check` passes (no TypeScript errors)
- [ ] All public functions documented with doc comments
- [ ] All commits reference [PHASE-1-FOUNDATION]

### Documentation
- [ ] README.md updated with setup instructions
- [ ] ARCHITECTURE.md documents module structure
- [ ] DATABASE.md documents schema with migration notes
- [ ] API.md documents all Tauri commands
- [ ] Code comments explain non-obvious logic
- [ ] Pre-commit hooks configured and documented

## Technical Requirements

### Architecture Changes
- Create `src-tauri/src/feed.rs` - Feed fetching and parsing logic
- Create `src-tauri/src/db.rs` - Database operations and schema
- Create `src-tauri/src/api.rs` - Tauri command definitions
- Create `src/components/FeedList.tsx` - Feed list sidebar
- Create `src/components/ArticleList.tsx` - Article list
- Create `src/components/ArticleView.tsx` - Article viewer
- Create `src/hooks/useFeed.ts` - Custom hook for feed operations
- Create `src/App.tsx` - Main layout and routing

### Database Schema

```sql
-- Feeds table
CREATE TABLE feeds (
    id TEXT PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    last_updated INTEGER,
    created_at INTEGER NOT NULL,
    CHECK (url != '')
);

-- Articles table
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
    CONSTRAINT unique_article UNIQUE (feed_id, url)
);

-- Settings table
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Indices for performance
CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_is_read ON articles(is_read);
CREATE INDEX idx_articles_is_starred ON articles(is_starred);
CREATE INDEX idx_feeds_created_at ON feeds(created_at);
```

### Dependencies

**Rust Crates**:
- [ ] `tauri` (1.4+) - Desktop framework
- [ ] `tokio` (latest) - Async runtime
- [ ] `reqwest` (latest) - HTTP client
- [ ] `rss` (latest) - RSS parsing
- [ ] `atom_syndication` (latest) - Atom parsing
- [ ] `serde` (latest) - Serialization
- [ ] `serde_json` (latest) - JSON serialization
- [ ] `rusqlite` (latest) - SQLite driver
- [ ] `uuid` (latest) - ID generation

**npm Packages**:
- [ ] `@tauri-apps/api` (latest) - Tauri frontend API
- [ ] `react` (18+) - UI framework
- [ ] `typescript` (latest) - Type safety
- [ ] `tailwindcss` (latest) - Styling
- [ ] `axios` or `@tauri-apps/api` - HTTP client

### Dependencies NOT Included Yet
- Content extraction (readability-rs) - Phase 2
- Full-text search - Phase 3
- Sync/backend - Phase 4

## Implementation Guide

### Step 1: Initialize Tauri Project
```bash
cd unreal-rss
cargo tauri init
# Select React + TypeScript
# Configure for Linux desktop
```

**Checklist**:
- [ ] Project structure created
- [ ] Cargo.toml configured
- [ ] package.json configured
- [ ] tauri.conf.json set up
- [ ] Dev environment runs without errors

### Step 2: Implement Feed Fetching
**File**: `src-tauri/src/feed.rs`

Create functions:
- `async fn fetch_feed(url: &str) -> Result<Feed, FeedError>` - Fetch and parse
- `fn parse_rss(content: &str) -> Result<Feed, FeedError>` - Parse RSS 2.0
- `fn parse_atom(content: &str) -> Result<Feed, FeedError>` - Parse Atom 1.0
- `struct Feed { id, url, title, description, articles }` - Data model
- `struct Article { id, title, description, url, pub_date }` - Data model

**Tests to write first**:
- `test_fetch_valid_rss_feed` - Happy path
- `test_fetch_invalid_url` - URL validation
- `test_fetch_timeout` - Network timeout
- `test_parse_rss_feed` - RSS parsing
- `test_parse_atom_feed` - Atom parsing
- `test_parse_invalid_feed` - Malformed feed

**Acceptance**:
- [ ] `cargo test feed::tests` passes
- [ ] `cargo clippy` on feed.rs shows zero warnings
- [ ] Doc comments on all public functions

### Step 3: Implement Database Layer
**File**: `src-tauri/src/db.rs`

Create functions:
- `fn init_db() -> Result<Connection, rusqlite::Error>` - Initialize/migrate DB
- `fn create_tables(db: &Connection) -> Result<(), rusqlite::Error>` - Create schema
- `fn add_feed(db: &Connection, feed: &Feed) -> Result<String, Error>` - Insert feed
- `fn get_feeds(db: &Connection) -> Result<Vec<Feed>, Error>` - Get all feeds
- `fn delete_feed(db: &Connection, feed_id: &str) -> Result<(), Error>` - Delete feed
- `fn add_articles(db: &Connection, articles: &[Article]) -> Result<(), Error>` - Bulk insert
- `fn get_articles(db: &Connection, feed_id: &str) -> Result<Vec<Article>, Error>` - Get articles
- `fn mark_as_read(db: &Connection, article_id: &str) -> Result<(), Error>` - Update read status
- `fn star_article(db: &Connection, article_id: &str) -> Result<(), Error>` - Toggle star

**Tests to write first**:
- `test_init_db` - DB creation
- `test_add_and_get_feed` - Insert/select feed
- `test_delete_feed` - Delete cascade
- `test_add_articles` - Bulk insert
- `test_mark_as_read` - Update status
- `test_database_constraints` - Foreign keys work

**Acceptance**:
- [ ] All CRUD operations tested
- [ ] Foreign key constraints enforced
- [ ] Indices present for common queries
- [ ] Database initializes cleanly on new runs

### Step 4: Create Tauri API Commands
**File**: `src-tauri/src/api.rs`

Tauri commands (these bridge Rust and React):
```rust
#[tauri::command]
async fn add_feed(url: String, state: State<'_, AppState>) -> Result<Feed, String>
#[tauri::command]
async fn get_feeds(state: State<'_, AppState>) -> Result<Vec<Feed>, String>
#[tauri::command]
async fn delete_feed(feed_id: String, state: State<'_, AppState>) -> Result<(), String>
#[tauri::command]
async fn get_articles(feed_id: String, state: State<'_, AppState>) -> Result<Vec<Article>, String>
#[tauri::command]
async fn get_article(article_id: String, state: State<'_, AppState>) -> Result<Article, String>
#[tauri::command]
async fn mark_as_read(article_id: String, state: State<'_, AppState>) -> Result<(), String>
#[tauri::command]
async fn mark_as_unread(article_id: String, state: State<'_, AppState>) -> Result<(), String>
#[tauri::command]
async fn star_article(article_id: String, state: State<'_, AppState>) -> Result<(), String>
```

**Tests to write first**:
- `test_add_feed_command` - Add feed via API
- `test_add_feed_invalid_url` - Validate URL
- `test_get_feeds_empty` - Empty feed list
- `test_get_articles` - Get articles for feed
- `test_mark_as_read` - Update via API

**Acceptance**:
- [ ] All commands defined
- [ ] Input validation at boundaries
- [ ] Error messages clear and consistent
- [ ] Commands return proper JSON types

### Step 5: Build React UI Components
**Files**:
- `src/components/FeedList.tsx` - Sidebar feed list
- `src/components/ArticleList.tsx` - Article list
- `src/components/ArticleView.tsx` - Article viewer
- `src/pages/App.tsx` - Main layout

**Components**:

**FeedList.tsx**:
- Display list of feeds with unread counts
- Add feed button/form
- Delete feed button
- Clicking feed shows articles

**ArticleList.tsx**:
- Display articles for selected feed
- Show title, date, read status
- Clicking article opens it
- Keyboard navigation

**ArticleView.tsx**:
- Display full article
- Show title, source, date, content
- Mark read/unread button
- Star button
- Navigation (prev/next article)

**App.tsx**:
- Main layout: sidebar (feeds) | main (articles) | detail (article view)
- Keyboard shortcuts
- Loading states

**Tests to write first**:
- `test_feed_list_renders` - Component renders
- `test_feed_list_add_feed` - Add feed form
- `test_article_list_displays_articles` - Article list
- `test_article_view_shows_content` - Article display
- `test_keyboard_navigation` - Arrow keys work

**Acceptance**:
- [ ] All components render without errors
- [ ] Basic functionality works (add, delete, view)
- [ ] Keyboard navigation functional
- [ ] Loading states display
- [ ] Error states display

### Step 6: Configure Pre-Commit Hooks & Testing
**File**: `.pre-commit-config.yaml`

Configure:
- Rust formatting: `cargo fmt --check`
- Rust linting: `cargo clippy -- -D warnings`
- Rust tests: `cargo test`
- TypeScript linting: ESLint
- TypeScript formatting: Prettier
- TypeScript type-check

**Also create**:
- `src-tauri/tests/integration_tests.rs` - Integration tests
- `src/__tests__/` - Component tests
- GitHub Actions CI/CD file

**Checklist**:
- [ ] Pre-commit hooks installed
- [ ] All hooks pass locally
- [ ] CI/CD pipeline works
- [ ] Test results visible in PR

## Validation Checklist

**BEFORE submitting PR, verify ALL of these:**

### Code Quality
- [ ] `cargo fmt --check` passes (no formatting issues)
- [ ] `cargo clippy -- -D warnings` passes (zero warnings)
- [ ] `npm run lint` passes (zero ESLint errors)
- [ ] `npm run format --check` passes (no formatting issues)
- [ ] `npm run type-check` passes (no TypeScript errors)
- [ ] No `unwrap()` calls in production code (except main.rs)
- [ ] All public functions have doc comments

### Testing
- [ ] `cargo test` - All Rust tests pass
- [ ] `npm run test` - All TypeScript tests pass
- [ ] Code coverage report shows >80% for new code
- [ ] No test files contain `#[ignore]` (skipped tests)
- [ ] Integration tests verify end-to-end workflows

### Functionality
- [ ] App starts: `npm run tauri dev` works
- [ ] Can add feed manually: URL input works
- [ ] Can fetch articles: Articles appear after adding feed
- [ ] Can view articles: Click article displays content
- [ ] Can mark as read: Read status persists
- [ ] Can star articles: Star status persists
- [ ] Error handling: Invalid URLs show error message
- [ ] Offline mode: Articles readable without network

### Documentation
- [ ] All public Rust functions documented with `///` comments
- [ ] All React components have JSDoc/TypeScript types
- [ ] Non-obvious logic has inline comments
- [ ] README.md updated with Phase 1 completion notes
- [ ] ARCHITECTURE.md describes module organization
- [ ] DATABASE.md documents schema
- [ ] API.md documents all Tauri commands

### Commits & PR
- [ ] All commits reference [PHASE-1-FOUNDATION] in message
- [ ] Commit messages are clear and descriptive
- [ ] PR title: "Phase 1: Foundation [PHASE-1-FOUNDATION]"
- [ ] PR description links to this spec
- [ ] No merge conflicts

### Security & Performance
- [ ] No SQL injection vulnerabilities (use parameterized queries)
- [ ] No sensitive data in logs
- [ ] App startup time <3 seconds (measured)
- [ ] Feed parsing <2 seconds for typical feeds
- [ ] Database queries use indices

## Definition of Done

**A feature is complete and ready to merge when ALL of these are true:**

1. ✅ **All acceptance criteria met** - Every criterion checked off
2. ✅ **All tests pass** - `cargo test && npm run test` succeeds
3. ✅ **All code quality checks pass** - fmt, clippy, lint, type-check
4. ✅ **Code coverage >80%** - New code is tested thoroughly
5. ✅ **Documentation complete** - Code comments, API docs updated
6. ✅ **Validation checklist verified** - All items above checked
7. ✅ **No regressions** - All existing tests still pass
8. ✅ **PR reviewed and approved** - Code review completed
9. ✅ **Commit messages reference spec** - All commits tagged [PHASE-1-FOUNDATION]
10. ✅ **Ready for Phase 2** - No blockers for next phase

**If any item is not complete, do not merge.**

## Copilot Guardrails

**When implementing Phase 1, Copilot MUST:**

- [ ] **Read spec first** - Understand all acceptance criteria before writing code
- [ ] **Write tests first** - Use TDD: tests before implementation
- [ ] **No unwrap()** - Use Result types and ? operator
- [ ] **Validate input** - All Tauri command arguments validated
- [ ] **Document public APIs** - Every public function has doc comment
- [ ] **Error messages** - Clear, helpful error context
- [ ] **Commit references spec** - Every commit: `[PHASE-1-FOUNDATION]`
- [ ] **No skipped tests** - No `#[ignore]` in final code
- [ ] **Parameterized queries** - Prevent SQL injection
- [ ] **Ask for clarification** - If anything is ambiguous

## Success Criteria

**Phase 1 is successful when:**
- ✅ Working RSS reader (add feed, view articles)
- ✅ All acceptance criteria met
- ✅ >80% test coverage
- ✅ Zero clippy warnings
- ✅ All guardrails in place
- ✅ Ready to move to Phase 2 (reader UX polish)

---

**Status**: Ready for Implementation
**Estimated Duration**: 2 weeks
**Difficulty**: High (learning Tauri, establishing patterns)
**Next Phase**: PHASE-2-READER.md
