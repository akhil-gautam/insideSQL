# Plan trees and estimates implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement inline, then obtain one fresh read-only review.

**Goal:** Author chapter 7 and expose complete plan operation evidence with accessible graph/tree inspection and correct second-module navigation.

**Architecture:** Preserve PostgreSQL JSON as the evidence. Graph and tree share an operation detail renderer; graph selection has a native keyboard-accessible selector. Fixed six-chapter modules follow the approved curriculum. Chapter 7 uses the existing commerce fixture and measured-query gate; independent raw-row calculations verify its six result contracts.

**Tech Stack:** Existing Rust/PostgreSQL, React Flow, React, Vitest and Playwright; no new dependencies.

**Spec:** docs/PLAN.md.

## Global constraints

- Preserve all 84 narrated chapters/504 tasks; authored drafts are not delivered videos.
- Restricted SQL roles, isolated assessment variants, all publication/prior-pass gates remain unchanged.
- Preserve raw plan JSON without deriving fake event traces or imposing fixed physical algorithms/time thresholds.
- No source export or new video publication while Phase 1 acceptance is unresolved.

## Review focus

- Estimates must not appear as actual measurements; zero values and zero loops remain visible.
- Every node and nested field (including workers/subplans) remains accessible; plan changes cannot show stale selection.
- Six-chapter module boundaries must not mislabel chapter 7 or bypass prerequisites.
- Result grain, duplicates, nulls, ties and all-customer preservation need independent calculations across three variants.
- Plan narration must distinguish estimated completion from early termination and per-loop measurements.

### Task 1: Chapter, operation inspection and navigation

**Files:** Create course/chapter07.json, crates/academy-core/src/tree_tests.rs, web/src/Results.test.tsx; modify service.rs test module, Results.tsx, App.tsx/App.test.tsx, style.css and delivery notes.

**Interfaces:** Keep QueryResult/Chapter/command contracts. Operation inspector consumes PlanNode fields verbatim excluding child Plans, recursively handled by the tree. Modules derive from chapter number and six-chapter approved grouping.

- [x] Observe missing chapter, missing operation details and wrong chapter-seven module labels in focused tests.
- [x] Author six measured SQL exercises and full transcript; independently calculate complete results on every fixture variant, run real Explain/Analyze and API grading.
- [x] Implement operation details for graph/tree, safe display of structured fields, estimate/measurement labels, selection reset and native keyboard selector; test zeros/nested workers/new plans.
- [x] Group chapter navigation by approved module and label chapter-seven header correctly; retain unavailable chapter locks.
- [x] Run full Rust/frontend/browser checks, native compilation, fresh local review; record limits and commit.

## Ledger

- Previous goal turn: progress. 88b6e22 authored chapter 6 and measurement gate; 46 checks/builds passed, worktree clean.
- Ruling: show raw per-node properties rather than inventing total rows/timing ratios — preserves observation semantics, including zero loops and parallel worker details.
- Ruling: module membership follows the plan's fixed six-chapter sequence — avoids introducing mutable curriculum metadata and new backend contracts.
- Claude export approval and locked native GUI remain pending; safe content/runtime work continues.

- Red evidence: missing chapter 7 and missing operation selector/details. Navigation fixture initially omitted its fixture key; corrected it and verified the old App's incorrect chapter-seven SQL-journey label before restoring the new module mapping. React Flow test mock was extended for imported enums.
- Content: chapter 7 contains 1,467 words/14 transcript sections and six measured report exercises. Independent raw-row calculations cover all three commerce variants; estimates and actual root grain are checked separately.
- Fresh local review caught customer 240's impossible unknown-total case. Added oracle assertion, observed red, switched the contract/reference/transcript to cold customer 202 and added a rejecting nonnull filter. Reviewer verified the narrow correction; no outstanding findings.
- Visual inspection: top source/bottom target handles give direct upward child-to-parent arrows. Browser checks cover a real zero-output operation, graph selection, accessible selector, tree and unmodified JSON; estimates have no manufactured actual fields.
- Final frontend checks: 14 passed; browser checks: 7 passed; production and native --no-bundle compilation pass. Owned preview servers stopped. Final full core suite: 30 passed (51 total checks); native GUI and external Claude approvals remain unresolved.

- Completion: all six reports match independent results across three variants and pass measured API grading. Six measured wrong reports fail, including the reviewed null-preservation case. Fresh local review has no outstanding findings. Full course remains active; no additional video publication or source export.
