# Measurement lab identity implementation plan

> Execute inline with superpowers:executing-plans; one fresh read-only final review.

**Goal:** Accept and display measured-plan evidence only for the actual current PostgreSQL cluster and exercise database, including after a promoted Reset whose SQLite invalidation fails.

**Architecture:** Read the owned cluster's system identifier once at startup and combine it with the actual database OID. Add a nullable identity to stored analysis without deleting scripts, passes or legacy evidence. Run records the identity of its restricted measured connection; Check and asynchronous Progress inspect current catalog identities under the existing lifecycle lock. Identifiers remain internal to Rust/SQLite.

**Tech stack:** Existing Rust/Tokio/tokio-postgres/rusqlite, PostgreSQL18, current native/preview bridges.

**Spec:** docs/PLAN.md. Preserve all 84 chapters/504 exercises and the complete fixture, media, monitoring, recovery and cross-platform requirements.

## Global constraints

- Learner SQL stays on restricted roles. Administrator SQL is fixed authored cluster/catalog inspection; credentials never enter the webview. Progress/resources payloads exclude identity tokens; deliberate learner SQL can read PostgreSQL's available public metadata, and this remains local self-assessment.
- SQLite migration is additive; old unbound measurements require Analyze again. Scripts, answer unlocks and earned passes remain intact.
- Database identity is the PostgreSQL system identifier plus database OID; an OID alone is insufficient across clusters.
- Run/Check/Progress use existing lifecycle read locks, so Reset cannot replace a database during identity measurement.
- Progress becomes async in both bridges and every Rust caller, without changing its JSON shape or webview command name.
- No source export, new delivered video or installed bundle replacement. General Reset journals/orphan reconciliation, full monitoring/calibration, later chapters/media and native/cross-platform acceptance remain open.

## Review focus

- Failed SQLite invalidation after PostgreSQL promotion must not leave usable or displayed evidence for the former database; a fresh Analyze must restore eligibility.
- Reused database OIDs in a different owned cluster must not accept copied old measurement evidence, while a restart of the same cluster/database retains valid evidence.
- Legacy schema migration and unbound records preserve drafts, pass revisions and solution unlocks; missing identity is never silently attached to a current database.
- Progress inspects known lab names without lazily allocating labs or exposing catalog identities; an unavailable runtime fails the evidence check instead of returning stale success.
- Estimates, invalid/cancelled SQL, edits/revisions, Reset and publication/prior-pass gates keep their existing semantics; native/preview async callers stay synchronized.

### Task 1: Bind measurement storage and every consumer to current lab identity

**Files:** Modify progress.rs, runtime.rs, service.rs, preview.rs, native main.rs, existing Rust progress callers, tests/progress.rs, README.md/PRODUCT.md/docs/PROGRESS.md. Create crates/academy-core/src/analysis_identity_tests.rs and docs/reviews/measurement-lab-identity.md.

**Interfaces:** Progress::record_analysis_for_lab(exercise,revision,sql,plan,identity:&str)->Result<()> and has_analysis_for_lab(exercise,revision,sql,identity:&str)->Result<bool>; existing generic record_analysis/has_analysis retained for raw history/storage consumers. Cluster private system_identifier:String; Cluster::connection_identity(&Client)->Result<String>, Cluster::lab_identities(&[String])->Result<HashMap<String,String>>; Academy::progress()->async Result<Value>.

- [x] Write a real promoted-Reset/SQLite-trigger regression before implementation. Enable measurement for an authorized exercise, Analyze its correct SQL, then install a test-only SQLite BEFORE DELETE trigger raising FAIL on analysis. Reset must return the storage failure after restoring its database; the raw stale row remains but Progress must report analyzed=false and Check must require Analyze, with zero graded variants. Expect the current implementation to report stale analyzed=true.

```rust
sqlite.execute_batch("CREATE TRIGGER fail_analysis_delete BEFORE DELETE ON analysis BEGIN SELECT RAISE(FAIL,'test invalidation failure'); END;").unwrap();
assert!(academy.reset(&exercise.id).await.is_err());
assert!(academy.store.has_analysis(&exercise.id,&exercise.revision,&exercise.solution_sql).unwrap());
assert_eq!(academy.progress().unwrap()[&exercise.id]["analyzed"],false);
```

- [x] Add storage tests for additive migration, identity mismatch and legacy unbound evidence. Add real owned-cluster tests for same-cluster restart retention and different-cluster OID reuse rejection. Expected missing APIs/async signature before implementation; each test names the stale-evidence or history-loss behavior it prevents.

```rust
progress.record_analysis_for_lab("first","v1","SELECT 1",&measured,"123:42").unwrap();
assert!(!progress.has_analysis_for_lab("first","v1","SELECT 1","124:42").unwrap());
assert!(!progress.has_analysis_for_lab("first","v1","SELECT 1","123:43").unwrap());
```

- [x] Add nullable lab_identity TEXT to analysis and migrate four-column legacy tables with PRAGMA table_info plus ALTER TABLE ADD COLUMN. A shared private recording method writes identity or NULL with the existing measured-plan validation and upsert. The bound lookup matches exercise/revision/SQL/identity exactly; legacy recording leaves identity NULL.

```sql
ALTER TABLE analysis ADD COLUMN lab_identity TEXT;
SELECT EXISTS(SELECT 1 FROM analysis WHERE exercise=?1 AND revision=?2 AND sql=?3 AND lab_identity=?4);
```

- [x] Capture actual cluster identity via the owned administrator, then derive current database identities from fixed authored pg_catalog queries. Scope catalog lookup to the caller's generated known lab names; don't create any labs. Run's identity comes from its actual restricted client. Estimate-only or failed/cancelled measurements never get a bound record.

```sql
SELECT system_identifier::text FROM pg_catalog.pg_control_system();
SELECT oid FROM pg_catalog.pg_database WHERE datname=pg_catalog.current_database();
SELECT datname,oid FROM pg_catalog.pg_database WHERE datname=ANY($1);
```

- [x] Change Check to match the current database identity before allocating assessment copies. Change Progress to async, read current identities under the lifecycle read lock, and return false for missing/unbound/mismatched evidence. Synchronize both bridges and all Rust callers; webview JSON stays unchanged. Run focused identity tests GREEN, including failed invalidation, fresh remeasurement, restart retention and equal-OID different-cluster rejection.

```rust
let current=self.cluster.lab_identities(&[db.clone()]).await?;
let measured=current.get(&db).map(|identity|self.store.has_analysis_for_lab(&e.id,&e.revision,&request.sql,identity)).transpose()?.unwrap_or(false);
```

- [ ] Run full core/PostgreSQL, frontend and browser suites plus production/native debug --no-bundle builds. Stop only owned previews. Confirm analysis token isn't returned in progress/resources JSON, missing lazy labs stay absent, and existing cinematic/media/grading flows still pass.
- [ ] Commit implementation/evidence; obtain one fresh read-only source review of this increment. Regrade findings, fix material issues RED→GREEN once and rerun required checks; commit review/progress notes. Keep full goal active.

## Ledger

- Previous goal turn: progress. 802f500 records disk safeguards/staged Reset plus two review repairs;74 checks/builds passed. No owned preview sessions remain. Full goal active.
- Ruling: bind proof to actual cluster and database identity before broader recovery — this closes usable stale evidence even if cross-store invalidation fails — old unbound measurements require Analyze again; general interrupted Reset/orphan recovery remains required.
- Ruling: inspect current identities in asynchronous Progress rather than cache them — a cache can remain stale after a promoted Reset or lost acknowledgement — Progress now needs the owned runtime available and both Rust bridges/callers must await it.
