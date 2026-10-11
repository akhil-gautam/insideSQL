# Grouping chapter and compiled catalog implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan inline.

**Goal:** Author chapter 4 with six verified grouping, ordering and limit exercises, and compile authored chapters without a manual source list.

**Architecture:** Discover contiguous chapterNN.json files at Rust build time, embed trusted sources, and retain runtime catalog validation and publication gates. Chapter 4 uses the existing immutable commerce fixture. Independent Rust calculations over raw PostgreSQL rows verify all six reference results on every variant.

**Tech Stack:** Rust standard library, PostgreSQL 18, Serde, existing BigDecimal.

**Spec:** docs/PLAN.md.

## Global constraints

- 84 authored narrated chapters and 504 working exercises remain the target.
- Exactly six exercises per chapter; preserve preceding chapter revisions and fixture data.
- No source export, customer data, broader learner permissions or existing server connection.
- Do not mass-produce videos before Phase 1 acceptance; chapter 4 stays unpublished.

## Review focus

- Malformed names, missing chapter numbers and symlinks must fail a build instead of silently losing content.
- Draft chapters remain unavailable through every backend operation.
- Missing customers, null amounts, tied totals and early limits must not produce plausible wrong answers.
- UTC date grouping and ordered outputs retain their requested types and grain.
- Illustrations and estimated plans must not be presented as measurements or guaranteed execution algorithms.

### Task 1: Compile and verify authored grouping content

**Files:** Create course/chapter04.json, crates/academy-core/build.rs, tests/catalog_build.rs and src/grouping_tests.rs; modify service.rs, progress and review notes.

**Interfaces:** Build output is an embedded &[&str] consumed by Academy::start. Runtime Chapter/Fixture and frontend course interfaces stay unchanged. Raw-row oracle consumes customer identities and order status, numeric amount and UTC day.

- [x] Write and run a failing chapter-four access test and catalog discovery tests.
- [x] Embed valid contiguous authored sources using a standard-library build script.
- [x] Author a complete transcript and six SQL questions/reference explanations, grounded in PostgreSQL 18 documentation.
- [x] Verify each reference independently on all three variants, reject common wrong queries, and inspect actual aggregate/sort/limit measurements.
- [x] Run core/frontend/browser checks and production/native compilation; obtain a fresh local review, record evidence and commit.

## Ledger

- Base: 4d12e83; worktree clean. Cinematic layers/video previously delivered and verified; original full-course goal continues.
- Ruling: continue safe content and pipeline work while specific Claude source-export approval is pending; do not publish drafts or mass-render videos.
- Preflight: build source list feeds runtime validation, then existing navigation/gates. Fixture selection, media paths and first-three-chapter revisions stay unchanged.
- Native UI check remains pending because the Mac was locked; compile without replacing the running bundle.
- Red evidence: discovery test failed because build.rs did not exist; native chapter-access test failed because chapter 4 was absent.
- Green evidence: all six references independently match raw-row calculations on seeds 0–2, including all-null amount groups and ties; five wrong queries fail. UTC/date types remain correct under a changed session timezone. Actual aggregate/sort/limit measurements inspected.
- Final verification: 20 core, nine frontend and six browser tests pass; production/native compilation pass. Fresh local source review clean. Existing chapters/fixtures unchanged. Commit contains no generated media or source export.
