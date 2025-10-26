## Description

<!-- Brief description of what this PR does -->

## Type of Change

<!-- Mark the relevant option with an "x" -->

- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ New feature (non-breaking change that adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to change)
- [ ] 📚 Documentation update
- [ ] 🔧 Infrastructure/tooling change
- [ ] ♻️ Refactoring (no functional changes)

## Phase Reference

<!-- Which phase does this belong to? -->

- [ ] PHASE-1-INFRASTRUCTURE (git setup, pre-commit, CI/CD)
- [ ] PHASE-1-FOUNDATION (MVP features)
- [ ] PHASE-2-READER (Reader UX)
- [ ] PHASE-3-POLISH (Search, export, polish)
- [ ] PHASE-4-BACKEND (Backend/sync)

**Spec Reference**: Link to the relevant spec or issue

## Acceptance Criteria Met

<!-- From the spec, list the acceptance criteria and check them off -->

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

## Validation Checklist

<!-- Use the validation checklist from the relevant spec -->

### Code Quality

- [ ] Rust code passes `cargo fmt --all -- --check`
- [ ] Rust code passes `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] TypeScript passes `npm run lint`
- [ ] TypeScript passes `npm run format --check`
- [ ] No `unwrap()` in production Rust code
- [ ] No `any` types in TypeScript
- [ ] All public functions have doc comments

### Testing

- [ ] New tests added (unit + integration)
- [ ] All tests pass: `cargo test && npm run test`
- [ ] Test coverage ≥ 80% for new code
- [ ] Error cases tested, not just happy path
- [ ] No `#[ignore]` in production tests

### Documentation

- [ ] Documentation updated (if needed)
- [ ] Commit message follows format: `<type>(<scope>): <desc> [PHASE-X]`
- [ ] All spec acceptance criteria referenced

### Definition of Done

- [ ] Code reviewed and approved
- [ ] CI/CD pipeline passes
- [ ] Validation checklist 100% complete
- [ ] Ready for next phase (or phase complete)

## Testing Done

<!-- Describe the testing you performed -->

## Screenshots (if applicable)

<!-- Add screenshots for UI changes -->

## Related Issues

<!-- Link to related issues: Closes #123 -->

## Pre-Merge Checklist

- [ ] I have read the relevant spec document (PHASE-X-XXXXX.md)
- [ ] All acceptance criteria are met
- [ ] I have followed .github/copilot-instructions.md standards
- [ ] Commit message references the phase: `[PHASE-X]`
- [ ] No scope creep (Phase 2+ features deferred)
- [ ] Ready for code review

---

**Note**: All PRs require approval from at least one reviewer before merging. Ensure your commit message references the spec phase for easy tracking.
