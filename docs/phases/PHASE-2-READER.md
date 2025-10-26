# PHASE 2: Reader Experience

**Status**: Planned (Do not start until Phase 1 complete)
**Duration**: 2 weeks
**Reference**: [PHASE-2-READER]

## Overview

Phase 2 transforms the basic feed reader into a beautiful, distraction-free reading experience. This phase focuses on content extraction, typography customization, theming, and refined UI/UX.

**Goals**: Users can read articles with beautiful typography, customizable fonts, dark/light themes, and a distraction-free interface.

## Key Features (Sketch)

- [ ] Content extraction and cleanup (readability)
- [ ] Typography customization (font family, size, line height, spacing)
- [ ] Dark/light/auto theme modes
- [ ] Reading time estimation
- [ ] Distraction-free reading mode
- [ ] Smooth scrolling and pagination
- [ ] Article annotations (highlighting, notes)

## Technical Requirements (Sketch)

**Dependencies to add:**
- `readability-rs` or similar for content extraction
- Additional styling/theme infrastructure

**Database changes:**
- [ ] Settings table for user preferences
- [ ] Articles table additional columns for extracted content

**UI changes:**
- [ ] Reading view redesign
- [ ] Theme toggle
- [ ] Settings panel

## Detailed Specs Required (Before starting)

- [ ] **READER-EXTRACTION.md** - Content extraction and cleanup approach
- [ ] **TYPOGRAPHY.md** - Font system, sizing scale, line-height rules
- [ ] **THEMES.md** - Light/dark/auto theme implementation
- [ ] **READING-VIEW.md** - Distraction-free mode and interactions

## Definition of Done

All specs completed and reviewed before starting implementation.

---

**Next**: Phase 3 - Polish
**Blocked by**: Phase 1 complete
