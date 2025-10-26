# Documentation Guide

This directory contains all specifications, design docs, and development guidelines for Unreal RSS.

## Quick Navigation

### Start Here
- **[PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md)** - Vision, core principles, MVP features, technical stack
- **[DEVELOPMENT-GUIDELINES.md](DEVELOPMENT-GUIDELINES.md)** - How to set up environment, run tests, submit changes

### For AI/Copilot Development
- **[COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md)** - Hard guardrails for code quality, style, and testing
- **[SPEC-TEMPLATE.md](SPEC-TEMPLATE.md)** - Template for writing phases/specs (includes acceptance criteria, validation checklist, DoD)

### Phase & Spec Documents
- **[phases/](phases/)** - Development phases with acceptance criteria and validation checklists
  - `PHASE-1-FOUNDATION.md` - Tauri scaffold, basic feed parsing
  - `PHASE-2-READER.md` - Reader UX, typography, themes
  - `PHASE-3-POLISH.md` - Search, export, performance
  - `PHASE-4-BACKEND.md` - API design, sync foundation

- **[specs/](specs/)** - Detailed technical specifications
  - `ARCHITECTURE.md` - System design, module breakdown
  - `DATABASE.md` - SQLite schema, migrations
  - `API.md` - Tauri command definitions
  - `UI-DESIGN.md` - User flows, wireframes

### Design & Research
- **[design/](design/)** - User flows, wireframes, design system
- Research notes and design iterations

## How to Work with Specs

### Reading a Spec
1. Start with **Overview** - high-level summary
2. Review **Objectives** - what needs to be done
3. Check **Acceptance Criteria** - what passes/fails
4. Read **Implementation Guide** - step-by-step approach
5. Verify **Validation Checklist** - how to validate the work
6. Understand **Definition of Done** - quality gates

### Writing Code to a Spec
1. Read the spec completely
2. Write tests FIRST (TDD)
3. Implement to pass tests
4. Run validation checklist
5. Commit with spec reference: `feat(backend): ... [PHASE-1-FOUNDATION]`
6. Ensure Definition of Done is met before merging

### Spec Structure (Required)
Every phase and spec must include:
- ✅ **Acceptance Criteria** - Specific, testable requirements
- ✅ **Validation Checklist** - How to verify implementation
- ✅ **Definition of Done** - Quality gates before merge
- ✅ **Copilot Guardrails** - Specific standards for AI coding

See [SPEC-TEMPLATE.md](SPEC-TEMPLATE.md) for the complete template.

## Code Quality Standards

### Pre-Commit Guardrails (Enforced)
```bash
# Rust
cargo fmt --check    # Code formatting
cargo clippy         # Linting (zero warnings allowed)
cargo test           # All tests must pass

# TypeScript
npm run lint         # ESLint
npm run format       # Prettier
npm run type-check   # TypeScript strict mode
npm run test         # All tests must pass
```

### Enterprise-Grade Requirements
- ✅ 80%+ test coverage for new code
- ✅ All public APIs documented
- ✅ All commits reference a spec
- ✅ Code review required before merge
- ✅ No TODO comments without context
- ✅ Error handling required at all boundaries

See [COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md) for detailed standards.

## Development Workflow

### 1. Select a Phase
- Start with Phase 1: Foundation
- Don't skip phases or cherry-pick features

### 2. Review the Spec
- Read the full phase spec document
- Understand acceptance criteria
- Review architecture changes if any

### 3. Create Feature Branch
```bash
git checkout -b phase-1-foundation
```

### 4. Implement & Test
- Write tests first (TDD)
- Implement to pass tests
- Run validation checklist frequently

### 5. Commit with Spec Reference
```bash
git commit -m "feat(backend): add feed fetching [PHASE-1-FOUNDATION]"
```

### 6. Verify Definition of Done
- [ ] All acceptance criteria met
- [ ] All validation checklist items pass
- [ ] 80%+ test coverage
- [ ] Code review passed
- [ ] Commit messages reference spec

### 7. Merge
- PR must be reviewed before merge
- Merge only when Definition of Done is complete

## Key Files by Role

### For Project Leads
- PROJECT-OVERVIEW.md - Understand vision and scope
- DEVELOPMENT-GUIDELINES.md - Understand process

### For Developers (AI or Human)
- Current phase spec - What to build
- COPILOT-INSTRUCTIONS.md - How to code
- SPEC-TEMPLATE.md - How to understand specs

### For Code Reviewers
- Validation Checklist in spec - What to verify
- COPILOT-INSTRUCTIONS.md - Code standards
- Definition of Done - Quality gates

## Documentation Standards

### Commit Messages
```
<type>(<scope>): <description> [SPEC-REF]

Examples:
feat(backend): add feed fetching and parsing [PHASE-1-FOUNDATION]
fix(frontend): handle loading state in article view [PHASE-1-FOUNDATION]
test(backend): add tests for feed parser [PHASE-1-FOUNDATION]
docs: update README with architecture overview [PHASE-1-FOUNDATION]
```

### File Naming
- Phases: `PHASE-N-DESCRIPTIVE-NAME.md`
- Specs: `DESCRIPTIVE-NAME.md`
- Implementation: Use standard language conventions

### Cross-References
- Always link to related specs
- Reference specs in commit messages
- Update indexes when creating new docs

## Frequently Asked Questions

### Q: Should I create a new spec file or add to an existing one?
**A**: One spec file per feature/phase. Don't combine features. Use the SPEC-TEMPLATE.md for structure.

### Q: What if the spec is ambiguous?
**A**: Ask the human/project lead before coding. It's better to clarify than to guess.

### Q: Can I skip tests?
**A**: No. Tests are required for all production code. See COPILOT-INSTRUCTIONS.md for testing standards.

### Q: What if I find a bug in existing code?
**A**: File it as a separate spec/issue. Don't mix bug fixes with feature implementation.

### Q: How much documentation is enough?
**A**: All public functions must have doc comments with examples for non-trivial ones. See COPILOT-INSTRUCTIONS.md for examples.

---

**Last Updated**: October 2025
**Maintained by**: Unreal RSS Team
