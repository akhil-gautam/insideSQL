# Sort and aggregate implementation plan

> Execute inline with superpowers:executing-plans; one fresh read-only final review.

**Goal:** Author unpublished chapter 10 with six measured sort/aggregate exercises and independently verified hash, memory-sort and disk-sort evidence.

**Architecture:** Reuse closed read-only ScanOrders and the existing runner, measured-query gate, complete report grading and lab lifecycle. Add chapter10.json and a focused real-PostgreSQL test module. Retain React Flow measurements when replacing same-size plans, fixing a regression exposed by browser validation. No fixture, production API, runtime default, library or cinematic scene changes.

**Tech stack:** PostgreSQL 18, Rust/Tokio, existing BigDecimal, existing frontend and native build/check pipelines.

**Spec:** docs/PLAN.md. Full 84 narrated chapters / 504 exercises and all phase/release gates remain required.

## Global constraints

- Each variant contains exactly 100k orders; hot shares remain 65/45/25%. Earlier fixtures/revisions remain unchanged.
- All six exercises require successful Analyze of current SQL. Chapter remains unpublished pending narrated media and acceptance.
- Learner SQL uses SELECT-only roles. Result bounds and existing numeric-compatible assessment stay unchanged.
- Every report returns at most 1,000 rows. No learner node/time gate. Session controls belong only to owned validation labs.
- No mass-produced videos or source export while Phase 1 acceptance and specific Claude export approval remain unresolved.

## Review focus

- Top-N has explicit NULL placement and a unique tie breaker before LIMIT.
- Unknown values count as events but not known-average inputs; all-null groups have zero requested sum and NULL average.
- FILTER changes an aggregate's population without discarding rows needed by sibling measures.
- HAVING qualifies complete paid groups before ordering/limiting, and duplicates remain physical events.
- Grouping sets preserve subtotal identities/masks and null output keys; distinct-value counts do not justify distinct-value money sums.
- Memory experiments change only isolated session settings, retain exact reports and read actual spill evidence rather than fabricate it.

### Task 1: Measured sort/aggregate chapter

**Files:** Create course/chapter10.json and crates/academy-core/src/sort_aggregate_tests.rs. Modify service.rs test registration, README.md, PRODUCT.md and docs/PROGRESS.md. Fix web/src/Results.tsx, retain its unit renderer mock's real state hook in Results.test.tsx, and add the real browser regression to web/e2e/plan-inspector.spec.ts. Record review in docs/reviews/sort-aggregate.md.

**Interfaces:** Chapter10 consumes Fixture::ScanOrders; unchanged RunRequest/run/check/reset/progress and execute/normalized support validation. Tests read raw rows directly through their owned connection; production bounds remain unchanged.

- [x] Write the two real-PG tests first. Build a missing-chapter assertion and six raw-row report oracles using integer counts, BigDecimal sums/rounded known averages, independent sorting and grouping maps.

```rust
let chapter = academy.chapters.get(9).expect("Chapter ten must be authored");
assert!(academy.run(request("10-01", "SELECT 1", "run")).await.is_err());
assert_eq!(actual.column_types, expected.column_types);
assert_eq!(normalized(&actual), normalized(&expected));
```

- [x] Run `cargo test -p academy-core sort_aggregate_tests -- --nocapture`. Expected: both tests fail because chapter ten is missing, rather than SQL/compile errors.
- [x] Author six ordered contracts: paid top25 by amount DESC NULLS LAST/id ASC; cold UTC days with event/known/sum/rounded known average; every active customer with event/paid/known-paid/paid-sum/pending counts; paid groups with >=6 events and >=1500 known sum ranked sum DESC/id ASC top20; hot+cold customer/status detail/customer subtotals/grand total with GROUPING mask; status event/known/distinct-known-value/sum reports.

```sql
SELECT id,total FROM orders WHERE status='paid'
ORDER BY total DESC NULLS LAST,id LIMIT 25;
SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(total),
       COALESCE(SUM(total),0),ROUND(AVG(total),2)
FROM orders WHERE customer_id=777 GROUP BY 1 ORDER BY 1;
SELECT customer_id,COUNT(*),COUNT(*) FILTER(WHERE status='paid'),
       COUNT(total) FILTER(WHERE status='paid'),
       COALESCE(SUM(total) FILTER(WHERE status='paid'),0),
       COUNT(*) FILTER(WHERE status='pending')
FROM orders GROUP BY customer_id ORDER BY customer_id;
SELECT customer_id,COUNT(*),COUNT(total),COALESCE(SUM(total),0)
FROM orders WHERE status='paid' GROUP BY customer_id
HAVING COUNT(*)>=6 AND COALESCE(SUM(total),0)>=1500
ORDER BY 4 DESC,customer_id LIMIT 20;
SELECT GROUPING(customer_id,status),customer_id,status,COUNT(*),COUNT(total),COALESCE(SUM(total),0)
FROM orders WHERE customer_id IN(1,777)
GROUP BY GROUPING SETS((customer_id,status),(customer_id),())
ORDER BY 1,2 NULLS LAST,3 NULLS LAST;
SELECT status,COUNT(*),COUNT(total),COUNT(DISTINCT total),COALESCE(SUM(total),0)
FROM orders GROUP BY status ORDER BY status;
```

- [x] Run the focused tests again. Expected: six exact typed reports match independent raw-row calculations in all three variants, estimates omit actual fields, measured root output matches report grain, references pass Analyze+Check, ten measured wrong reports fail, Reset restores a changed owned baseline and clears proof, and assessment copies are absent after checks.
- [x] In owned test sessions, disable index/bitmap/index-only and parallel paths. Compare the customer conditional report under hash aggregation at 16MB, sorted aggregation at 64kB and at 16MB. Expected: identical reports; observed Hashed/Sorted strategies, Disk/Memory sort-space evidence, and positive temp blocks for the low-memory sort. Restore settings by closing these owned connections; no production defaults change.
- [ ] Run full `cargo test -p academy-core`, `npm test`, `npm run test:e2e`, `npm run build` and `npm run tauri -- build --debug --no-bundle`. Expected: all pass; owned preview processes stop; installed bundle remains intact.
- [ ] Commit the implementation; obtain one fresh read-only reviewer of the task range; fix material findings RED→GREEN and rerun required checks. Commit truthful progress/review notes and keep full-course branch/goal active.

## Ledger

- Previous goal turn: progress. 84f00c0/a0ae45f authored and verified chapter9; 57 checks and builds passed, fresh local review clean. Current worktree is clean. Full goal remains active.
- Ruling: reuse unchanged ScanOrders rather than add a duplicate fixture — sort/aggregate reports need its existing nulls, repeated amounts and 100k populations — richer domains and optional larger profiles remain separate required work.
- Ruling: memory/path controls stay in validation sessions with no time/path grader — tests can prove real evidence without prescribing a planner choice — hardware calibration and app resource guards remain outstanding.
- Ruling: fix disappearing plan nodes discovered by browser validation in the shared inspector — preserve actual React Flow measurements across same-size replacements and selection updates — prior measurements remain until ResizeObserver measures changed-size content; browser coverage must check successive plans and existing estimate/measurement transitions.
- Pre-flight: chapter10 consumes the existing fixture enum/catalog; build discovers the next contiguous file; schema/editor and runner consume unchanged interfaces. No shared-interface conflict found.
