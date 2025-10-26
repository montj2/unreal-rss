# Documentation & Spec Template

Use this template for all phase and spec documents to maintain consistency and clarity for AI-assisted development.

## Template Structure

```markdown
# [PHASE/SPEC NAME]

## Overview
[High-level summary of what this phase/spec accomplishes]

## Objectives
- [ ] Specific, measurable objective 1
- [ ] Specific, measurable objective 2
- [ ] Specific, measurable objective 3

## Acceptance Criteria

### Feature: [Feature Name]
- [ ] Criterion 1: [Specific, testable requirement]
- [ ] Criterion 2: [Specific, testable requirement]
- [ ] Criterion 3: [Specific, testable requirement]

## Technical Requirements

### Architecture Changes
[Describe any architectural changes, new modules, or refactoring]

### Database Changes
[If applicable: schema migrations, new tables, etc.]

### API Changes
[New Tauri commands, command signatures, return types]

### Dependencies
- [ ] New crate: `name` (version, justification)
- [ ] New npm package: `name` (version, justification)

## Implementation Guide

### Step 1: [Task Name]
[Detailed steps for implementing this task]

### Step 2: [Task Name]
[Detailed steps for implementing this task]

## Validation Checklist

### Code Quality
- [ ] `cargo fmt` passes (no formatting issues)
- [ ] `cargo clippy` passes (zero warnings)
- [ ] All new public functions have doc comments
- [ ] All public types are properly documented
- [ ] No TODO comments left in code

### Testing
- [ ] Unit tests written for all new logic
- [ ] Integration tests written for feature workflows
- [ ] Test coverage >80% for new code
- [ ] All tests pass locally: `cargo test`
- [ ] No skipped tests

### Frontend (TypeScript/React)
- [ ] ESLint passes: `npm run lint`
- [ ] Prettier formatting: `npm run format`
- [ ] TypeScript type checking passes: `npm run type-check`
- [ ] All React components have PropTypes/TS types
- [ ] No console warnings/errors

### Documentation
- [ ] All new modules have module-level docs
- [ ] All public functions documented with examples
- [ ] README updated if needed
- [ ] Commit messages reference this spec

## Definition of Done

**A feature is complete when ALL of the following are true:**

1. ✅ **Code is written** - Implementation matches acceptance criteria
2. ✅ **Tests pass** - Unit, integration tests written and passing
3. ✅ **Linting passes** - No clippy warnings (Rust), ESLint errors (TS)
4. ✅ **Documentation complete** - Code comments, API docs, README updated
5. ✅ **Peer review** - Code reviewed by project lead
6. ✅ **No regressions** - All existing tests still pass
7. ✅ **Performance verified** - Meets success metrics (if applicable)
8. ✅ **Commit message** - References spec, clear description

## Copilot Guardrails

**When implementing this spec, Copilot must:**

- [ ] Follow all code style guidelines from COPILOT-INSTRUCTIONS.md
- [ ] Write tests FIRST (TDD approach)
- [ ] Add doc comments to all public APIs
- [ ] Never skip error handling
- [ ] Validate user input at API boundaries
- [ ] Use consistent error types across codebase
- [ ] Reference this spec in all commit messages

## Success Criteria

[Optional: Specific metrics to measure success]
- Performance target: [if applicable]
- Coverage target: [if applicable]
- User flow: [if applicable]

---

**Status**: Not Started
**Assigned to**: [TBD]
**Started**: [Date]
**Completed**: [Date]
```

## Usage Rules

1. **One spec per file** - Don't combine phases or features in one document
2. **Clear acceptance criteria** - Use checkbox format, make each criterion specific and testable
3. **Validation checklist is mandatory** - Every merge must verify ALL checkboxes
4. **Definition of Done applies to everything** - No exceptions
5. **Reference this spec in commits** - Format: `feat(phase-1): description [PHASE-1-FOUNDATION]`

---

See `COPILOT-INSTRUCTIONS.md` for additional coding guardrails.
