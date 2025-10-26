# Documentation Index

Complete guide to all specifications and development documents for Unreal RSS.

## 📋 Core Documents (START HERE)

| Document | Purpose | Audience |
|----------|---------|----------|
| **[PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md)** | Vision, architecture, MVP features | Everyone |
| **[README.md](README.md)** | Navigation guide for all docs | Everyone |
| **[DEVELOPMENT-GUIDELINES.md](DEVELOPMENT-GUIDELINES.md)** | Setup, workflow, testing | Developers |
| **[COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md)** | Code standards & guardrails | AI/Copilot |
| **[COPILOT-SESSION-PRIMER.md](COPILOT-SESSION-PRIMER.md)** | Session startup prompts & priming | Developers + AI |
| **[COPILOT-WORK-CHECKLIST.md](COPILOT-WORK-CHECKLIST.md)** | Pre-implementation checklist | AI/Copilot Users |
| **[SPEC-TEMPLATE.md](SPEC-TEMPLATE.md)** | How to write/read specs | Everyone |

## 🎯 Development Phases

Each phase includes: Overview → Objectives → Acceptance Criteria → Implementation Guide → Validation Checklist → Definition of Done

| Phase | Status | Duration | Key Deliverables |
|-------|--------|----------|------------------|
| **[PHASE-1-FOUNDATION.md](phases/PHASE-1-FOUNDATION.md)** | Ready | 2 weeks | Tauri scaffold, feed fetching, basic UI, guardrails |
| **PHASE-2-READER.md** | Not Started | 2 weeks | Reader UX, typography, themes, keyboard nav |
| **PHASE-3-POLISH.md** | Not Started | 2 weeks | Search, export, optimization, tests |
| **PHASE-4-BACKEND.md** | Not Started | 2+ weeks | API design, sync, multi-device foundation |

## 📐 Technical Specifications

Detailed technical requirements for each system component.

| Spec | Purpose | Content |
|------|---------|---------|
| **specs/ARCHITECTURE.md** | System design | Module structure, data flow, patterns |
| **specs/DATABASE.md** | Data model | Schema design, migrations, relationships |
| **specs/API.md** | Tauri commands | All IPC endpoints, signatures, types |
| **specs/UI-DESIGN.md** | User interface | Wireframes, flows, components |

*These are created during Phase 1 as part of PHASE-1-FOUNDATION.md*

## 🎨 Design & Research

User experience, design systems, and research notes.

| Document | Purpose |
|----------|---------|
| **design/wireframes.md** | UI mockups and layouts |
| **design/flows.md** | User workflows and interactions |
| **design/design-system.md** | Colors, typography, components |

## 🚀 Quick Start Paths

### For Project Leads
1. Read: [PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md)
2. Review: [PHASE-1-FOUNDATION.md](phases/PHASE-1-FOUNDATION.md)
3. Understand: [DEVELOPMENT-GUIDELINES.md](DEVELOPMENT-GUIDELINES.md)

### For Developers (Implementing Features)
1. Read: [PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md)
2. **Prime your session**: [COPILOT-SESSION-PRIMER.md](COPILOT-SESSION-PRIMER.md)
3. Get current phase: e.g., [PHASE-1-FOUNDATION.md](phases/PHASE-1-FOUNDATION.md)
4. Review: [COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md)
5. Setup: [DEVELOPMENT-GUIDELINES.md](DEVELOPMENT-GUIDELINES.md)
6. Implement using [COPILOT-WORK-CHECKLIST.md](COPILOT-WORK-CHECKLIST.md)

### For Code Reviewers
1. Get spec for the PR (e.g., PHASE-1-FOUNDATION.md)
2. Check: Validation Checklist from spec
3. Reference: [COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md) standards
4. Verify: Definition of Done is met

### For AI/Copilot Users
1. **Each session**: Use [COPILOT-SESSION-PRIMER.md](COPILOT-SESSION-PRIMER.md) to get primed
2. **Pre-work**: Read relevant phase spec
3. **During**: Follow [COPILOT-INSTRUCTIONS.md](COPILOT-INSTRUCTIONS.md)
4. **Validation**: Use [COPILOT-WORK-CHECKLIST.md](COPILOT-WORK-CHECKLIST.md)
5. **Before commit**: Run all validation checks
6. **On issues**: Reference spec and ask for clarification

## 📚 Document Structure

### Each Phase Document Contains:
```
├── Overview
├── Objectives
├── Acceptance Criteria
├── Technical Requirements
├── Implementation Guide
├── Validation Checklist
├── Definition of Done
└── Copilot Guardrails
```

### Each Spec Document Contains:
```
├── Overview
├── Objectives
├── Acceptance Criteria
├── Technical Requirements
├── Implementation Guide
├── Validation Checklist
├── Definition of Done
└── Success Criteria
```

## 🔗 Document Relationships

```
PROJECT-OVERVIEW.md (Vision & Goals)
    ├─→ PHASE-1-FOUNDATION.md (First sprint)
    ├─→ PHASE-2-READER.md (Second sprint)
    ├─→ PHASE-3-POLISH.md (Third sprint)
    └─→ PHASE-4-BACKEND.md (Fourth sprint)

Each Phase links to:
    ├─→ specs/ARCHITECTURE.md
    ├─→ specs/DATABASE.md
    ├─→ specs/API.md
    └─→ specs/UI-DESIGN.md

Development workflow:
    ├─→ DEVELOPMENT-GUIDELINES.md (Setup & workflow)
    ├─→ COPILOT-INSTRUCTIONS.md (Code standards)
    ├─→ COPILOT-WORK-CHECKLIST.md (Pre-implementation)
    └─→ SPEC-TEMPLATE.md (Document format)
```

## 🎯 Success Metrics

Each phase has clear success criteria:
- ✅ All acceptance criteria met
- ✅ 80%+ test coverage
- ✅ Zero clippy/lint warnings
- ✅ All validation checklist items pass
- ✅ Definition of Done verified
- ✅ Ready for next phase

## 📊 Current Status

| Phase | Status | Progress | Next Steps |
|-------|--------|----------|-----------|
| Foundation | Ready for Implementation | 0% | Create Tauri project scaffold |
| Reader | Planned | 0% | Start after Phase 1 complete |
| Polish | Planned | 0% | Start after Phase 2 complete |
| Backend | Planned | 0% | Start after Phase 3 complete |

## 🔍 How to Find Something

**Q: How do I know what to work on?**
A: Start with the current phase spec (e.g., PHASE-1-FOUNDATION.md)

**Q: How do I write code to a spec?**
A: Follow COPILOT-WORK-CHECKLIST.md + COPILOT-INSTRUCTIONS.md

**Q: What are the coding standards?**
A: See COPILOT-INSTRUCTIONS.md

**Q: How do I set up my environment?**
A: See DEVELOPMENT-GUIDELINES.md

**Q: What if a spec is unclear?**
A: Ask for clarification before implementing

**Q: Where's the database schema?**
A: In PHASE-1-FOUNDATION.md (Technical Requirements section)

**Q: What goes in specs/ vs phases/?**
A: phases/ = time-based milestones, specs/ = technical details

**Q: How do I validate my work?**
A: Use the Validation Checklist from the relevant spec

## 📝 Writing New Documents

When creating new specs or phases, use [SPEC-TEMPLATE.md](SPEC-TEMPLATE.md):
1. Copy the template structure
2. Fill in all sections
3. Include acceptance criteria
4. Include validation checklist
5. Include definition of done
6. Add to this index

## 🔐 Quality Gates

All code must pass before merge:
- ✅ Acceptance criteria met (from spec)
- ✅ Validation checklist verified
- ✅ Definition of Done satisfied
- ✅ Tests pass (80%+ coverage)
- ✅ Linting passes (zero warnings)
- ✅ Code review approved
- ✅ Commit message references spec

## 📞 Getting Help

| Question | Resource |
|----------|----------|
| What should I build? | Current phase spec (e.g., PHASE-1-FOUNDATION.md) |
| How should I code? | COPILOT-INSTRUCTIONS.md |
| How do I validate? | Spec's Validation Checklist |
| What about [feature]? | Search this index or current phase |
| Is this done? | Check Definition of Done from spec |

---

**Last Updated**: October 2025
**Maintained by**: Unreal RSS Team
**Total Documents**: 12
**Total Phases**: 4
**MVP Target**: 6 weeks (Phases 1-3)
