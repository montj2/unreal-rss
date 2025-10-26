# Unreal RSS - Project Overview

## Vision

A lightweight, distraction-free desktop RSS reader built with Rust and Tauri. Unreal RSS prioritizes the **reading experience** above all else, with beautiful typography, content cleanup, and sharing capabilities. Designed as a foundation for future backend sync and mobile applications.

## Why Tauri?

**Tauri** is a lightweight desktop application framework that combines:
- **Frontend**: Web technologies (React/TypeScript) for beautiful, customizable UI
- **Backend**: Rust for performance, system access, and future API development
- **Result**: ~40-60MB footprint (vs 150MB+ Electron), native OS window chrome, full control over typography

For a reading-focused app, Tauri gives us web-grade typography/styling while keeping the app lightweight.

## Core Principles

1. **Reading First** - Typography, spacing, and content clarity are primary concerns
2. **Lightweight** - Minimal resource usage, fast startup
3. **Backend-Ready** - Architecture supports future sync/mobile apps
4. **Offline Capable** - Full functionality without internet
5. **User Control** - Customizable fonts, themes, layout
6. **Enterprise Grade** - Spec-driven development with strong guardrails for AI/Copilot assisted coding

## MVP Feature Set

### Reader Experience
- [ ] Clean article rendering (content extraction/cleanup)
- [ ] Customizable typography (font family, size, line height)
- [ ] Dark/light/auto theme modes
- [ ] Reading time estimation
- [ ] Distraction-free reading mode (hide sidebar, minimal UI)
- [ ] Smooth scrolling and pagination

### Feed Management
- [ ] Add/edit/delete feeds
- [ ] Organize feeds into categories/folders
- [ ] Search feeds and articles
- [ ] OPML import/export
- [ ] Feed auto-refresh (configurable intervals)

### Article Actions
- [ ] Mark read/unread
- [ ] Star/favorite articles
- [ ] Share article (copy link, open in browser)
- [ ] Keyboard shortcuts for navigation
- [ ] Sort by date, title, reading time

### Data & Sync
- [ ] Local SQLite database
- [ ] Article caching for offline reading
- [ ] Export to JSON/OPML
- [ ] No external dependencies (local-first by default)

### Future (Post-MVP)
- [ ] Backend API for multi-device sync
- [ ] Mobile apps (iOS/Android)
- [ ] RSS to Podcast conversion
- [ ] Full-text search with indexing
- [ ] Annotation/highlighting
- [ ] Integration with read-it-later services (Pocket, etc.)

## Technical Architecture

### Stack
- **Desktop**: Tauri (Rust backend + React frontend)
- **Frontend**: React + TypeScript
- **Backend**: Rust (tokio async runtime)
- **Database**: SQLite + rusqlite
- **Feed Parsing**: rss + atom_syndication crates
- **Content Cleanup**: readability-rs or similar
- **Styling**: TailwindCSS

### Project Structure
```
unreal-rss/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── feed.rs         # Feed fetching & parsing
│   │   ├── db.rs           # Database operations
│   │   ├── reader.rs       # Content extraction
│   │   └── api.rs          # Tauri commands
│   └── Cargo.toml
├── src/                    # React frontend
│   ├── components/
│   ├── pages/
│   ├── hooks/
│   ├── styles/
│   └── App.tsx
├── docs/
│   ├── PROJECT-OVERVIEW.md (this file)
│   ├── phases/
│   │   ├── PHASE-1-FOUNDATION.md
│   │   ├── PHASE-2-READER.md
│   │   ├── PHASE-3-POLISH.md
│   │   └── PHASE-4-BACKEND.md
│   ├── specs/
│   │   ├── ARCHITECTURE.md
│   │   ├── DATABASE.md
│   │   ├── API.md
│   │   └── UI-DESIGN.md
│   ├── design/
│   ├── COPILOT-INSTRUCTIONS.md
│   └── DEVELOPMENT-GUIDELINES.md
├── .pre-commit-config.yaml
├── .prettierrc
├── .eslintrc.cjs
├── tauri.conf.json
└── README.md
```

## Development Phases

### Phase 1: Foundation (2 weeks)
- Tauri project scaffold
- Basic feed fetching & parsing
- SQLite schema design
- Simple feed list + article view UI

**Deliverable**: Working feed reader with basic functionality, all guardrails in place

### Phase 2: Reader Experience (2 weeks)
- Content extraction & rendering
- Typography customization
- Dark/light themes
- Keyboard navigation

**Deliverable**: Polished reading experience with full customization

### Phase 3: Polish (2 weeks)
- Share/export features
- Search implementation
- Performance optimization
- Testing & bug fixes

**Deliverable**: Production-ready MVP

### Phase 4: Backend Foundation (TBD)
- API design for future sync
- Authentication scaffold
- Cloud storage integration (optional)

**Deliverable**: Foundation for multi-device support

## Documentation Standards

Each phase and spec includes:
- **Acceptance Criteria** - What "done" looks like
- **Validation Checklist** - How to verify implementation
- **Definition of Done** - Quality gates before merge
- **Copilot Instructions** - Specific guardrails for AI coding

## Code Quality Standards

### Pre-Commit Guardrails
- Rust: `cargo fmt`, `cargo clippy`, tests must pass
- TypeScript: ESLint, Prettier, type checking
- Documentation: All public APIs documented

### Enterprise-Grade Practices
- All commits must reference spec/phase
- Code review required from spec
- 80%+ test coverage for critical paths
- No dependencies added without review
- All breaking changes documented

### Copilot Instructions
See `docs/COPILOT-INSTRUCTIONS.md` for:
- Code style guidelines
- Architecture patterns
- Common pitfalls to avoid
- Testing requirements
- Documentation expectations

## Success Metrics

- **Performance**: App starts in <1 second, article renders in <500ms
- **Memory**: <100MB idle, <200MB with 1000+ articles
- **UX**: Keyboard-first workflow, <5 clicks to read new article
- **Quality**: >80% test coverage, zero clippy warnings
- **Maintainability**: All critical code fully documented

## Non-Goals (MVP)

- Mobile-first design
- Real-time collaboration
- Podcast support
- Social features
- Cloud sync (Phase 2)

---

**Next Steps:**
1. ✅ Define PROJECT-OVERVIEW.md (this document)
2. Set up documentation structure and templates
3. Configure pre-commit hooks and tooling
4. Create detailed Phase 1 spec (PHASE-1-FOUNDATION.md)
5. Begin Phase 1 implementation
