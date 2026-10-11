# Writes and mutation lab implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan inline.

**Goal:** Author chapter 5 with six real mutation exercises and grade both returned data and resulting disposable table state.

**Architecture:** A closed commerce_writes fixture extends commerce with a conflict-key example and grants DML only on orders. Each mutation assessment compares learner and reference executions in separate fresh databases. Analyze executes inside a rolled-back transaction; Run commits writes to the exercise's private lab. Existing read-only fixtures and chapter revisions remain unchanged.

**Tech Stack:** Rust/Tokio/PostgreSQL 18, existing typed result comparator, React/Vitest.

**Spec:** docs/PLAN.md.

## Global constraints

- Preserve the entire 84-chapter/504-exercise course target.
- Exactly six exercises; publication/prior-pass gates and first-attempt answer rules remain enforced.
- Only disposable exercise objects may be mutated; no external data, OS privilege or schema ownership.
- Fresh fixtures for each assessment; do not assume rollback reverses every possible database effect.
- Chapter 5 remains unpublished pending authored media and acceptance gates.

## Review focus

- SELECT spoofing RETURNING must fail without the required durable state.
- Reference execution must start from its own untouched baseline and both databases must be cleaned up.
- Run changes persist in that exercise only; Analyze/cancellation/constraint failures leave starting table state intact.
- Narrow table permissions must not weaken chapters 1–4 or permit other table/schema/role writes.
- Missing conflict rows, null amounts, child references and unchanged rows must be verified independently.

### Task 1: Authored writes and complete state grading

**Files:** Create course/chapter05.json, course/fixtures/commerce-writes.sql and src/mutation_tests.rs; modify fixtures.rs, runtime.rs, service.rs, catalog.json, App.tsx/types.ts/App.test.tsx and delivery notes.

**Interfaces:** Fixture::CommerceWrites exposes a closed writable-table list used for grants, grading and rollback selection. Course fixture metadata advertises those tables to the workbench. QueryResult and learner commands stay unchanged.

- [x] Add real regression tests and watch missing chapter, SELECT spoof and Analyze persistence fail before their corresponding fixes.
- [x] Author chapter 5 and a deterministic constrained write fixture with narrow grants; independently verify all six transformations on every variant.
- [x] Compare outputs and complete writable-table state in separate fresh reference/learner databases; preserve cleanup/cancellation and read-only behavior.
- [x] Roll back mutating Analyze and explain Run/Check/Analyze/Reset behavior in the workbench with a failing behavior test first.
- [x] Run full checks and native compilation, obtain fresh local review, record verified limitations and commit.

## Ledger

- Previous goal turn: progress; e225683 added chapter 4, compiled discovery and 35 verified checks. Worktree clean.
- Ruling: continue reversible authored content and lab mechanisms while specific Claude source-export approval is pending. No additional video is rendered/published.
- Preflight: create_lab serves visible/check/reset tests; fixture selector supplies permissions/state tables. Analyze rollback uses the existing execute/cancel path. Fixture metadata supplies the learner-facing notice; no command contract change.
- Native GUI recheck remains pending while the Mac is locked; compile without replacing the running bundle.
- Red → green: missing chapter 5; SELECT spoof accepted; Analyze retained total 35 instead of starting 25; absent write notice. All now pass.
- Final: six reference outputs/states match independent calculations on every variant, five wrong submissions fail, constraint/permission/cancellation/persistence/reset/isolation checks pass. 25 core + ten frontend + six browser checks pass; production/native compilation pass; fresh local review clean.
- Ruling: keep complete state comparison bounded/fail-closed for current inspection fixtures; streaming comparison is required before larger write profiles ship. No new phase/media delivery or external review is claimed.
