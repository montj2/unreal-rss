# Copilot Session Primer

**Purpose**: Get Copilot primed and ready at the start of each development session.

**When to Use**: Start of every new development session (morning, after break, context switch).

**Expected Duration**: 2-3 minutes to paste all prompts and get responses.

**Outcome**: Copilot has full context of project, current phase, standards, and immediate work tasks.

---

## Session Startup Checklist

- [ ] Open VS Code workspace: `/home/montj2/unreal-rss`
- [ ] Open terminal in workspace root
- [ ] Verify git status is clean: `git status`
- [ ] Verify current branch: `git branch`
- [ ] Follow prompts below in order

---

## PROMPT 1: Project Context (Copy & Paste)

```
I'm working on Unreal RSS, a Tauri-based desktop RSS reader.

Please read these files to understand the project:
1. /home/montj2/unreal-rss/README-SPECIFICATION.md (overview)
2. /home/montj2/unreal-rss/docs/PROJECT-OVERVIEW.md (vision)
3. /home/montj2/unreal-rss/.github/copilot-instructions.md (your guardrails)

Once you've read them, summarize back to me:
- Current project status
- Technology stack
- MVP scope
- Your key constraints as AI assistant
```

**What to Expect**: Copilot will summarize the project, confirm it understands scope boundaries, and acknowledge the guardrails.

**Validation**: Copilot should mention:
- Tauri + Rust backend + React frontend
- SQLite database
- MVP = add feeds, read articles, mark read/starred
- Phase boundaries (Phase 2+ out of scope)
- Spec-first development, test-driven, no scope creep

---

## PROMPT 2: Current Phase Context (Copy & Paste)

```
What phase are we in currently?
- If we're NOT in Phase 1 yet (still in setup): Read /home/montj2/unreal-rss/docs/phases/PHASE-1-INFRASTRUCTURE.md
- If we're in Phase 1 feature work: Read /home/montj2/unreal-rss/docs/phases/PHASE-1-FOUNDATION.md

Read the relevant phase document, then tell me:
1. Current phase and what we're building
2. Today's specific deliverables
3. The acceptance criteria I need to meet
4. The validation checklist you'll use to verify completeness
5. What to watch out for (anti-patterns, gotchas)
```

**What to Expect**: Copilot will confirm current phase and summarize what you should be working on today.

**Validation**: Copilot should:
- Reference specific acceptance criteria
- Mention the validation checklist
- Warn about specific pitfalls (e.g., "don't skip pre-commit setup")
- Provide clear success definition

---

## PROMPT 3: Today's Specific Task (Copy & Paste)

*Replace `[TASK_DESCRIPTION]` with your actual task*

```
Here's my task for today:

[TASK_DESCRIPTION]

For this task, I need you to:

1. **Confirm this is in-scope**
   - Is this part of the current phase?
   - Does it match accepted acceptance criteria?
   - Should I proceed or defer?

2. **Point me to relevant specs**
   - Which docs should I read first?
   - Any database schema I need to understand?
   - Any API contracts I need to follow?

3. **Outline the implementation approach**
   - What tests should I write first? (TDD)
   - What error cases do I need to handle?
   - Any guardrails I should follow for this type of change?

4. **Provide your validation criteria**
   - How will you know this task is done?
   - What validation checklist should I use?
   - What metrics prove success?

Do not write code yet. Just plan and prepare me.
```

**What to Expect**: Copilot confirms the task is in-scope and provides a detailed implementation plan before you write a single line of code.

**Validation**: Before you start coding, Copilot should have:
- ✅ Confirmed the task is in-scope
- ✅ Pointed you to all relevant specs
- ✅ Outlined the implementation approach
- ✅ Provided clear validation criteria

---

## PROMPT 4: Code Review Readiness (Use Before Commits)

*Use this when you think a task is done and want Copilot to review*

```
I think my implementation is complete. Before I commit, please review:

1. **Does my code match the spec?**
   - Read the relevant section of /home/montj2/unreal-rss/docs/phases/PHASE-1-FOUNDATION.md (or current phase)
   - Verify each acceptance criterion
   - Are there any I missed?

2. **Are all guardrails followed?**
   - Check /home/montj2/unreal-rss/.github/copilot-instructions.md
   - Verify error handling (no unwrap!)
   - Verify testing (TDD approach)
   - Verify documentation (doc comments)

3. **Validation checklist**
   - Use the validation checklist from the current phase spec
   - Go through each item
   - Mark what passes and what fails

4. **Green-light decision**
   - Should I commit this?
   - Any changes needed before merge?
   - Any risks or gotchas?

Please review my code and indicate if it's ready to commit.
```

**What to Expect**: Copilot will do a thorough spec-based code review and tell you if it's ready to commit or what needs fixing.

---

## PROMPT 5: When You Get Stuck (Copy & Paste)

```
I'm stuck on [DESCRIBE_PROBLEM].

Please help me by:

1. **Confirm the requirement**
   - What does the spec say about this?
   - Read: [RELEVANT_SPEC_FILE]
   - What exactly should I be building?

2. **Suggest an approach**
   - Should I handle this in Rust or React?
   - What pattern from /home/montj2/unreal-rss/.github/copilot-instructions.md applies?
   - Any edge cases I should consider?

3. **Provide error handling guidance**
   - What errors can occur?
   - How should I handle each one?
   - What should the user see if something fails?

4. **Point me to examples**
   - Similar code elsewhere in the project?
   - Example from the spec that illustrates this?
   - Test cases I should read?

Help me think through this, but don't write the solution yet.
```

**What to Expect**: Copilot will help you think through the problem without just giving you code.

---

## Quick Commands for Terminal

Once Copilot has your context, here are the commands you'll need:

```bash
# Start development server (after Tauri scaffold)
npm run dev

# Run format checks (pre-commit does this, but you can check manually)
cargo fmt --check
cargo clippy -- -D warnings
npm run lint
npm run format --check

# Run tests
cargo test
npm run test

# Create a new branch for your work
git checkout -b feature/your-feature-name

# After you're done, commit with proper format
git commit -m "feat(scope): description [PHASE-1-FOUNDATION]"

# Push and create PR
git push -u origin feature/your-feature-name
```

---

## Session Ending Checklist

Before you close VS Code:

- [ ] All code is committed (no uncommitted changes)
- [ ] All commits reference the current phase: `[PHASE-1-FOUNDATION]` etc.
- [ ] All pre-commit checks pass: `cargo fmt`, `cargo clippy`, `npm run lint`
- [ ] All tests pass: `cargo test`, `npm run test`
- [ ] Git history is clean: `git log -1 --oneline` shows your commit with spec reference
- [ ] Next developer can pick up from clean state

---

## Example Session Flow

### 9:00 AM - Start Session

```bash
# Terminal
$ git status
On branch feature/feed-parsing
nothing to commit, working tree clean

# VS Code
# Paste PROMPT 1 → Copilot summarizes project
# Paste PROMPT 2 → Copilot confirms we're in Phase 1 Foundation, Days 4-5 (feed parsing)
# Paste PROMPT 3 with task: "Add RSS 2.0 feed parser supporting <title>, <description>, <link>, <pubDate>"
```

Copilot tells you:
- This is in-scope (Phase 1, Days 4-5)
- Read: specs/API.md (add_feed command needs to parse feeds)
- TDD approach: Write parser tests first, then implement
- Validation: Parser handles malformed XML gracefully, returns Result type

### 9:15 AM - Write Code

You write RSS parser tests first (TDD), then implementation. Every 1-2 hours, paste PROMPT 4 to get a spec-based review.

### 5:00 PM - End Session

```bash
$ git log -3 --oneline
a1b2c3d feat(backend): add RSS 2.0 feed parser [PHASE-1-FOUNDATION]
f4e5d6c feat(backend): add feed URL validation [PHASE-1-FOUNDATION]
g7h8i9j feat(setup): initialize cargo project [PHASE-1-INFRASTRUCTURE]

$ cargo test
   Compiling unreal-rss v0.1.0
    Finished test [uninitialized] target(s) in 0.23s
     running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured

$ npm run lint
✓ All checks passed
```

Commit and close. Next developer picks up clean state.

---

## When to Re-Prime

Re-run the session primer prompts if:

- ✅ Starting a new development day
- ✅ Coming back from a long break (>1 hour)
- ✅ Switching to a different phase
- ✅ Switching to a different component (feed management → article reading)
- ✅ Confused about what you should be working on
- ✅ Need a fresh validation checklist

**Do NOT re-prime if**: You're in the middle of implementing a feature and just need tactical help.

---

## Key Principles

1. **Spec First**: Always read the spec before asking Copilot for code
2. **Plan Before Code**: Get Copilot's input on approach before writing
3. **Validation Mindset**: Use Copilot to validate spec compliance before committing
4. **Clean Boundaries**: Stay within current phase scope - Phase 2+ is off-limits
5. **Error Handling**: Every prompt reminds about error handling and guardrails

---

## Troubleshooting Session Priming

### "Copilot doesn't remember the project"
→ Paste PROMPT 1 again. Copilot context may have reset.

### "Copilot suggests out-of-scope features"
→ Remind it: "This is Phase 1 Foundation only. Phase 2+ features are out of scope. What should Phase 2 be for?"

### "Copilot suggests using `unwrap()` or ignoring errors"
→ Say: "Remember, no `unwrap()` in production. How should we handle this error properly?"

### "I'm not sure if a task is in-scope"
→ Paste PROMPT 3 with your task description. Copilot will confirm.

### "Copilot is writing too much code too fast"
→ Say: "Slow down. I want you to help me think, not write the solution. What should I do first?"

---

## Success Metrics

You know the session primer is working if:

✅ You start coding only after reviewing the spec
✅ You write tests before implementation (TDD)
✅ Your commits reference the current phase: `[PHASE-1-FOUNDATION]`
✅ All code passes `cargo fmt`, `cargo clippy`, `npm run lint`
✅ All tests pass before committing
✅ Your git log is clean and tells the story of what you built
✅ Scope creep is zero (Phase 2+ features deferred)
✅ Code review with Copilot takes <5 minutes because spec was followed

---

**Remember**: The session primer isn't about speed. It's about **quality, scope control, and sustainable development**. Taking 3 minutes to prime Copilot at the start saves hours of rework later.

**Next Session?** Just paste the prompts above. Copilot will remind you of all the standards and validate your work.
