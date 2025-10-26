# Development Guidelines

## Project Setup & Tooling

### Required Tools
- Rust 1.70+ (install via rustup)
- Node.js 18+ (install via nvm or package manager)
- SQLite 3 (usually pre-installed on Linux)
- Git (with pre-commit hooks enabled)

### Environment Setup
```bash
# Clone repository
git clone <repo-url>
cd unreal-rss

# Install Rust dependencies
cargo build

# Install Node dependencies
npm install

# Install pre-commit hooks
pre-commit install

# Verify setup
cargo test
npm run test
```

## Workflow

### Starting a New Phase or Spec

1. **Read the spec** - Completely understand acceptance criteria and validation checklist
2. **Create feature branch**: `git checkout -b phase-1-foundation`
3. **Reference in commits**: All commits must include spec reference in message
4. **Open PR early**: Let's discuss implementation approach before writing code

### During Implementation

1. **Write tests first** (TDD approach)
2. **Run pre-commit checks** frequently: `pre-commit run --all-files`
3. **Commit incrementally** - Small, focused commits with spec references
4. **No WIP merges** - Phase must be complete before merging to main

### Before Submitting PR

1. **Verify all checklist items** in the spec's Validation Checklist
2. **Run full test suite**: `cargo test && npm run test`
3. **Check coverage**: Ensure >80% for new code
4. **Review commit messages**: Do they reference the spec?
5. **Update docs**: Any architecture changes documented?

## Running Tests

### Rust Backend
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_parse_feed

# Run with output
cargo test -- --nocapture

# Generate coverage (requires coverage tool)
cargo tarpaulin
```

### TypeScript Frontend
```bash
# Run all tests
npm run test

# Run specific test file
npm run test -- ArticleView.test.tsx

# Watch mode
npm run test -- --watch

# Coverage
npm run test -- --coverage
```

### Integration Tests
```bash
# Run the app in dev mode
npm run tauri dev

# Manually test features against acceptance criteria
```

## Code Review Process

1. **Self-review first** - Check your own code against the checklist
2. **Run validation checks** - Ensure all validation checklist items pass
3. **Create PR with spec reference** in the title
4. **Link to spec doc** in PR description
5. **Address feedback** promptly

## Debugging

### Rust Backend
```bash
# Debug logs in code
println!("Debug: {:?}", variable);
// or use the log crate:
log::debug!("Debug: {:?}", variable);

# Run with debug output
RUST_LOG=debug cargo run
```

### Frontend
```bash
# Browser DevTools (Tauri app opens with DevTools in dev mode)
# Press F12 to open

# Console logs
console.log("Debug:", variable);
console.error("Error:", error);
```

### Tauri Logs
```bash
# View Tauri logs
cargo run | grep "tauri"
```

## Performance Profiling

### Rust
```bash
# Use cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin unreal-rss
# View: flamegraph.svg
```

### Frontend
- Use Chrome DevTools Performance tab
- Profile React rendering: React Profiler

## Dependency Management

### Rust
- **All new crates must be justified** in a PR comment
- List alternatives considered
- Check for security: `cargo audit`

### Node/npm
- **Same standard** - justify new packages
- Check for security: `npm audit`

### Approved Dependencies (will grow)
- Tauri core
- React
- TypeScript
- SQLite + rusqlite
- tokio
- serde
- reqwest
- readability-rs (for content extraction)

## Release Process

1. Merge all completed phases to `main`
2. Run full test suite: `cargo test && npm run test`
3. Tag release: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. Build release: `cargo tauri build`

---

**Need help?** Check:
1. SPEC-TEMPLATE.md - How to read/write specs
2. COPILOT-INSTRUCTIONS.md - Coding standards
3. Phase/Spec docs - Specific requirements
4. Project README.md - High-level overview
