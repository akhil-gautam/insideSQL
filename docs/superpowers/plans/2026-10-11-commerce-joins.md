# Commerce fixtures and joins chapter implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan inline.

**Goal:** Author chapter 3 and grade its six real join queries against a richer deterministic commerce fixture.

**Architecture:** Select a closed Rust Fixture enum from authored chapter metadata. Compile fixture SQL and learner schema metadata into the app. Use the selected fixture for visible labs, all grading variants and reset; retain the exact original orders fixture for chapters 1–2. Chapter 3 stays unpublished while media/phase gates are pending.

**Tech Stack:** Rust, PostgreSQL 18, Serde, React, CodeMirror, Vitest.

**Spec:** docs/PLAN.md.

## Global constraints

- 84 authored narrated chapters and 504 working tasks remain the target.
- No customer data, existing server connection, broader learner permissions or external source export.
- Six required exercises per chapter; grade typed results and multiplicity across three fixtures.
- Do not mass-produce videos before Phase 1 acceptance.

## Review focus

- Chapter 3 stays unavailable before publication and all prior passes.
- Visible, assessment and reset labs select the same fixture family.
- Null foreign keys, customers without orders, duplicate line values and multiple captures expose common wrong queries.
- Failed creation must clean up only objects it created.
- Editor completion and schema panels describe the selected fixture rather than always orders.

### Task 1: Author and execute commerce joins

**Files:** Create course/chapter03.json, course/fixtures/orders.sql, commerce.sql, catalog.json, crates/academy-core/src/fixtures.rs; modify runtime.rs, service.rs, lib.rs, web/src/App.tsx, types.ts, SqlEditor.tsx and App.test.tsx.

**Interfaces:** Chapter.fixture defaults to orders; Fixture::Orders and Fixture::Commerce select trusted SQL. Cluster::create_lab receives Fixture and seed 0–2. Course.fixtures supplies table names, typed columns and fixture notes.

- [x] Add failing real PostgreSQL test through the existing Academy API for chapter 3 access, six reference queries and deliberately wrong outer/anti/fanout queries.
- [x] Verify failure against the two-chapter catalog.
- [x] Author chapter 3 transcript, six complete queries/explanations and primary-source references; add deterministic constrained fixtures with invariants.
- [x] Route visible labs, grading and reset through the selected fixture; preserve original orders data and permissions. Clean up partially created owned databases/roles on failure.
- [x] Test selected schema/completion, update UI and run full frontend/core/build checks.
- [x] Fresh local read-only review; fix material findings, record verified evidence and commit.

## Ledger

- Previous goal turn: progress; navigation committed as 3cd8caf, 25 checks passed, native playback and cinematic planner verified.
- Ruling: continue reversible fixtures/content work while Claude source-export approval is pending; chapter 3 does not become a delivered video chapter.
- Preflight callers: ensure_lab, check, reset, runtime credential test; UI schema button, exercise hint and CodeMirror dialect schema. First two chapter revisions/data must remain unchanged.
- Red → green: chapter-three absence, commerce schema/completion, partial-create role leak, unpublished prior chapter access, and exact-six validation. Independent baseline oracle and real schema/distribution tests added.
- Final: 15 core/PostgreSQL, nine frontend and six browser tests pass; production/native compilation pass; fresh local source review clean. No original exercise revisions or first-two-chapter data changes.
- Native UI check blocked by locked Mac; compiled --no-bundle so the current running bundle/resources were preserved. No video chapter was published and no source was exported for Claude review.
