# Runtime workspace ownership implementation plan

> **For agentic workers:** Use superpowers:executing-plans inline; obtain one fresh read-only review after the committed increment.

**Goal:** Prevent a rejected second startup from modifying or stopping the database owned by the first app, including a crashed/older app without an application lock.

**Architecture:** Acquire a nonblocking exclusive standard-library file lock in the canonical private runtime directory before credentials, initialization, logs or child launch. Hold its descriptor with the owned PostgreSQL child and release only after that child's shutdown. Refuse a pre-existing postmaster.pid without adopting, connecting, signalling or deleting it. A stopped old Cluster object must never stop a newly started instance.

**Diagnosis correction:** A real replaced-marker regression also proves pg_ctl directory cleanup can stop another test-owned server after our own Child has exited. Stop must first reap/check the exact owned child and return successfully if it has already exited, without signalling any PID from the directory. This closes the corresponding failed-child path, not just the startup guard.

**Live-child correction:** Replaced metadata can also redirect directory-based shutdown while the actual owned child remains alive. Signal only the retained Child PID via bundled pg_ctl kill INT, poll/reap that exact child for the existing five-second allowance, and keep the own-child hard-stop fallback. Do not use directory PID metadata as shutdown authority.

**Tech stack:** Existing Rust1.94.1, std::fs::File locking (stable1.89), Tokio and bundled PostgreSQL18. No new dependency or learner-facing command.

**Spec:** docs/PLAN.md startup/conflicts/coexistence/crash/cleanup acceptance.

## Constraints and rulings to carry

- Test only explicitly created temporary clusters. Preserve installed apps, existing servers, credentials, media, scripts, passes and fixtures.
- This protects ownership; live orphan adoption and safe stale-marker recovery remain required later gates. Never infer authorization from a PID or prefix.
- Full approved-build authorization covers this bounded acceptance fix; no repeated design approval or phase-completion claim. Mass video production and external Claude source transfer remain gated.

## Review focus

1. The lock must precede shared-state mutation and last through child shutdown; canonical aliases cannot bypass it.
2. Rejected startup must not truncate logs, change credentials/catalogs or signal the first server.
3. Older/crashed owners without a lock must also survive rejected startup; do not connect to an existing server.
4. Explicit stop must permit reopen before the old Rust object drops; old Drop must not stop the successor.
5. Failed startup, process exit and unusable lock paths must fail safely without leaving a permanent logical lock. Conservative stale-marker refusal must be reported honestly.

### Task 1: Guard runtime directory ownership

Files: crates/academy-core/src/runtime.rs; new crates/academy-core/src/runtime/ownership_tests.rs; docs/PROGRESS.md; docs/reviews/runtime-ownership.md.

Interfaces: Cluster::start/start_with_disk_floor and stop/Drop retain signatures. Internal child ownership retains both Child and File. Sole child field references are constructor and stop; service/native/preview callers remain unchanged.

- [x] Add real PostgreSQL duplicate-start regression preserving first PID, live learner connection, edited999.99 data, credentials and log prefix. Expected RED: first connection/server is stopped after rejected startup.
- [x] Add canonical-alias, independent-directory coexistence, explicit stop/reopen/old Drop, real exit86 live-orphan and conservative marker/lock failure cases. Implement the lock and pre-existing marker guard at the narrow startup boundary; retain it with child lifetime. Expected GREEN: real conflicts refuse before touching the owner and independent instances remain usable; no orphan adoption claim.
- [x] Run frontend/browser suites and production/native debug no-bundle build; run the complete bundled PostgreSQL/core suite as the postcommit task-done verification below. Whole-task command: env ACADEMY_POSTGRES=/Users/akhilgautam/projects/personal/sql-internals-academy/src-tauri/resources/postgres cargo test -p academy-core -- --test-threads=1. Expected all pass; preserve earlier graph/size warnings and separately scoped native/platform evidence.
- [x] Commit; record task-done full test result; obtain one fresh read-only review, disposition every finding and declined behavior. Fix material findings once with RED→GREEN and green suites, defer Minors. Expected reviewed increment with full course goal active and external Claude block accurately retained.

Primary sources: [Rust File::try_lock](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock), [PostgreSQL18 pg_ctl kill and directory-based stop](https://www.postgresql.org/docs/18/app-pg-ctl.html). Actual macOS lock/child behavior must be demonstrated by the tests.

Completion:63f423d verified by postcommit task-done (64 core),23 frontend,11 final uninstrumented browser, native builds. Fresh local review reports no actionable findings; every declined behavior is dispositioned in docs/reviews/runtime-ownership.md. Earlier graph failures and specific Claude approval remain open. New direct user authorization changes publication handling: push source and final docs to akhil-gautam/insideSQL main; retain local branch, do not merge/force/create a PR.
