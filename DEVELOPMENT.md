# Development Setup Guide

Complete guide to set up your local development environment for Unreal RSS.

**Time**: ~15 minutes | **Difficulty**: Beginner

---

## Prerequisites

Install these tools first:

### 1. Rust (Stable Toolchain)

```bash
# Install from https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Node.js (18+)

```bash
# Install from https://nodejs.org/ (LTS recommended)
# Or use a version manager (nvm, fnm, etc.)

# Verify installation
node --version    # Should be v18.x or higher
npm --version
```

### 3. Pre-commit Framework

```bash
# Install via pip (requires Python 3.6+)
pip install pre-commit

# Verify installation
pre-commit --version
```

### 4. Tauri CLI

```bash
# Install via npm
npm install -g @tauri-apps/cli

# Or install locally (recommended for project)
npm install --save-dev @tauri-apps/cli
```

### 5. System Dependencies (Linux only)

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

# Fedora
sudo dnf install gtk3-devel webkit2gtk-devel libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

---

## Project Setup

### 1. Clone Repository

```bash
cd /path/to/projects
git clone <repository-url>
cd unreal-rss
```

### 2. Install Dependencies

```bash
# Frontend dependencies
npm install

# Rust will be compiled on first build/test
```

### 3. Setup Pre-commit Hooks

```bash
# Install pre-commit hooks
pre-commit install

# Verify installation
pre-commit run --all-files

# Expected output: All hooks should pass (green ✓)
```

### 4. Create .env File (if needed)

```bash
# For now, no env vars needed for Phase 1
# Create empty file for future use
touch .env
```

### 5. Verify Setup

```bash
# Test Rust build
cargo build --manifest-path src-tauri/Cargo.toml

# Test TypeScript
npm run type-check

# Test pre-commit hooks
pre-commit run --all-files

# Expected: All pass without errors
```

---

## Daily Development Workflow

### Start a New Feature

```bash
# 1. Update main branch
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b feature/your-feature-name

# 3. Prime Copilot (if using AI assistance)
# Open VS Code and use /docs/COPILOT-SESSION-PRIMER.md
```

### Write Code

```bash
# Follow the current phase spec
# Example: docs/phases/PHASE-1-FOUNDATION.md

# Key principles:
# 1. Read the spec FIRST
# 2. Write tests BEFORE implementation (TDD)
# 3. Follow guardrails in .github/copilot-instructions.md
# 4. Commit frequently with spec references
```

### Run Pre-commit Checks (Manual)

```bash
# Rust formatting + linting + tests
cd src-tauri
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --lib

# TypeScript formatting + linting + tests
npm run format
npm run lint
npm run type-check
npm run test

# Or run all hooks at once
pre-commit run --all-files
```

### Commit Your Changes

```bash
# Make sure pre-commit hooks pass first!
# If not, they'll auto-fix or tell you what to fix

# Commit with spec reference
git commit -m "feat(backend): add RSS parser [PHASE-1-FOUNDATION]"
git commit -m "fix(frontend): handle empty feed list [PHASE-1-FOUNDATION]"
git commit -m "test(backend): add feed parser tests [PHASE-1-FOUNDATION]"

# Format: <type>(<scope>): <description> [PHASE-X]
# Types: feat, fix, test, docs, refactor, chore
# Scopes: backend, frontend, db, api, setup
```

### Push and Create PR

```bash
# Push to remote
git push -u origin feature/your-feature-name

# Create PR on GitHub
# Use .github/PULL_REQUEST_TEMPLATE.md as guide

# CI/CD will automatically run:
# - cargo fmt check
# - cargo clippy
# - cargo test
# - npm run lint
# - npm run format check
# - npm run type-check
# - npm run test
```

### Code Review

```bash
# Address review comments
git add .
git commit -m "review: address feedback [PHASE-1-FOUNDATION]"
git push

# Once approved, merge via GitHub UI
# Prefer "Squash and merge" to keep history clean
```

---

## Development Server

### Start the Tauri Dev Server

```bash
# Terminal 1: Start frontend with HMR
npm run dev

# Terminal 2 (after Tauri scaffold): Start Tauri dev
cd src-tauri
cargo tauri dev

# Expected: Window opens with hot reload enabled
```

### View Logs

```bash
# Rust backend logs (from tauri dev terminal)
# Errors will appear in the terminal

# Frontend logs (browser console)
# Open DevTools: Ctrl+Shift+I or Cmd+Option+I

# Or check dev terminal for React errors
```

---

## Testing

### Rust Tests

```bash
# Run all tests
cd src-tauri
cargo test

# Run specific test
cargo test feed_parser

# Run with output
cargo test -- --nocapture

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo test
```

### TypeScript/React Tests

```bash
# Run all tests
npm run test

# Run specific test file
npm run test -- article-list.test.tsx

# Run in watch mode (during development)
npm run test -- --watch

# Run with coverage
npm run test -- --coverage --watchAll=false
```

### Integration Tests

```bash
# Run end-to-end tests (after Tauri scaffold)
npm run test:e2e
```

---

## Debugging

### Rust Backend

```bash
# Print debug info in code
dbg!(variable);

# Or use println!
println!("Debug: {:?}", value);

# Run with backtrace
RUST_BACKTRACE=full cargo test

# Use rust-analyzer (in VS Code)
# Install rust-analyzer extension for IDE debugging
```

### TypeScript/React Frontend

```bash
# Browser DevTools
# Ctrl+Shift+I or F12

# Add console.log
console.log('Debug:', value);

# Use React DevTools extension (browser)

# Debugger statement
debugger;  // Execution stops here if DevTools open
```

### Database (SQLite)

```bash
# After creating database, inspect with CLI
sqlite3 ~/.config/unreal-rss/app.db

# Common commands
.tables              # List tables
.schema feeds        # Show table schema
SELECT COUNT(*) FROM articles;  # Count rows
.quit                # Exit
```

---

## Troubleshooting

### Pre-commit Hook Failing

```bash
# See which hook failed
pre-commit run --all-files

# Fix formatting issues
cargo fmt --all
npm run format

# Fix clippy warnings
cargo clippy --fix

# Try again
pre-commit run --all-files
```

### Rust Build Error

```bash
# Clean and rebuild
cd src-tauri
cargo clean
cargo build

# Check for toolchain issues
rustup update
rustup component add rust-analyzer
```

### Node Modules Issue

```bash
# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### Pre-commit Not Enforcing on Commit

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Verify
ls -la .git/hooks/ | grep pre-commit
```

### Missing Tauri Dependencies (Linux)

```bash
# On Ubuntu/Debian
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev \
  libappindicator3-dev librsvg2-dev patchelf
```

---

## CI/CD Pipeline

### GitHub Actions Automatic Checks

When you push a PR, `.github/workflows/ci.yml` automatically:

1. **Rust Checks** (~5 min)
   - `cargo fmt --check`
   - `cargo clippy -- -D warnings`
   - `cargo test --lib`

2. **TypeScript Checks** (~3 min)
   - `npm run format --check`
   - `npm run lint`
   - `npm run type-check`
   - `npm run test --coverage`

3. **Tauri Build** (~10 min)
   - `npm run tauri build`

4. **Documentation** (~1 min)
   - Verify key docs exist

All checks must pass before merge. If any fail, fix locally, commit, and push.

---

## Performance Tips

### Speed Up Builds

```bash
# Use sccache for incremental Rust builds
cargo install sccache
export RUSTC_WRAPPER=sccache

# Use lld for faster linking (Linux)
# Add to ~/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Speed Up Tests

```bash
# Run tests in parallel
cargo test -- --test-threads=4

# Skip doc tests
cargo test --lib
```

---

## IDE Setup

### VS Code (Recommended)

```bash
# Install extensions
- Rust-analyzer (rust-lang.rust-analyzer)
- ESLint (dbaeumer.vscode-eslint)
- Prettier (esbenp.prettier-vscode)
- Tauri (tauri-apps.tauri-vscode)

# In .vscode/settings.json:
{
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[typescript]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  }
}
```

### JetBrains IDEs (IntelliJ, CLion, etc.)

```bash
# Install plugins
- Rust (JetBrains built-in)
- ESLint (for TypeScript)
- Prettier (for formatting)

# Enable built-in code inspections
```

---

## Getting Help

**Before asking for help, check:**

1. Is the issue in your local setup? → Run `pre-commit run --all-files`
2. Is it in the code? → Check relevant spec in `docs/phases/`
3. Is it a known issue? → Check project GitHub issues
4. Do I understand the requirements? → Read the spec FIRST

**When asking for help:**

1. Describe what you're trying to do (reference spec)
2. Show error message/output
3. Provide steps to reproduce
4. Include your system info: `uname -a`, `rustc --version`, `npm --version`

---

## Quick Reference

```bash
# Development workflow
git checkout -b feature/my-feature
npm run dev                          # Start dev server
pre-commit run --all-files           # Check all code
git commit -m "feat(scope): desc [PHASE-1-FOUNDATION]"
git push -u origin feature/my-feature
# Create PR on GitHub

# Common commands
cargo fmt --all                      # Format Rust
cargo clippy --all-targets -- -D warnings  # Lint Rust
npm run format                       # Format TypeScript
npm run lint                         # Lint TypeScript
cargo test                           # Test Rust
npm run test                         # Test TypeScript
pre-commit run --all-files           # Run all hooks

# Debugging
RUST_BACKTRACE=1 cargo test         # Rust with backtrace
npm run test -- --watch             # TypeScript watch mode
DevTools: Ctrl+Shift+I              # Browser console
dbg!(var)                           # Rust debug print
console.log(var)                    # JS debug print
```

---

## Next Steps

1. ✅ Follow setup steps above (15 minutes)
2. ✅ Verify setup: `pre-commit run --all-files`
3. ✅ Create feature branch: `git checkout -b feature/my-first-feature`
4. ✅ Read current phase spec: `docs/phases/PHASE-1-INFRASTRUCTURE.md`
5. ✅ Prime Copilot: Use `docs/COPILOT-SESSION-PRIMER.md`
6. ✅ Write code following TDD
7. ✅ Commit with spec reference: `[PHASE-1-INFRASTRUCTURE]`
8. ✅ Push and create PR

**You're ready to develop!**

---

**Questions?** Check the docs:
- Project overview: `docs/PROJECT-OVERVIEW.md`
- Current phase: `docs/phases/PHASE-1-FOUNDATION.md` (after infrastructure)
- Copilot guardrails: `.github/copilot-instructions.md`
- Copilot session primer: `docs/COPILOT-SESSION-PRIMER.md`
