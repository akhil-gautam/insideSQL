# Join strategies and large relational fixture

> Execute inline with superpowers:executing-plans; one fresh read-only final review.

**Goal:** Author unpublished chapter 9 with six measured join reports and a read-only 100k-order relational fixture.

**Spec:** docs/PLAN.md; retain all 84 videos/504 exercises and remaining phase gates.

**Architecture:** Add closed Fixture::JoinOrders (`join_orders`) by composing the unchanged scan-orders.sql with new join-orders.sql. The latter creates 1,000 customers, constrained line items/payments, nullable unassigned payments, duplicate-valued child rows, child foreign-key indexes and fresh statistics. Use existing execution, measured-query gate, multiset grading and on-demand lab lifecycle. Production result caps stay unchanged.

**Constraints:** Existing fixtures/revisions remain unchanged. Learner roles get SELECT only. Exactly 100k orders and 65/45/25% hot shares; customers 999/1000 have no orders. Every reference returns at most 1,000 rows. No named algorithm/time grading gate. No media mass production or source export while Phase 1 acceptance is unresolved.

**Review focus:**
- Preserve customer identities despite repeated names and zero activity.
- Count outer matches, not the synthetic row; preserve unknown amounts.
- Existence must not multiply orders with qualifying lines.
- Nullable unassigned captured payments must not defeat absence testing.
- Physical duplicate children count separately; preaggregate independent child relations before combining their measures.
- Match metadata to PostgreSQL types, constraints, indexes, actual row populations and permissions; reset/assessment copies must clean up.

### Task 1: Relational scan fixture and measured join chapter

**Files:** Create course/fixtures/join-orders.sql, course/chapter09.json, crates/academy-core/src/join_tests.rs; update fixtures.rs/catalog.json, service.rs test registration and delivery notes.

**Interfaces:** JoinOrders remains a closed read-only enum variant with validated seed 0..2. Chapter9 has six require_analyze exercises, all unpublished. Existing run/check/reset/progress commands are reused.

- [x] Write two real PostgreSQL tests; observe unknown fixture/missing chapter failures before implementation.
- [x] Compose read-only 100k-order fixture with customers, constrained children, teaching cases and metadata. Verify counts/hot shares/foreign keys/indexes/determinism/permissions on all variants.
- [x] Author six ordered contracts: cold-customer order/name join; all-customer paid summaries; scalar paid orders with qualifying-line existence; hot paid orders without captured payments grouped by UTC date; paid SKU line totals; cold-order independent line/payment rollups.
- [x] Independently derive complete typed results from raw base rows for every variant; verify measured grain, Explain separation, current Analyze+Check, wrong reports, reset and assessment cleanup. Inspect nested-loop/hash/merge alternatives in test-owned sessions only; require no particular strategy from learners.
- [ ] Run focused and full core/frontend/browser suites plus production/native --no-bundle builds; obtain one fresh read-only review, resolve material findings, update notes and commit.

## Ledger

- Previous goal turn: progress. 86ef4f8 completed cinematic depth; 8f483e4 authored scan chapter8/100k fixture. Worktree clean; 55 checks and builds passed. Full goal remains active.
- Ruling: compose unchanged scan orders with new relational tables rather than enlarge the old inspection-commerce fixture — maintains prior lessons while providing real join populations; optional 2m/10m profiles remain outstanding.
- Ruling: derived report outputs remain <=1,000 rows while tests read raw rows directly — preserves production bounds; this does not implement large-result streaming.
- Ruling: algorithm comparisons use isolated test session settings, not a named-node grading rule — verifies teaching alternatives without turning planner choices into correctness requirements; hardware-performance calibration remains separate.
- Pre-flight: fixture enum/source composition supplies chapter9 and create_lab; catalog supplies schema/editor. Both consumers use closed identifiers and existing SELECT-only grants. No production interface change required.
- Native GUI remains locked; Claude source-export approval is pending. No new video publication.
