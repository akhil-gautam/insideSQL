# Interrupted Reset recovery implementation plan

> **For agentic workers:** Use superpowers:executing-plans inline; one fresh read-only review follows the complete increment.

**Goal:** Recover an interrupted disposable-lab Reset from durable PostgreSQL evidence while preserving the active lab, SQL drafts, earned passes and restricted permissions.

**Architecture:** Store an admin-only Reset intent in the owned cluster's postgres database before creating staging. Reserve its exact database OID using PostgreSQL 18 CREATE DATABASE OID, so even interruption between allocation and seeding is identifiable. Commit both catalog renames and the journal's committed flag together; reconcile only the recorded names, OIDs and administrator ownership on startup and after operation errors.

**Tech stack:** Existing Rust/Tokio/tokio-postgres/UUID, PostgreSQL18 and SQLite progress. No new dependencies or webview interfaces.

**Review correction:** Fresh local review found historical explicit-OID reuse can resurrect retained proof. The single fix pass adds durable administrator-owned per-lab UUID generations to actual cluster/OID binding, registered on ordinary creation and committed atomically with Reset promotion. Legacy two-field analysis requires Analyze again; scripts and passes remain preserved. This is the ledgered correction to the original OID-only assumption.

**Spec:** docs/PLAN.md, including full84/504 delivery and phase gates.

## Constraints

- Connect only to app/test-owned PostgreSQL; learner queries retain restricted roles.
- No prefix-only deletion, unjournaled orphan deletion, learner-data import, existing-bundle replacement, media production or source export.
- Preserve scripts/passes/unlocks. Existing cluster+database analysis identity rejects proof after a committed replacement; uncommitted Reset keeps the original identity.
- Startup recovery is not orphan-server adoption/watchdog implementation. Tests stop their explicitly owned server after terminating the test app worker.

## Review focus

1. Explicit OID allocation must not weaken current measurement-proof binding; a same-name object with another OID or owner must stop recovery before any deletion.
2. Database allocation is nontransactional: a preseed crash must still have an exact recorded ownership identity.
3. Unknown promotion outcome must be read from the durable transaction, never inferred from a connection error.
4. Cleanup interrupted after dropping retired must be idempotent; preserve the canonical lab and role.
5. Corrupt/unsafe journal identifiers, misleading prefix lookalikes, disk rejection and learner access must fail safely.

### Task 1: Journal and reconcile Reset

Files: crates/academy-core/src/runtime.rs; new crates/academy-core/src/runtime/reset.rs; crates/academy-core/src/service.rs; new crates/academy-core/src/reset_recovery_tests.rs; docs/PROGRESS.md; docs/reviews/reset-recovery.md.

Interfaces: Cluster::replace_lab(database, role, fixture, seed) changes from Result<String> to Result<()> and owns cleanup/reconciliation. Academy::reset awaits replacement then clear_analysis. Cluster startup initializes/reconciles its private journal before returning; normal create_lab stays unchanged except optional internal CREATE DATABASE OID for Reset. Journal fields: database_name TEXT PRIMARY KEY, old_oid OID, new_oid OID UNIQUE, owner_oid OID, replacement_name TEXT UNIQUE, retired_name TEXT UNIQUE, committed BOOLEAN DEFAULT false. Schema academy_runtime is administrator-owned and revoked from PUBLIC.

- [x] Write a real subprocess test over reserved, allocated, staged, open-rename transaction, committed-without-receipt and retired-already-dropped states. Worker exits86 without Drop; parent validates the worker's owned data directory, stops only that server immediately and restarts the app. Assert original999.99 versus baseline125.50 according to commit, no recorded temp databases/journal rows, role retained, drafts/pass retained, original versus changed analysis identity, repeat restart safe. Expected RED: pending journal row survives current startup.
- [x] Implement reserve/stage/promote/reconcile in a focused runtime child module. Reserve safe canonical lab names and generated UUID temporary names with a free OID >=16384; INSERT intent before nontransactional CREATE DATABASE WITH OID. Require exact catalog OIDs/owner and matching journal shape before promotion or cleanup. Use BEGIN plus both ALTER DATABASE RENAMEs and committed=true then COMMIT. Reconnect to read durable outcome on errors; leave ambiguous state intact. Expected GREEN: all six real crash states recover without lost scripts or unsafe deletion.
- [x] Add real regression cases for wrong OID/owner, unsafe journal data, unchanged prefix lookalikes, restricted journal access, rejected setup/promotion and cleanup retry. Expected RED then GREEN at each newly found missing guard; existing disk/identity tests remain passing.
- [x] Run full real PostgreSQL/core, frontend and browser suites, production/native --no-bundle build. Whole-task command: ACADEMY_POSTGRES=/Users/akhilgautam/projects/personal/sql-internals-academy/src-tauri/resources/postgres cargo test -p academy-core -- --test-threads=1. Expected all pass; report any existing intermittent graph failure explicitly. Stop only owned previews.
- [x] Commit; obtain one fresh read-only local review, disposition every finding/ruling, fix material findings once with RED→GREEN evidence and rerun required suites. Record external Claude approval block honestly. Expected clean increment; full course and phase acceptance remain active.

Primary sources: [CREATE DATABASE OID/nontransactional allocation](https://www.postgresql.org/docs/18/sql-createdatabase.html), [ALTER DATABASE](https://www.postgresql.org/docs/18/sql-alterdatabase.html). Actual supported behavior is verified against the pinned bundled runtime.
