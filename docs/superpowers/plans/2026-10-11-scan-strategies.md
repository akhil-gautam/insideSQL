# Scan strategies and 100k fixture implementation plan

> **For agentic workers:** Use superpowers:executing-plans inline, then one fresh read-only review.

**Goal:** Author chapter 8 with six measured scan investigations and a genuine 100,000-order interactive fixture.

**Architecture:** Add a closed scan_orders fixture with one read-only orders table, fixed UTC timestamps, decimal amounts, nulls, 192-character payloads and primary/customer/time/status B-tree indexes. Three equally sized seeded variants change hot-key proportions and row attributes. Existing Rust execution, measured-query evidence and fresh-variant grading remain unchanged. Independent calculations read raw rows and derive expected reports outside the reference SQL.

**Tech Stack:** Existing Rust/PostgreSQL 18/Serde and frontend fixture catalog; no new dependencies.

**Spec:** docs/PLAN.md.

## Global constraints

- Preserve 84 narrated chapters/504 tasks; chapter 8 is an unpublished draft, not a delivered video.
- Do not change earlier fixture definitions or chapter revisions; existing servers and real customer data remain untouched.
- Exactly 100,000 orders per scan variant; queries return bounded reports through existing result limits.
- Restricted learner roles cannot write, build/drop indexes or alter the schema.
- No fixed physical-node or elapsed-time assessment gates; explain the observed path and its scope.
- No video mass production or source export while Phase 1/Claude acceptance is unresolved.

## Review focus

- Cold customer must contain both known and unknown amounts across all variants.
- UTC half-open time boundaries and descending result order must match exact typed output.
- Broad scan output is a summary, not a claim that one row means little input work.
- Fixture indexes, constraints, row width, distribution and catalog must agree with PostgreSQL.
- Large fixture reset and grading remain isolated, bounded and clean up assessment copies.

### Task 1: Fixture and authored scan investigations

**Files:** Create course/fixtures/scan-orders.sql, course/chapter08.json, crates/academy-core/src/scan_tests.rs; modify fixtures.rs/catalog.json, service.rs test module and delivery notes.

**Interfaces:** Fixture::ScanOrders serializes as scan_orders and remains read-only. SQL is authored setup with only validated seed substitution (0..2). Existing course/run/check/reset/progress commands remain intact.

- [x] Observe unknown scan fixture and missing chapter 8 in focused real-PostgreSQL tests.
- [x] Implement fixture and verify 100k rows, 65%/45%/25% hot-key distributions, nulls, exact numeric/timestamps, fixed payload width, valid indexes, deterministic reconstruction and restricted writes/index creation.
- [x] Author six queries/full transcript; independently derive complete results from raw rows on three variants, measure plans and pass API grading. Reject wrong null handling, time boundary and ordering; prove Reset restores a visible read-only lab.
- [x] Run full core/frontend/browser checks and native compilation; obtain fresh local review, document measured size/resource limits and remaining scope, commit.

## Ledger

- Previous goal turn: progress. 7b66287 authored chapter 7/operation inspector/module navigation, 51 passing checks and native compilation; clean worktree.
- Ruling: fixed 100k rows per variant with varying hot-key fractions — fits the approved interactive profile and tests distribution changes without pretending a larger profile is ready.
- Ruling: payload is a fixed synthetic 192-character text field, not a TOAST demonstration — provides heap width while leaving physical row storage to later chapters.
- Ruling: read raw 100k base rows directly in the independent test oracle; learner UI/assessment output caps remain unchanged — avoids weakening complete-result bounds for production callers.
- Native GUI remains locked; Claude source-export approval remains pending. No new media publication.
- Red/green: unknown fixture and missing chapter failed before implementation; later an explicit all-null-day assertion failed before reserved cold events were added. Seven measured wrong queries now fail assessment, including SUM without the required zero convention. Metadata completion uses a real CodeMirror schema source/cursor rather than assuming a DOM input event moves its selection.
- Verified 32 core, 16 frontend and seven browser checks plus production/native compilation. Local source review and narrow all-null follow-up clean. The concurrent cinematic pass is separately recorded in docs/reviews/cinematic-depth.md.
- Measured final fixture: 26,427,392-byte heap; total relation sizes 32,423,936 / 32,415,744 / 32,399,360 bytes for seeds 0/1/2. Setup plus fixture checks took 954–1,037 ms here; this is machine-specific evidence, not a speed gate. Runtime memory/disk watchdogs and optional 2m/10m profiles remain open.
