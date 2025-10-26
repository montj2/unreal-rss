# PHASE 1 INFRASTRUCTURE SPEC

**Status**: Part of Phase 1 - Foundation
**Duration**: ~3-4 days (done first, before implementation)
**Reference**: [PHASE-1-INFRASTRUCTURE]

## Overview

Before any feature code is written, the project infrastructure must be solid. This spec covers git setup, pre-commit hooks, commit standards, development workflow, and CI/CD foundations.

**Goal**: Establish an airtight development process that prevents off-the-rails development.

## Acceptance Criteria

### Git Setup
- [ ] Repository initialized with proper .gitignore
- [ ] Branch protection configured for `main`
- [ ] Development workflow documented (branching strategy)
- [ ] All team members can clone and develop
- [ ] No accidental secrets/commits to main

### Pre-Commit Hooks
- [ ] Pre-commit framework installed (.pre-commit-config.yaml)
- [ ] Rust formatting (cargo fmt) enforced
- [ ] Rust linting (cargo clippy -D warnings) enforced
- [ ] Rust tests (cargo test) must pass
- [ ] TypeScript linting (ESLint) enforced
- [ ] TypeScript formatting (Prettier) enforced
- [ ] TypeScript type-check passes
- [ ] TypeScript tests pass
- [ ] Commit message format validated
- [ ] All checks run locally before push
- [ ] No way to bypass hooks (except explicit override with consequences)

### Commit Standards
- [ ] Format: `<type>(<scope>): <description> [PHASE-X-NAME]`
- [ ] All commits reference a spec
- [ ] Clear commit history with no "WIP" or "temp" commits
- [ ] Each commit is atomic and focused
- [ ] All commits pass validation before merge

### Development Workflow
- [ ] Clear branching strategy documented (gitflow or trunk-based)
- [ ] Feature branches created from `main`
- [ ] PR required for all changes (even small ones)
- [ ] PR title references spec
- [ ] PR description links to spec and validation checklist
- [ ] Minimum 1 review before merge
- [ ] All checks pass before merge allowed
- [ ] Squash commits on merge (clean history)

### CI/CD Foundation
- [ ] GitHub Actions workflow file created
- [ ] Rust tests run on every PR
- [ ] TypeScript tests run on every PR
- [ ] Linting/formatting checks run on every PR
- [ ] Build artifacts generated (for testing)
- [ ] CI failures block merge

### Documentation
- [ ] Git workflow documented in DEVELOPMENT-GUIDELINES.md
- [ ] Pre-commit hook setup instructions documented
- [ ] Commit message examples provided
- [ ] PR process documented
- [ ] Troubleshooting guide for common issues

## Technical Requirements

### Tools & Versions
- Git 2.30+
- pre-commit 2.20+
- Rust 1.70+ (via rustup)
- Node.js 18+ (via nvm recommended)
- Python 3.8+ (for pre-commit)

### Files to Create/Configure
- `.gitignore` - Comprehensive ignores for Rust + Node
- `.pre-commit-config.yaml` - Pre-commit hook configuration
- `.github/workflows/ci.yml` - GitHub Actions CI/CD
- `.github/PULL_REQUEST_TEMPLATE.md` - PR template
- `DEVELOPMENT.md` - Developer setup and workflow guide

### Branch Strategy

**Recommendation: Trunk-Based Development with Feature Branches**

```
main (always production-ready)
  ├── feature/phase-1-foundation (feature branch)
  ├── feature/phase-1-feed-parsing (feature branch)
  └── feature/phase-1-ui (feature branch)

Rules:
- Branch from: main
- Branch naming: feature/PHASE-X-descriptive-name
- PR required: Yes, always
- Squash on merge: Yes
- Delete branch after merge: Yes
- Max branch lifetime: 5 business days
```

## Implementation Guide

### Step 1: Initialize Git Configuration (Day 1 - 2 hours)

1. Create `.gitignore` for Rust + Node.js + OS files
2. Create `.git/hooks/pre-commit` (or use pre-commit framework)
3. Test that `.gitignore` works correctly
4. Verify nothing sensitive is committed

**Files to create:**
- `.gitignore`
- `.editorconfig` (optional but recommended)

**Deliverable**: Clean git history, nothing sensitive

### Step 2: Set Up Pre-Commit Hooks (Day 1 - 3 hours)

1. Install pre-commit: `pip install pre-commit`
2. Create `.pre-commit-config.yaml` with:
   - Rust formatters (cargo fmt)
   - Rust linters (cargo clippy)
   - Rust tests (cargo test)
   - TypeScript linters (ESLint)
   - TypeScript formatters (Prettier)
   - Type checking (tsc)
   - TypeScript tests (jest/vitest)
   - Commit message validation (commitlint)

3. Install hooks locally: `pre-commit install`
4. Test hooks work on a sample commit
5. Document any troubleshooting

**File to create:**
- `.pre-commit-config.yaml`

**Validation**: Attempt to commit bad code → hooks block it

### Step 3: Set Up GitHub Actions CI/CD (Day 2 - 4 hours)

1. Create `.github/workflows/ci.yml` that:
   - Runs on every PR
   - Checks Rust formatting
   - Runs cargo clippy
   - Runs cargo test
   - Checks TypeScript linting
   - Runs TypeScript tests
   - Verifies commit messages
   - Reports pass/fail in PR

2. Configure branch protection on `main`:
   - Require PR before merge
   - Require status checks to pass
   - Require code review
   - Dismiss approvals on new commits

3. Test CI by creating a test PR with intentional failures

**Files to create:**
- `.github/workflows/ci.yml`
- `.github/PULL_REQUEST_TEMPLATE.md`

**Validation**: CI blocks bad PRs, allows good ones

### Step 4: Documentation & Training (Day 2 - 3 hours)

1. Update `DEVELOPMENT.md`:
   - Setup instructions
   - Branch creation workflow
   - Commit guidelines
   - PR process
   - Common troubleshooting

2. Create `DEVELOPMENT-CHECKLIST.md`:
   - First-time setup
   - Daily workflow
   - Before pushing changes
   - When troubleshooting

3. Document pre-commit bypass (emergency only):
   ```bash
   git commit --no-verify  # Only in true emergencies
   ```

**Files to create/update:**
- `DEVELOPMENT.md` (comprehensive guide)
- `DEVELOPMENT-CHECKLIST.md` (quick reference)

**Deliverable**: New developers can onboard in <1 hour

## Validation Checklist

**All of these must pass before Phase 1 code begins:**

### Git & Repo
- [ ] `.gitignore` is comprehensive (no accidental commits)
- [ ] `.editorconfig` set up for consistency
- [ ] `main` branch is protected (no direct commits)
- [ ] Branch naming convention documented
- [ ] Team can clone and develop
- [ ] No secrets in git history

### Pre-Commit Hooks
- [ ] Pre-commit framework installed
- [ ] `.pre-commit-config.yaml` configured
- [ ] All hooks run locally before push
- [ ] Hooks block bad code (test with intentional violations)
- [ ] Hooks don't block good code
- [ ] Setup time for new developer: <10 minutes
- [ ] Developers understand when/why hooks trigger

### CI/CD
- [ ] `.github/workflows/ci.yml` created and tested
- [ ] CI runs on every PR
- [ ] CI blocks merge when checks fail
- [ ] CI passes when code is good
- [ ] CI status visible in PR
- [ ] Developers understand CI output

### Commit & PR Process
- [ ] First commit created with proper format: `feat(setup): initialize project [PHASE-1-INFRASTRUCTURE]`
- [ ] PR process tested (create, approve, merge)
- [ ] Commit messages consistently formatted
- [ ] All commits reference spec
- [ ] PR template populated correctly
- [ ] Merge strategy consistent (squash on merge)

### Documentation
- [ ] `DEVELOPMENT.md` complete and clear
- [ ] Setup instructions tested (clean clone → works)
- [ ] Commit format documented with examples
- [ ] PR process documented with screenshots
- [ ] Troubleshooting guide covers common issues
- [ ] Pre-commit hook issues documented

## Definition of Done

**Infrastructure is complete when ALL of these are true:**

1. ✅ **Git clean** - Proper .gitignore, branch protection, no secrets
2. ✅ **Pre-commit working** - All hooks configured and tested
3. ✅ **CI/CD working** - GitHub Actions running, blocking bad code
4. ✅ **Process documented** - Developers can follow it without questions
5. ✅ **Team trained** - Everyone understands the workflow
6. ✅ **First commit made** - Proper format, passes all checks
7. ✅ **First PR merged** - Full process tested end-to-end
8. ✅ **No regressions** - Main branch is clean and ready

## Copilot Guardrails

**When setting up infrastructure, Copilot must:**

- [ ] **Follow existing patterns** - Don't invent new processes
- [ ] **Document everything** - All configs commented and explained
- [ ] **Test thoroughly** - Verify each component works
- [ ] **Ask for clarification** - If requirements are unclear
- [ ] **No shortcuts** - Every guardrail is intentional
- [ ] **Version control configs** - All configs in git
- [ ] **Update documentation** - Every config change documented

## Timeline Allocation

**Days 1-2 of Phase 1 (before any feature code):**

| Time | Task | Owner |
|------|------|-------|
| Day 1, Morning (2h) | Git setup + .gitignore | Copilot |
| Day 1, Afternoon (3h) | Pre-commit hooks | Copilot |
| Day 2, Morning (4h) | CI/CD GitHub Actions | Copilot |
| Day 2, Afternoon (3h) | Documentation | Copilot |
| Day 2, EOD | Infrastructure review + first commit | Human |

**Days 3-14 of Phase 1:**
Feature implementation with infrastructure in place

**Success metric**: All 14 days' worth of features complete with zero process violations

---

**Must be 100% complete before Phase 1 feature implementation begins.**

**Status**: Ready to implement
**Blocker for**: Phase 1 feature work
**Next**: PHASE-1-FOUNDATION feature specs (ARCHITECTURE, DATABASE, API, UI-DESIGN)
