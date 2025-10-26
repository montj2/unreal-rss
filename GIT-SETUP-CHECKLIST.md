# Initial Git Setup & First Commit Checklist

**Status**: Ready to execute
**Expected Time**: 30-45 minutes
**Outcome**: Git repo initialized, all infrastructure files committed, ready for GitHub

---

## Pre-Requisites Check

Before proceeding, verify these are installed:

```bash
# Run this to verify
rustc --version      # Should show Rust version
cargo --version      # Should show Cargo version
node --version       # Should show v18.x or higher
npm --version        # Should show npm version
git --version        # Should show git version
pre-commit --version # Should show pre-commit version
```

If any command shows "not found", complete `TOOLS-SETUP.md` first.

**Status**: ⬜ Verify all tools installed

---

## Step 1: Initialize Git Repository (5 min)

```bash
cd /home/montj2/unreal-rss

# Check if git is already initialized
git status

# If "fatal: not a git repository", initialize:
git init
git branch -M main

# Configure git user
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Verify
git config --list | grep user
```

**Expected Output**:
```
user.name=Your Name
user.email=your.email@example.com
```

**Acceptance Criteria**:
- [ ] Git initialized in `/home/montj2/unreal-rss/`
- [ ] Main branch exists
- [ ] Git user configured (name + email)
- [ ] `git status` works without error

---

## Step 2: Install npm Dependencies (10 min)

```bash
cd /home/montj2/unreal-rss

# Install dependencies
npm install

# Verify installation
npm list --depth=0  # Shows main dependencies
```

**Expected Output**: Shows package list without errors

**Acceptance Criteria**:
- [ ] `node_modules/` directory created
- [ ] `package-lock.json` updated
- [ ] No errors during install
- [ ] `npm --version` works

---

## Step 3: Setup Pre-commit Hooks (5 min)

```bash
cd /home/montj2/unreal-rss

# Install pre-commit hooks into git
pre-commit install
pre-commit install --hook-type pre-commit

# Verify installation
ls -la .git/hooks/ | grep pre-commit

# Expected: pre-commit -> .../.pre-commit/somepath
```

**Acceptance Criteria**:
- [ ] `.git/hooks/pre-commit` exists and is symlinked
- [ ] `pre-commit install` completed without error
- [ ] `ls -la .git/hooks/` shows `pre-commit` entry

---

## Step 4: Run Pre-commit Check (5-10 min)

```bash
cd /home/montj2/unreal-rss

# Run pre-commit on all files
pre-commit run --all-files

# Expected: Some tools might fail if not installed yet, but no critical errors
# The following might error (expected if tools not installed):
#   - cargo fmt (no Rust project yet)
#   - cargo clippy (no Rust project yet)
#   - cargo test (no Rust project yet)
#
# The following should pass:
#   - prettier (formats markdown)
#   - check-json
#   - check-yaml
#   - trailing-whitespace
#   - end-of-file-fixer
```

**Note**: It's OK if Rust-related checks fail at this stage - we'll create the Rust project later.

**Acceptance Criteria**:
- [ ] Pre-commit runs without fatal errors
- [ ] File checks pass (trailing whitespace, end-of-file, JSON, YAML)
- [ ] File modifications made by auto-fixes are visible

---

## Step 5: Commit Auto-Fixed Files (5 min)

After pre-commit runs, it may auto-fix some files. Commit these fixes:

```bash
cd /home/montj2/unreal-rss

# Check what changed
git status

# Add all changes (from pre-commit auto-fixes)
git add -A

# If there are no changes (nothing to commit), skip to Step 6

# Commit auto-fixes
git commit -m "chore: pre-commit auto-fixes [PHASE-1-INFRASTRUCTURE]"

# Verify
git log --oneline -1
```

**Acceptance Criteria**:
- [ ] Auto-fixed files committed (or nothing to commit, that's OK)
- [ ] Commit message references `[PHASE-1-INFRASTRUCTURE]`
- [ ] `git status` shows clean working tree (after adding)

---

## Step 6: Add All Documentation & Infrastructure Files (5 min)

```bash
cd /home/montj2/unreal-rss

# Verify files exist
ls -la .gitignore
ls -la .pre-commit-config.yaml
ls -la .github/workflows/ci.yml
ls -la .github/PULL_REQUEST_TEMPLATE.md
ls -la DEVELOPMENT.md
ls -la docs/COPILOT-SESSION-PRIMER.md

# Add everything
git add .

# Verify what's being committed
git status

# Should show all these files "ready to commit"
```

**Acceptance Criteria**:
- [ ] All infrastructure files listed in `git status`
- [ ] All documentation files listed in `git status`
- [ ] Staging area ready for commit

---

## Step 7: Create Initial Commit (2 min)

```bash
cd /home/montj2/unreal-rss

# Commit all infrastructure and docs
git commit -m "feat(setup): initial documentation and infrastructure [PHASE-1-INFRASTRUCTURE]"

# Verify commit was created
git log -1 --oneline
git log -1 --stat  # Shows all files committed
```

**Expected Output**:
```
abc1234 feat(setup): initial documentation and infrastructure [PHASE-1-INFRASTRUCTURE]

Commit summary will show:
- docs/
- .github/
- .gitignore
- .pre-commit-config.yaml
- DEVELOPMENT.md
- (all other infrastructure files)
```

**Acceptance Criteria**:
- [ ] Initial commit created
- [ ] Commit message references `[PHASE-1-INFRASTRUCTURE]`
- [ ] All infrastructure files in commit
- [ ] All documentation in commit
- [ ] `git log` shows clean history

---

## Step 8: Verify Repository State (5 min)

```bash
cd /home/montj2/unreal-rss

# Check git status
git status
# Expected: "nothing to commit, working tree clean"

# Check git log
git log --oneline
# Expected: Shows your initial commit(s)

# Check branch
git branch
# Expected: * main

# Verify .gitignore is working
touch test.db
git status
# Expected: test.db NOT listed (it's ignored)
rm test.db

# Run pre-commit one more time to verify setup
pre-commit run --all-files
# Expected: Passes (or minimal errors for Rust components)
```

**Acceptance Criteria**:
- [ ] Working tree clean (`git status` shows nothing)
- [ ] Commits present in history (`git log`)
- [ ] `.gitignore` properly ignoring files
- [ ] Pre-commit hooks operational

---

## Step 9: Create GitHub Repository (Online)

Go to GitHub.com and:

1. Click "+" → "New repository"
2. Name it: `unreal-rss`
3. Description: "Reading-first RSS reader built with Tauri + Rust + React"
4. Choose: **Private** (for now)
5. Click "Create repository"
6. **Do NOT initialize with README** (we already have one)

**After repo created**, you'll see instructions like:

```bash
git remote add origin https://github.com/YOUR-USERNAME/unreal-rss.git
git branch -M main
git push -u origin main
```

---

## Step 10: Push to GitHub (5 min)

```bash
cd /home/montj2/unreal-rss

# Add GitHub as remote
git remote add origin https://github.com/YOUR-USERNAME/unreal-rss.git

# Verify remote added
git remote -v
# Expected:
# origin  https://github.com/YOUR-USERNAME/unreal-rss.git (fetch)
# origin  https://github.com/YOUR-USERNAME/unreal-rss.git (push)

# Push to GitHub
git push -u origin main

# Verify
git branch -vv
# Expected: main tracking origin/main
```

**Acceptance Criteria**:
- [ ] Remote added (`git remote -v` shows origin)
- [ ] Push successful (no auth errors)
- [ ] Branch tracking origin/main

---

## Step 11: Share Git URL with Me

Once pushed, provide me with:

1. **Repository URL**: `https://github.com/YOUR-USERNAME/unreal-rss`
2. **Branch**: `main`
3. **Commit hash**: (from `git log -1 --oneline`)

I'll then:
- Verify the repository
- Check all files are present
- Validate GitHub Actions workflow setup
- Prepare for PHASE-1-INFRASTRUCTURE execution

---

## ✅ Completion Checklist

After all 11 steps, verify:

- [ ] Git repository initialized locally
- [ ] npm dependencies installed (`node_modules/` exists)
- [ ] Pre-commit hooks installed in `.git/hooks/`
- [ ] All documentation files committed
- [ ] All infrastructure files committed
- [ ] Initial commit created with `[PHASE-1-INFRASTRUCTURE]` tag
- [ ] GitHub repository created
- [ ] Code pushed to GitHub
- [ ] Main branch protected (if available in GitHub)
- [ ] Git repository URL shared with me

---

## Troubleshooting

### "fatal: not a git repository"
```bash
cd /home/montj2/unreal-rss
git init
git branch -M main
```

### "Permission denied" when pushing to GitHub
→ Set up SSH key or use personal access token for HTTPS

### Pre-commit hook rejected my commit
→ Check error message, fix files, and try again

### ".gitignore not working"
→ If file was already tracked: `git rm --cached filename`

### "node_modules not installed"
→ Run: `npm install` from project root

### Pre-commit says "command not found: cargo"
→ Expected! Rust project not created yet. This is OK.

---

## Next After Completion

Once all steps complete and URL shared:

1. **PHASE-1-INFRASTRUCTURE execution**
   - Verify GitHub Actions CI/CD working
   - Set up branch protection
   - Test pre-commit enforcement

2. **PHASE-1-FOUNDATION implementation**
   - Create Tauri project scaffold
   - Set up database
   - Begin feature development

---

**Status**: 🟢 Ready to execute
**Estimated Time**: 30-45 minutes
**Next**: Complete steps above, share GitHub URL
