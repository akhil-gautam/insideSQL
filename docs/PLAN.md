# SQL Internals Academy implementation plan

## Binding product requirements

Build an offline Tauri 2 desktop course with a Rust application core, React webview, real app-managed PostgreSQL 18, Theatre.js scenes and Remotion videos. The learner already knows basic SQL. Teach SQL's journey through the parser, rewriter, planner and executor, then advanced PostgreSQL internals and optimization. Ship 84 narrated chapters, synchronized captions/transcripts, six required SQL exercises per chapter (504 total), rich synthetic fixtures, and source-linked advanced tours. All six exercises must pass to unlock the next chapter. Solutions remain collapsed and unavailable until the first nonempty learner-written Run or Check, including a syntax error; unchanged starters and comments do not count. Allow unlimited retries. Offline progress requires no account.

## Architecture

Rust owns PostgreSQL initialization/start/stop/reset, restricted learner connections, cancellation and resource monitoring, isolated assessment databases, typed multiset comparisons, progress in SQLite, and verified content packs. Administrator credentials never enter the webview. No existing server connections, real dumps, or production actions. Mutation exercises own only disposable exercise objects. Privileged inspection uses narrow authored operations. Advanced sessions use deterministic barriers. Use real PostgreSQL rather than SQL emulation.

React/Vite/TypeScript workbench: navigation, captions/video, interactive mechanism scenes, CodeMirror SQL editor, schema/result panels, React Flow plan explorer and accessible text representation, virtualized results, Radix accessible controls. Rust interfaces: open_exercise, run_sql, check_submission, cancel_run, reset_lab, run_scenario_step, record_attempt, get_solution, get_progress, install_course_pack.

Theatre Studio authors exported state; Theatre Core shares React/SVG scenes with Remotion. Remotion owns frame timing; controlled Theatre drivers avoid independent clocks and shared mutable scene state. Export 1080p English narrated videos with captions/transcripts, usually 10–20 minutes. Interactive replay uses @remotion/player. Assets/fonts/narration/evidence must be local and versioned. Learners need no Node/render tools. Label measured, derived and illustrated information distinctly.

Runtime: pinned complete PostgreSQL 18 build with bin/lib/share, private app directories and authentication, restricted roles, watchdogs, bounded results, disk/resource checks, scripts preserved on recovery. Contrib extensions as needed: pg_stat_statements, pageinspect, pg_visibility, pgstattuple, pg_buffercache, pg_walinspect, amcheck, pg_trgm. Never grant broad inspection/OS privileges. Use fresh cloned fixtures for grading and reset rather than assuming rollback reverses all effects.

Libraries: Tauri, Tokio, tokio-postgres, Serde/serde_json, tracing, rusqlite, rust_decimal, reqwest, hashes/signatures/compressed archives; React, CodeMirror, React Flow/ELK, TanStack Table/Virtual, Radix, Theatre, Remotion; Rust integration tests, Vitest/Testing Library, Playwright and native smoke tests.

## Complete curriculum (six chapters per module)

1. SQL journey: SELECT through the engine; predicates/nulls/types; joins/intermediate results; grouping/sorting/limits; writes/visibility; first EXPLAIN ANALYZE.
2. Plans: trees/estimates; scans; joins; sort/aggregate; loops/timing/buffers; controlled comparisons.
3. Storage: heap pages/line pointers; row headers/layout; TOAST; shared buffers/OS cache; locality/correlation/clustering; visibility/free-space maps.
4. B-tree: search/splits/operator classes; composite indexes/skip scans; partial indexes; expression indexes/casts; covering indexes/heap fetches; order/bitmap/write cost.
5. Access methods: GIN arrays/JSONB; GiST ranges; SP-GiST; BRIN; full-text/trigram; mixed workloads.
6. Transformations: sargability; semi/anti/null; correlated subqueries; LATERAL/top-N; CTE inlining/materialization; sets/outer joins/duplicates.
7. Advanced SQL: fanout-safe aggregates; window frames; ranking/latest/ties; recursion/cycles; temporal/as-of; incremental rollups/materialized views.
8. Planner: catalogs/histograms/MCVs; extended statistics; selectivity/cardinality; paths/costs/join search/GEQO; generic/custom plans; stats/plan regressions.
9. Executor: pull/expression/memory contexts; rescans/Memoize/Materialize; hash batching; merge/incremental sort; memory/spills; parallel/JIT.
10. Concurrency: snapshots/XIDs; isolation; locks; deadlocks/retries; HOT/vacuum/freezing; long transactions/concurrent DDL.
11. Durability: WAL/LSNs/full-page images; commit/fsync; checkpoints/background writer/I/O; physical replication; logical replication/slots; backup/PITR.
12. Scaling: partitions/pruning; partitionwise joins/aggregates; bulk/staging/backfills; concurrent index maintenance; N+1/batching; keyset/large results.
13. Observability: statement/I/O stats; activity/locks/waits; CPU/I/O diagnosis; benchmarks/percentiles; configuration; regressions/read-write-storage tradeoffs.
14. Capstones: multi-tenant latest records; recursive transfers; gated temporal rollups; snapshot-consistent caches; lateral/grouped rollups; unfamiliar incident.

Reference patterns were inspected read-only in i-shares: grant_leg_totals_sql.rb, list_grant_rollup_service.rb and grants/administered_by_company.rb. Re-author exercises with synthetic data; do not copy customer names/data.

## Fixtures and assessment

Seven domains: commerce, SaaS tenants, equity lifecycle, event analytics, search, hierarchies and financial ledgers. Seeded generators, fixed timestamps/UTC, validated constraints/invariants. Largest-table profiles: up to 1k inspection, 100k interactive, 2m optimizer, optional 10m stress. Distributions include hot tenants/keys, correlation, nulls, duplicates, missing relationships, wide rows, stale stats, churn/bloat, varied selectivity, and partition boundaries. Heavy profiles opt-in with disk estimates.

Grade complete typed result multisets (including duplicate multiplicity), order only when specified, exact decimals/time, mutation state, concurrency outcomes, and per-exercise optimization criteria across multiple distributions. No SQL text matching, universal index-plan requirement, or hardware-independent millisecond gate. Reference computations and grading variants are inaccessible to learner database roles; local course is self-assessment, not tamper-proof certification. Progress evidence is keyed by exercise/grader revision; preserve scripts/history when updated exercises need rechecking.

## Phases and completion gates

1. One complete vertical chapter: packaged macOS app manages PostgreSQL, plays narrated Theatre/Remotion video offline, runs real SQL, grades six questions, unlocks solutions correctly and resumes progress. Prove deterministic frames and permissions.
2. Chapters 1–18: runner/reset/cancel, plans, page inspection, first fixture domains, reusable content pipeline.
3. Chapters 19–42: indexes/advanced SQL, distributions, mutation grading, all domains.
4. Chapters 43–60: planner/executor source tours, performance grading, deterministic concurrent sessions.
5. Chapters 61–84: replication/recovery labs, workload experiments, all capstones.
6. Windows/Linux and Intel macOS packaging, runtime and native playback/grading/lifecycle checks.

After every completed phase, attempt Claude review and fix material findings; skip only if unavailable/nonresponsive and record evidence. Do not mass-produce videos before Phase 1 passes.

Acceptance: clean installs/coexistence/conflicts/crash/disk/cleanup; permission boundaries; exact grader semantics/unlocks/gates; meaningful resource assessment; deterministic concurrency/media; keyboard/screen-reader/reduced motion/contrast; all 84 complete videos/transcripts/504 working tasks; interrupted download/update recovery; recorded Remotion license. Defaults: Apple Silicon first; 8GB normal labs/16GB advanced recommendation; no cloud accounts, billing, AI tutor or custom executor tracing.
