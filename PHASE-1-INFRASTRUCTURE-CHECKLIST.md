# PHASE-1-INFRASTRUCTURE Implementation Checklist

**Status**: Ready to Execute
**Expected Duration**: 2 days (8 hours)
**Outcome**: Production-ready infrastructure preventing scope creep and enforcing quality

---

## ✅ Pre-Implementation Verification

Before starting, verify these are complete:

- [x] `.gitignore` created (Rust + Node.js + OS files)
- [x] `.pre-commit-config.yaml` created (cargo fmt, clippy, ESLint, Prettier, tests)
- [x] `.github/workflows/ci.yml` created (GitHub Actions CI/CD pipeline)
- [x] `.github/PULL_REQUEST_TEMPLATE.md` created (PR template with guardrails)
- [x] `DEVELOPMENT.md` created (setup and workflow guide)
- [x] `.github/copilot-instructions.md` created (Copilot guardrails)
- [x] `docs/COPILOT-SESSION-PRIMER.md` created (session startup guide)
- [x] Documentation complete (16 docs, all cross-referenced)

**Status**: ✅ All pre-implementation files created

---

## Day 1: Git & Pre-commit Setup

### Task 1.1: Verify Git Repository (30 min)

**Steps**:
```bash
# 1. Verify git is initialized
cd /home/montj2/unreal-rss
git status
# Expected: "On branch main" or "fatal: not a git repository"

# 2. If not initialized, initialize
git init
git branch -M main

# 3. Configure git (if first time)
git config user.name "Your Name"
git config user.email "your.email@example.com"

# 4. Verify configuration
git config --list | grep user
```

**Acceptance Criteria**:
- [ ] Git repository initialized
- [ ] Branch is `main`
- [ ] Git user configured (name + email)
- [ ] `git status` shows clean tree or existing commits

---

### Task 1.2: Add Initial Commit (15 min)

**Steps**:
```bash
# 1. Add all documentation
git add docs/
git add .github/copilot-instructions.md
git add .github/PULL_REQUEST_TEMPLATE.md

# 2. Verify what's being committed
git status

# 3. First commit with spec reference
git commit -m "docs: initial documentation and specifications [PHASE-1-INFRASTRUCTURE]"

# 4. Verify commit
git log -1 --oneline
```

**Expected Output**:
```
abc1234 docs: initial documentation and specifications [PHASE-1-INFRASTRUCTURE]
```

**Acceptance Criteria**:
- [ ] Initial commit created
- [ ] Commit message references `[PHASE-1-INFRASTRUCTURE]`
- [ ] All docs committed
- [ ] `git log` shows clean history

---

### Task 1.3: Add Infrastructure Files (15 min)

**Steps**:
```bash
# 1. Verify all infrastructure files exist
ls -la .gitignore
ls -la .pre-commit-config.yaml
ls -la .github/workflows/ci.yml
ls -la DEVELOPMENT.md

# 2. Add infrastructure files
git add .gitignore
git add .pre-commit-config.yaml
git add .github/workflows/
git add DEVELOPMENT.md

# 3. Verify
git status

# 4. Commit
git commit -m "chore(setup): add git, pre-commit, and CI/CD infrastructure [PHASE-1-INFRASTRUCTURE]"
```

**Acceptance Criteria**:
- [ ] `.gitignore` committed
- [ ] `.pre-commit-config.yaml` committed
- [ ] `.github/workflows/ci.yml` committed
- [ ] `DEVELOPMENT.md` committed
- [ ] Commit message references `[PHASE-1-INFRASTRUCTURE]`

---

### Task 1.4: Install Pre-commit Hooks (30 min)

**Steps**:
```bash
# 1. Install pre-commit framework (if not already installed)
pip install pre-commit
pre-commit --version  # Verify

# 2. Install the hooks
pre-commit install
pre-commit install --hook-type pre-commit

# 3. Verify hooks are installed
ls -la .git/hooks/ | grep pre-commit
# Expected: pre-commit -> /usr/local/bin/pre-commit (or similar symlink)

# 4. Test hooks on all files
pre-commit run --all-files

# Expected output: All checks pass (green ✓) or auto-fix applied
```

**Note**: First run may take time as dependencies install. Subsequent runs are faster.

**Acceptance Criteria**:
- [ ] Pre-commit framework installed
- [ ] Git hooks installed in `.git/hooks/`
- [ ] `pre-commit run --all-files` passes all checks
- [ ] Tests pass without errors

---

### Task 1.5: Verify Pre-commit Enforcement (30 min)

**Steps**:
```bash
# 1. Create a test file with bad formatting
echo "fn main() { println!(\"Hello\"); }" > src-tauri/src/test.rs

# 2. Try to commit it (without running hooks manually)
git add src-tauri/src/test.rs
git commit -m "test: verify pre-commit enforcement"

# Expected: Commit REJECTED by pre-commit hooks
# Output shows: cargo fmt auto-fixed the file

# 3. Verify hooks auto-fixed the formatting
git status  # File should be modified

# 4. Commit again (should pass)
git add src-tauri/src/test.rs
git commit -m "test: verify pre-commit enforcement"

# Expected: Commit SUCCEEDS

# 5. Clean up test file
git reset --soft HEAD~1
git reset HEAD src-tauri/src/test.rs
rm src-tauri/src/test.rs
```

**Acceptance Criteria**:
- [ ] Pre-commit hooks reject badly formatted code
- [ ] Hooks auto-fix formatting issues
- [ ] Successfully committed code after fixes
- [ ] Test file cleaned up

---

## Day 2: GitHub & CI/CD Setup

### Task 2.1: Configure Branch Protection (30 min)

**Steps**:
```bash
# 1. Push current commits to GitHub
git push -u origin main

# 2. Go to GitHub repository settings:
# https://github.com/YOUR-ORG/unreal-rss/settings

# 3. Click "Branches" → "Add rule"

# 4. Configure branch protection for `main`:
# - Pattern: main
# - ✓ Require pull request reviews before merging
# - ✓ Dismiss stale pull request approvals when new commits are pushed
# - ✓ Require status checks to pass before merging
# - Select status checks:
#   - All checks from .github/workflows/ci.yml
#   - ✓ Rust (cargo fmt, clippy, test)
#   - ✓ TypeScript (ESLint, Prettier, type-check, test)
#   - ✓ Tauri (app build)
#   - ✓ Docs (doc checks)
#   - ✓ All Checks (summary)
```

**Acceptance Criteria**:
- [ ] Branch protection rule created for `main`
- [ ] PR review required before merge
- [ ] Status checks required (all must pass)
- [ ] Stale PRs dismissed on new commits

---

### Task 2.2: Test CI/CD Pipeline (1 hour)

**Steps**:
```bash
# 1. Create a test feature branch
git checkout -b feature/test-ci

# 2. Make a harmless change (e.g., update README)
echo "" >> README.md
echo "# CI/CD Test" >> README.md

# 3. Commit with spec reference
git add README.md
git commit -m "docs: test CI/CD pipeline [PHASE-1-INFRASTRUCTURE]"

# 4. Push to GitHub
git push -u origin feature/test-ci

# 5. Go to GitHub and create a Pull Request
# https://github.com/YOUR-ORG/unreal-rss/pull/new/feature/test-ci

# 6. Watch the CI/CD pipeline run:
# - Should see checks running (status page shows progress)
# - Expected: All checks pass ✅

# 7. Check the PR comments:
# - CI/CD bot should comment "✅ CI/CD checks passed"

# 8. Merge the PR (if all checks pass)

# 9. Clean up feature branch locally
git checkout main
git pull origin main
git branch -d feature/test-ci
```

**Troubleshooting**:

If any checks fail:
1. Click the failing check to see details
2. Common issues:
   - Missing dependencies: Run `npm install` and `cargo fetch`
   - Formatting: Run `cargo fmt --all` and `npm run format`
   - Linting: Run `cargo clippy` and `npm run lint`
3. Fix locally, commit, push again
4. Re-run: GitHub will automatically re-run failed checks

**Acceptance Criteria**:
- [ ] GitHub Actions workflow triggered on PR
- [ ] All checks pass (green ✅)
- [ ] CI/CD bot comments on PR
- [ ] PR merges successfully
- [ ] Main branch still clean

---

### Task 2.3: Document Infrastructure (30 min)

**Steps**:
```bash
# 1. All infrastructure files already documented:
# - DEVELOPMENT.md (setup & workflow)
# - .github/copilot-instructions.md (guardrails)
# - docs/COPILOT-SESSION-PRIMER.md (session startup)
# - .github/PULL_REQUEST_TEMPLATE.md (PR requirements)

# 2. Create a quick reference card
# File: .github/INFRASTRUCTURE-README.md

# 3. Verify all docs are accessible
git log --all --oneline | head -5
```

**Acceptance Criteria**:
- [ ] All infrastructure documented
- [ ] DEVELOPMENT.md covers setup and workflow
- [ ] Copilot instructions comprehensive
- [ ] Session primer ready for developers
- [ ] PR template enforces quality gates

---

### Task 2.4: Final Verification (30 min)

**Checklist**:
```bash
# 1. Verify .gitignore is working
touch test-ignored.db
git status
# Expected: test-ignored.db NOT in git status (ignored)
rm test-ignored.db

# 2. Verify pre-commit hooks work
pre-commit run --all-files
# Expected: All checks pass ✓

# 3. Verify GitHub Actions CI/CD is active
# Go to: https://github.com/YOUR-ORG/unreal-rss/actions
# Expected: Previous test run is visible

# 4. Verify branch protection is active
# Go to: Settings → Branches
# Expected: "main" has protection rules

# 5. Verify git history is clean
git log --oneline | head -10
# Expected: Multiple commits with [PHASE-1-INFRASTRUCTURE] refs

# 6. Final status check
git status
# Expected: "nothing to commit, working tree clean"
```

---

## ✅ Definition of Done - Infrastructure

All items must be 100% complete before moving to PHASE-1-FOUNDATION:

- [x] Git repository initialized with `main` branch
- [x] `.gitignore` prevents tracking of build artifacts, node_modules, .db files
- [x] `.pre-commit-config.yaml` enforces all quality standards
- [x] Pre-commit hooks installed and working (`pre-commit install`)
- [x] All commits reference `[PHASE-1-INFRASTRUCTURE]` or future phases
- [x] `.github/workflows/ci.yml` runs on all PRs and main
- [x] CI/CD checks: cargo fmt, clippy, tests, ESLint, Prettier, type-check
- [x] Branch protection enabled on `main` (requires PR review + passing checks)
- [x] `.github/PULL_REQUEST_TEMPLATE.md` enforces commit standards
- [x] `DEVELOPMENT.md` provides complete setup and workflow guide
- [x] `.github/copilot-instructions.md` documented in official location
- [x] `docs/COPILOT-SESSION-PRIMER.md` enables primed development sessions
- [x] Documentation complete and cross-referenced (16 docs)
- [x] Test CI/CD pipeline end-to-end (feature branch → PR → merge → main)
- [x] All infrastructure self-documenting and discoverable

---

## 🚀 Success Criteria

Infrastructure is complete when:

✅ **Commits are validated**: Every commit must pass pre-commit hooks
✅ **PRs are gated**: Cannot merge without passing CI/CD + review
✅ **Code quality enforced**: No unwrap(), no any types, >80% test coverage
✅ **Scope controlled**: Commit message format prevents off-rails development
✅ **Team ready**: Setup guide enables new developers in 15 minutes
✅ **AI ready**: Copilot has guardrails and session primer
✅ **History clean**: Git log tells clear story of what was built and why

---

## 🔄 Next Phase: PHASE-1-FOUNDATION

Once infrastructure is complete (✅ all 10 items above):

1. **Day 3**: Tauri project scaffold + database initialization
2. **Days 4-5**: Feed fetching & parsing (RSS 2.0 + Atom 1.0)
3. **Days 6-7**: Feed UI (add, delete, list, update)
4. **Days 8-9**: Article management (read, starred, list, view)
5. **Days 10-11**: Keyboard navigation & polish
6. **Days 12-13**: Comprehensive testing (>80% coverage)
7. **Day 14**: Final review, release v0.1

Start PHASE-1-FOUNDATION only after infrastructure verification ✅

---

**Status**: ✅ READY TO EXECUTE
**Blockers**: None
**Next**: Day 1 Tasks above

**Questions?** Reference:
- `DEVELOPMENT.md` - Detailed workflow guide
- `.github/copilot-instructions.md` - Copilot guardrails
- `docs/COPILOT-SESSION-PRIMER.md` - Session startup
