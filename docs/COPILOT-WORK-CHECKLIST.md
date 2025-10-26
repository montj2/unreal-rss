# Copilot Work Checklist

Use this checklist when working with Copilot (or other AI assistants) on any feature/phase.

## Pre-Implementation

- [ ] **Read the spec completely** - Understand all sections before asking Copilot to code
- [ ] **Identify acceptance criteria** - Know exactly what passes/fails
- [ ] **Review validation checklist** - Understand quality gates
- [ ] **Check Copilot instructions** - Know the coding standards
- [ ] **Plan the implementation** - Outline steps Copilot should follow
- [ ] **Identify edge cases** - Think about error scenarios

## During Implementation

### For Each Feature/Component

- [ ] **Start with tests** - Ask Copilot to write tests FIRST (TDD)
- [ ] **Then implement** - Ask for implementation to pass tests
- [ ] **Run validation frequently** - Check formatting, linting, clippy
- [ ] **Review generated code** - Is it correct? Does it match the spec?
- [ ] **Verify no unwrap()** - Check Rust code for error handling
- [ ] **Check documentation** - Are doc comments present?
- [ ] **Test manually** - Try the feature in the app

### Code Review (Before Commit)

- [ ] **Does it match the spec?** - Acceptance criteria all met?
- [ ] **Code quality checks pass?** - `cargo fmt`, `cargo clippy`, ESLint, Prettier
- [ ] **Tests pass?** - `cargo test && npm run test`
- [ ] **Coverage >80%?** - New code is tested
- [ ] **Doc comments present?** - All public APIs documented
- [ ] **No console warnings?** - Clean output
- [ ] **Error handling complete?** - No unwrap() in production
- [ ] **No TODO comments?** - Clean code

## Commit & Validation

- [ ] **Commit message format** - `<type>(<scope>): <description> [PHASE-1-FOUNDATION]`
- [ ] **Reference the spec** - Include [PHASE-X-...] tag
- [ ] **Run full validation** - All checks from validation checklist
- [ ] **Manual test** - Try the feature works in the app
- [ ] **No regressions** - Existing functionality still works

## Common Copilot Prompts

### Starting Fresh on a Component

```
I need to implement [Component Name] for Phase 1 Foundation.

Here's what it needs to do:
[Paste relevant acceptance criteria]

Here are the relevant guidelines:
[Point to COPILOT-INSTRUCTIONS.md section]

Start by writing tests FIRST using TDD approach. Then implement the component.
Make sure to:
- Write tests before implementation
- Add doc comments to all public functions
- Handle errors explicitly (no unwrap)
- Reference [PHASE-1-FOUNDATION] in commit messages
```

### Fixing Issues

```
This test is failing: [test name and error]

Here's the relevant code: [code snippet]

The expected behavior is: [describe what should happen]

Debug this and fix it, then verify the tests pass.
```

### Code Review

```
Review this code against the spec: [spec reference]

Check:
- Does it meet all acceptance criteria?
- Are there any clippy warnings?
- Are all public functions documented?
- Is error handling complete?

Point out any issues and how to fix them.
```

### Before Merging

```
I'm about to merge this PR for [PHASE-X].

Run the full validation checklist from the spec:
[Paste checklist items]

Tell me if anything is missing or needs fixing.
```

## Red Flags - Stop and Review

⚠️ **Stop work and review if:**

- [ ] Copilot generated code with `unwrap()` or `panic!()`
- [ ] No tests were written
- [ ] Acceptance criteria don't match implementation
- [ ] Code has no doc comments
- [ ] Clippy shows warnings
- [ ] TypeScript has type errors
- [ ] Error handling is missing
- [ ] Commit message doesn't reference spec
- [ ] Coverage dropped below 80%

**When you see a red flag:**
1. Don't commit that code
2. Ask Copilot to fix it
3. Reference the spec and relevant guidelines
4. Re-run validation checks

## Validation Commands

Run these before EVERY commit:

```bash
# Rust validation
cargo fmt --check         # Format check
cargo clippy -- -D warnings  # Linting (zero warnings allowed)
cargo test                # All tests must pass

# TypeScript validation
npm run lint              # ESLint check
npm run format --check    # Prettier check
npm run type-check        # TypeScript strict mode
npm run test              # All tests must pass

# Combined check
cargo fmt --check && cargo clippy -- -D warnings && cargo test && \
  npm run lint && npm run format --check && npm run type-check && npm run test
```

## When to Ask for Help

✋ **Ask the human/project lead if:**

- Spec is ambiguous or contradictory
- Two design approaches seem equally valid
- New dependency needed (must be approved)
- Performance concern or technical issue
- Something doesn't fit the stated architecture
- Unsure about implementation approach

**Example message to human:**
```
The spec for [feature] is unclear about [question].

It says: [quote from spec]

But it's not clear if we should [option A] or [option B].

Which approach is correct?
```

## Quality Gates Summary

**All code must pass these gates before merge:**

| Check | Command | Status |
|-------|---------|--------|
| Rust Formatting | `cargo fmt --check` | ✅ Pass |
| Rust Linting | `cargo clippy -- -D warnings` | ✅ Pass |
| Rust Tests | `cargo test` | ✅ Pass |
| TS Linting | `npm run lint` | ✅ Pass |
| TS Formatting | `npm run format --check` | ✅ Pass |
| TS Type Check | `npm run type-check` | ✅ Pass |
| TS Tests | `npm run test` | ✅ Pass |
| Code Coverage | >80% for new code | ✅ Pass |
| Doc Comments | All public APIs | ✅ Present |
| No Unwrap | Production code | ✅ Clean |
| Spec Reference | Commit message | ✅ Present |

---

**Remember**: Spec-driven development + TDD + Copilot guardrails = Enterprise-grade code

For detailed guidelines, see:
- `COPILOT-INSTRUCTIONS.md` - Coding standards
- `SPEC-TEMPLATE.md` - How to understand specs
- `DEVELOPMENT-GUIDELINES.md` - Workflow
- Current phase spec (e.g., `PHASE-1-FOUNDATION.md`)
