# First plan investigation implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan inline.

**Goal:** Author chapter 6 and require successful measured execution of the learner's current query before result grading.

**Architecture:** An optional require_analyze exercise flag enables an evidence gate. Rust records server-returned measured plans in additive SQLite storage, keyed by exercise, grader revision and exact submitted SQL. Result correctness still uses fresh variants and typed comparison, never a required SQL spelling or physical algorithm. Reset removes measurement evidence without deleting scripts or earned passes.

**Tech Stack:** Rust, PostgreSQL 18, SQLite/Serde, existing React plan views and Vitest.

**Spec:** docs/PLAN.md.

## Global constraints

- Preserve 84 chapters and 504 exercises, exact-six catalog and all publication/prior-pass gates.
- No universal index/node choice or hardware-independent time threshold.
- Successful Analyze supplies measured evidence; estimated, failed or cancelled runs do not.
- Chapter 6 stays unpublished; no mass video rendering before Phase 1 acceptance.
- No source export or existing database attachment.

## Review focus

- Measurement of another SQL, exercise or revision cannot satisfy the current requirement.
- Attempts still unlock answers even when evidence is missing; old exercises remain compatible.
- Reset invalidates evidence and persistence survives reopening without losing existing progress.
- All six queries need independent result oracles and measured root rows consistent with returned grain.
- Narration must distinguish illustration, estimates, execution and observer overhead without claiming one universal plan.

### Task 1: Authored investigation and persisted measurement gate

**Files:** Create course/chapter06.json and src/plan_tests.rs; modify progress.rs/service.rs, frontend exercise/progress types and notice tests, and delivery notes.

**Interfaces:** Exercise.require_analyze defaults false. Progress records measured JSON and validates revision/current SQL identity. Course progress exposes analyzed status; backend Check enforces it independently of the frontend.

- [x] Watch an existing public API regression accept a required query without Analyze, then implement the gate and persisted evidence.
- [x] Test successful/estimated/failed/cancelled analysis, edits, revision changes, reset, reopening and old-exercise compatibility.
- [x] Author six query exercises and full transcript; verify independent result calculations across every variant and actual measurements without fixed physical-node requirements.
- [x] Add a failing workbench behavior test, then show current measurement status and Analyze-before-Check instructions.
- [x] Run full checks/native compilation, obtain fresh local review, record limitations and commit.

## Ledger

- Previous goal turn: progress; dec8885 committed mutation labs and 41 passing checks. Worktree clean.
- Ruling: SQL identity binds evidence to the query actually measured; it is not comparison with reference SQL and equivalent formulations remain accepted after their own Analyze.
- Preflight: run records evidence after successful measured execution/rollback; check applies gate after attempt unlock; reset removes evidence after successful reconstruction. Existing query/mutation grading and command contracts stay intact.
- Native GUI/Claude source-export approvals remain pending; safe content and lab work continues without new video publication.

- Red: public Check accepted the required query without Analyze; chapter 6 was absent; the workbench measurement notice was absent. Implemented evidence storage/gate and authored chapter. Green: 29 core tests, 11 frontend tests and 6 browser checks (46 total), production/native --no-bundle compilation pass.
- Red: Reset retained the measurement notice; added progress refresh and verified Analyze/Reset/edit lifecycle. CodeMirror input is asynchronous, so its test waits for saved draft state before asserting the notice.
- Ruling: compare measured PostgreSQL JSON row counts numerically, not by integer/decimal JSON spelling — root numbers such as 63.00 are valid — no behavior change to grading.
- Fresh local review: corrected repeated order identity to repeated customer identity for the fifth exercise join; measurement implementation otherwise found consistent. External Claude review still pending specific source-export approval.
- Completion: six reference queries match independent raw-row calculations across all three fixture variants; public Analyze/Check flow passes each. Native GUI remains locked; later media/cross-platform/full-course work remains open. Owned test servers stopped; no existing database or app bundle replaced.
