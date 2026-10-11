# Course navigation implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan inline. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let the existing workbench display any published chapter and enforce completion of preceding chapters.

**Architecture:** Consume the Rust course catalog and revision-aware progress already returned by the API. Keep PostgreSQL access gates authoritative. Change native media_directory to the shared media root and append the selected chapter identifier in the UI.

**Tech Stack:** React, TypeScript, Vitest, Playwright, Rust/Tauri.

**Spec:** docs/PLAN.md; PRODUCT.md.

## Global constraints

- Preserve all 84 chapters and 504 required exercises; unpublished drafts are not delivered chapters.
- Keep the existing cinematic mechanism and the surrounding workbench design.
- Keep learner SQL restricted and backend chapter gates authoritative.
- Chapter 2 remains unpublished until media and acceptance gates pass.

## Review focus

- An unpublished chapter stays disabled after preceding chapters pass.
- A missing or obsolete prior pass locks every later chapter.
- Switching chapters restores that exercise's saved SQL, including a blank draft.
- A query in flight prevents chapter and exercise changes.
- Native and browser video paths use the selected chapter identifier.

### Task 1: Catalog-driven workbench

**Files:** Modify web/src/App.tsx, web/src/types.ts, src-tauri/src/main.rs, web/e2e/workbench.spec.ts; create web/src/App.test.tsx.

**Interfaces:** Consume Course.chapters: Chapter[] with published: boolean and Progress keyed by exercise identifier. Retain the legacy backend chapter field for existing callers. media_directory becomes the native media root.

- [x] Write App behavior tests using the API boundary: unpublished and incomplete chapter gates; all prior passes unlock published chapters; blank draft restoration; chapter-specific video/transcript/counters; busy navigation.
- [x] Run tests and confirm the old chapter-one-only interface fails the navigation assertions.
- [x] Use selected chapter state, dynamic exercise counts and guarded chapter selection. Update native media root; the existing completion browser assertion still matches the correct unpublished chapter message.
- [x] Run the complete frontend suite, TypeScript/build, real browser integration and Rust/native checks. Inspect the workbench and cinematic view.
- [x] Request a fresh local review, fix material findings, record evidence and commit. Claude export remains pending explicit destination/data approval; this foundation task does not complete Phase 2.

## Execution ledger

- Existing isolation: build/full-course, clean before this task.
- Ruling: proceed inline under the user's standing instruction to build the entire plan; no additional plan approval is needed for this reversible foundation work.
- Preflight: backend already returns chapters and revision-aware passes; media root and UI must change together. No progress migrations or fixture changes are needed.
- Red: three App tests failed because chapter navigation was absent. Fixed jsdom's missing Range geometry in the test environment before checking the final behavior.
- Green: eight frontend tests, six browser checks, eleven real PostgreSQL tests; production and native builds pass. Fresh local review clean; bundle media hash matches.
- Existing production chunk-size warning remains: main chunk approximately 1.34 MB and lazy graphics chunk approximately 554 KB before compression. No additional dependency was added.
- Native smoke check: catalog draft lock, existing progress, local asset video playback beyond 20 seconds and planner inspection verified. Development preview servers shut down cleanly; native app left available to inspect.
