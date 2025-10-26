# Copilot Instructions for Unreal RSS

**Purpose**: These are hard guardrails for AI-assisted development. Follow these religiously.

## General Principles

1. **Spec-First Development** - Always read the spec FIRST. Never write code without understanding acceptance criteria.
2. **Test-Driven Development** - Write tests before implementation code.
3. **No Experiments** - All code must serve the spec. Refactor only what's in scope.
4. **Ask for Clarification** - If spec is ambiguous, ask the human before proceeding.

## Rust Backend Standards

### Code Style
- Use `cargo fmt` formatting (enforced by pre-commit)
- Run `cargo clippy` and fix all warnings before committing
- No `unwrap()` except in `main.rs` or tests
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values (not `None` magic values)
- No global mutable state

### Error Handling
```rust
// ✅ Good: Custom error types
#[derive(Debug)]
pub enum FeedError {
    ParseError(String),
    NetworkError(String),
    DatabaseError(String),
}

impl std::fmt::Display for FeedError { ... }

// ❌ Bad: Using unwrap
let feed = parse_feed(url).unwrap();

// ✅ Good: Propagate errors
let feed = parse_feed(url)?;
```

### Documentation
- All public functions must have doc comments
- Include examples for non-trivial functions
- Document errors that can occur

```rust
/// Fetches and parses an RSS feed from a URL
///
/// # Arguments
/// * `url` - The RSS feed URL
///
/// # Returns
/// Returns `Ok(Feed)` on success, `Err(FeedError)` on failure
///
/// # Errors
/// - `FeedError::NetworkError` if HTTP request fails
/// - `FeedError::ParseError` if feed is invalid
pub async fn fetch_feed(url: &str) -> Result<Feed, FeedError> {
    // ...
}
```

### Testing
- Write unit tests in the same file as code, in `#[cfg(test)]` module
- Write integration tests in `src-tauri/tests/`
- Aim for 80%+ coverage of new code
- Test error cases, not just happy path

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_feed() {
        let xml = r#"<?xml version="1.0"?><rss>...</rss>"#;
        let result = parse_feed(xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_feed() {
        let xml = "not xml";
        let result = parse_feed(xml);
        assert!(result.is_err());
    }
}
```

### Async Patterns
- Use `tokio` for async runtime (already configured)
- Don't block in async code
- Use `.await` for async operations
- Prefer `tokio::spawn` for concurrent tasks

## TypeScript/React Frontend Standards

### Code Style
- Use `npm run format` (Prettier enforced)
- ESLint must pass: `npm run lint`
- TypeScript strict mode enabled
- No `any` types (use `unknown` if truly unknown, then narrow)

### Component Structure
```tsx
// ✅ Good: Clear component with proper types
interface ArticleProps {
  id: string;
  title: string;
  onClose: () => void;
}

export const ArticleView: React.FC<ArticleProps> = ({ id, title, onClose }) => {
  return <div>{title}</div>;
};

// ❌ Bad: Using any
const ArticleView = ({ id, title, onClose }: any) => { ... }
```

### Error Handling
```tsx
// ✅ Good: Handle errors gracefully
const [error, setError] = useState<string | null>(null);

const loadArticle = async () => {
  try {
    const article = await invoke('get_article', { id });
    setArticle(article);
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Unknown error');
  }
};

// ❌ Bad: Ignoring errors
const loadArticle = async () => {
  const article = await invoke('get_article', { id });
  setArticle(article);
};
```

### Testing
- Write tests for all hooks and utility functions
- Use React Testing Library for component tests
- Test user interactions, not implementation details

## Tauri IPC Standards

### Command Signatures
- All Tauri commands must have clear, documented signatures
- Return types must be JSON-serializable
- Input validation at the command boundary

```rust
// ✅ Good: Clear command with validation
#[tauri::command]
pub fn get_article(article_id: String) -> Result<Article, String> {
    if article_id.is_empty() {
        return Err("article_id cannot be empty".to_string());
    }
    // ... fetch and return
}

// ❌ Bad: No validation
#[tauri::command]
pub fn get_article(article_id: String) -> Article {
    // ... this can panic
}
```

### Frontend Invocation
```tsx
// ✅ Good: Type-safe invocation with error handling
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke<Article>('get_article', { articleId: '123' })
  .catch(err => console.error('Failed to load:', err));

// ❌ Bad: No error handling
const article = await invoke('get_article', { articleId: '123' });
```

## Database Standards

### Schema Migrations
- All database changes must be backwards compatible
- Schema lives in spec/DATABASE.md
- Version the schema in code comments

### Queries
- Use parameterized queries to prevent SQL injection
- Document complex queries
- Add indexes for frequently queried columns

```rust
// ✅ Good: Parameterized query
let article = db.query_row(
    "SELECT id, title FROM articles WHERE id = ?1",
    params![article_id],
    |row| { /* ... */ }
)?;

// ❌ Bad: String concatenation
let article = db.query_row(
    &format!("SELECT id, title FROM articles WHERE id = {}", article_id),
    [],
    |row| { /* ... */ }
)?;
```

## Commit Message Standards

**Format**: `<type>(<scope>): <description> [SPEC-REF]`

- `type`: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
- `scope`: `backend`, `frontend`, `db`, `api`, `ui`
- `description`: Clear, imperative voice (not past tense)
- `SPEC-REF`: Reference the spec document, e.g., `[PHASE-1-FOUNDATION]`

**Examples**:
```
feat(backend): add feed fetching and parsing [PHASE-1-FOUNDATION]
fix(frontend): handle loading state in article view [PHASE-1-FOUNDATION]
test(backend): add tests for feed parser [PHASE-1-FOUNDATION]
docs: update README with architecture overview [PHASE-1-FOUNDATION]
```

## Pre-Commit Checklist

**Before EVERY commit, verify:**

```bash
# Rust backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# TypeScript frontend
npm run lint
npm run format --check
npm run type-check
npm run test

# Commit message references spec
git log -1 --pretty=%B | grep -E "\[PHASE-|SPEC-"
```

## Common Pitfalls to Avoid

### ❌ Anti-Patterns

1. **Using `unwrap()` in production code**
   ```rust
   // BAD
   let feed = parse_feed(xml).unwrap();
   ```

2. **No error context**
   ```rust
   // BAD
   return Err("error".to_string());
   // GOOD
   return Err(format!("Failed to parse feed '{}': {}", url, e));
   ```

3. **Blocking in async code**
   ```rust
   // BAD
   let data = std::thread::sleep(Duration::from_secs(1)); // ❌ BLOCKS
   // GOOD
   tokio::time::sleep(Duration::from_secs(1)).await; // ✅ ASYNC
   ```

4. **No input validation at API boundaries**
   ```rust
   // BAD
   #[tauri::command]
   pub fn delete_feed(feed_id: String) -> Result<(), String> {
       // ... assumes feed_id is valid
   }

   // GOOD
   #[tauri::command]
   pub fn delete_feed(feed_id: String) -> Result<(), String> {
       if feed_id.is_empty() {
           return Err("feed_id cannot be empty".to_string());
       }
       // ...
   }
   ```

5. **Silently failing**
   ```rust
   // BAD
   let _ = db.execute("DELETE FROM articles WHERE feed_id = ?1", params![feed_id]);

   // GOOD
   db.execute("DELETE FROM articles WHERE feed_id = ?1", params![feed_id])?;
   ```

## When to Ask for Help

- Spec is ambiguous or contradictory
- Two design approaches seem equally valid
- Something doesn't fit the stated architecture
- A dependency isn't in the approved list
- Performance might be affected

**Always ask rather than guess.**

---

**Last Updated**: October 2025
**Version**: 1.0
