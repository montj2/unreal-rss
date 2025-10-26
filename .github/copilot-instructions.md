# Unreal RSS - Copilot Instructions

**Purpose**: These are hard guardrails for AI-assisted development. Follow these religiously.

**IMPORTANT**: At the start of every development session, use the **[Copilot Session Primer](../docs/COPILOT-SESSION-PRIMER.md)** to get yourself primed with project context and current phase information. This takes 2-3 minutes and prevents scope creep and off-the-rails development.

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
- All errors must use Result types and ? operator
- No `unwrap()` in production code
- Provide error context with `format!()` or custom error types
- Document errors in doc comments

### Documentation
- All public functions must have doc comments
- Include examples for non-trivial functions
- Document errors that can occur

### Testing
- Write unit tests in `#[cfg(test)]` module in same file
- Write integration tests in `src-tauri/tests/`
- Aim for 80%+ coverage of new code
- Test error cases, not just happy path
- No `#[ignore]` in production tests

## TypeScript/React Frontend Standards

### Code Style
- Use `npm run format` (Prettier enforced)
- ESLint must pass: `npm run lint`
- TypeScript strict mode enabled
- No `any` types (use `unknown` if truly unknown, then narrow)

### Component Structure
- Clear component interfaces with TypeScript types
- All props typed with TS interfaces
- No prop drilling (use context if needed)

### Error Handling
- Handle errors gracefully with try/catch
- Display error messages to users
- Log errors for debugging

### Testing
- Test for all hooks and utility functions
- Use React Testing Library for component tests
- Test user interactions, not implementation details

## Tauri IPC Standards

### Command Signatures
- All Tauri commands must have clear signatures
- Return types must be JSON-serializable
- Input validation at command boundary
- Clear error messages

### Command Pattern
```
#[tauri::command]
pub async fn command_name(arg1: Type1) -> Result<ReturnType, String> {
    // Validate inputs
    if arg1.is_empty() {
        return Err("arg1 cannot be empty".to_string());
    }
    // Implement logic
    // ...
}
```

## Database Standards

### Schema & Queries
- All database changes backwards compatible
- Use parameterized queries (prevent SQL injection)
- Document complex queries
- Add indices for frequently queried columns

### Pattern
```rust
// GOOD: Parameterized query
let result = db.query_row(
    "SELECT * FROM table WHERE id = ?1",
    params![id],
    |row| { /* ... */ }
)?;

// BAD: String concatenation
let result = db.query_row(
    &format!("SELECT * FROM table WHERE id = {}", id),
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
   // BAD - will panic
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
   // BAD - blocks entire runtime
   std::thread::sleep(Duration::from_secs(1));
   // GOOD - async sleep
   tokio::time::sleep(Duration::from_secs(1)).await;
   ```

4. **No input validation at API boundaries**
   ```rust
   // BAD - no validation
   #[tauri::command]
   pub fn delete_feed(feed_id: String) -> Result<(), String> {
       // ... assumes feed_id is valid
   }

   // GOOD - validates input
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
   // BAD - ignores errors
   let _ = db.execute("DELETE FROM articles WHERE feed_id = ?1", params![feed_id]);

   // GOOD - propagates errors
   db.execute("DELETE FROM articles WHERE feed_id = ?1", params![feed_id])?;
   ```

## When to Ask for Help

- Spec is ambiguous or contradictory
- Two design approaches seem equally valid
- New dependency needed (must be approved)
- Performance concern or technical issue
- Something doesn't fit stated architecture
- Unsure about implementation approach

**Always ask rather than guess.**

---

**Remember**: Spec-first + TDD + proper error handling = quality code

**Reference**: See `docs/PHASE-1-FOUNDATION.md` and `docs/COPILOT-INSTRUCTIONS.md` for detailed implementation guides.
