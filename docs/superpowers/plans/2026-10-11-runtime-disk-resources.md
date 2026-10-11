# Runtime disk safeguards and resource snapshot implementation plan

> Execute inline with superpowers:executing-plans; one fresh read-only final review.

**Goal:** Bound temporary spills, reject low-disk allocations/resets without deleting existing labs or scripts, and show actual lab storage and connection settings in the native workbench.

**Architecture:** Stage fresh Reset databases before transactional name promotion; clear measured proof after promotion and then clean the retired database. Reuse the owned Cluster, restricted learner connections, lifecycle locks and API bridges. Rustix is already installed/locked; use its safe Unix statvfs API for bytes available to this process on the managed runtime filesystem. Add one snapshot command and a small collapsed workbench disclosure. No media, fixture, curriculum or grader revisions change.

**Tech stack:** Rust/Tokio/PostgreSQL18, installed Rustix1.1.5 fs, existing React/native details and Tauri/preview bridges.

**Spec:** docs/PLAN.md. All 84 narrated chapters/504 exercises, hardware calibration, ongoing memory/disk watchdogs, heavy-profile estimates/reservations and all-platform release acceptance remain required.

## Global constraints

- Learner SQL and snapshot SQL use restricted roles; administrator credentials and runtime paths never enter the webview.
- PostgreSQL startup sets temp_file_limit=128MB per process; it is not a whole-query or application-wide disk/memory quota and does not cover explicit temporary tables.
- Require at least512 MiB free on the runtime filesystem before initialization, lab creation, execution/check allocation and Reset destruction. This floor is a preflight, not a reservation or a guarantee against later concurrent disk consumption.
- Draft/progress storage and existing lab objects must survive rejected resource preflights. Reset clears measured proof only after successful recreation.
- Keep work_mem=4MB/shared_buffers=64MB and current8s server/10s client limits; no new learner path/time grade.
- Native filesystem measurement supports macOS/Linux first; unsupported platforms fail closed until their native adapter and packaging gates are implemented.
- No source export or additional chapter video publication while specific Claude approval/Phase 1 acceptance remain unresolved.

## Review focus

- Low-space/measurement errors reject before any destructive reset or newly created role/database; scripts and previous progress survive.
- A learner cannot raise temp_file_limit through SET or set_config; a real spill error leaves the session and later queries usable.
- Snapshot output contains only authored storage/settings fields, uses current exercise authorization, and reports settings from its restricted connection.
- Units/scope are honest: available filesystem bytes, database storage excluding WAL/logs/temp, per-operation work memory and per-process spill allowance; no total-memory or hardware-performance guarantee.
- Collapsed/loading/error/reopened and switched-exercise views never show a stale snapshot as current; SQL editing and grading remain usable.

### Task 1: Guard and inspect owned runtime resources

**Files:** Create crates/academy-core/src/resources.rs, resource_tests.rs and web/src/LabResources.tsx, LabResources.test.tsx, web/e2e/resources.spec.ts. Modify core lib.rs/Cargo.toml/Cargo.lock, runtime.rs/service.rs, preview.rs, native main.rs, web types.ts/App.tsx/style.css, README.md/PRODUCT.md/PROGRESS.md. Record review in docs/reviews/runtime-disk-resources.md.

**Interfaces:** resources::available_disk_bytes(&Path)->Result<u64>, MIN_FREE_DISK_BYTES=512*1024*1024; Cluster::require_disk_space()->Result<u64>; Cluster::replace_lab(database,role,fixture,seed)->Result<String> returns the retired database name after successful transactional promotion; Cluster::drop_database(name)->Result<()> cleans only that database; private Cluster::start_with_disk_floor(home,postgres,floor)->Result<Self> for actual no-allocation test, public start always uses the constant; Academy::resources(id:&str)->Result<Value> async under lifecycle read lock; both bridges expose resources{id}; LabResources({id:string}) fetches only while expanded.

- [x] Write and run the existing-runtime regression first: a newly created restricted learner must SHOW temp_file_limit=128MB. Run `cargo test -p academy-core resource_temp_limit -- --nocapture`; expect the current unlimited value to fail this assertion.

```rust
let value:String=learner.query_one("SHOW temp_file_limit",&[]).await.unwrap().get(0);
assert_eq!(value,"128MB");
let denied=learner.batch_execute("SET temp_file_limit='256MB'").await.unwrap_err();
assert_eq!(denied.code().unwrap().code(),"42501");
```

- [x] Add low-disk/start/allocation/reset, restricted snapshot/spill and disclosure tests. Expect missing APIs/component before implementation. Real floor tests use u64::MAX in owned test instances, never fill a filesystem. Verify no new data/credentials/log for rejected initialization, no role/database for rejected creation, and existing reset baseline/scripts/measurement proof preserved.

```rust
let error=Cluster::start_with_disk_floor(home.path(),&runtime,u64::MAX).await.err().unwrap();
assert!(error.contains("Free up disk space"));
assert!(!home.path().join("data").exists());
academy.cluster.minimum_free_disk_bytes=u64::MAX;
assert!(academy.reset("01-01").await.is_err());
assert_eq!(academy.store.script("01-01").unwrap(),Some(saved_sql.into()));
let observed=academy.resources("01-01").await.unwrap();
assert!(observed["database_bytes"].as_u64().unwrap()>0);
assert_eq!(observed["work_mem"],"4MB");
```

- [x] Implement the safe native free-space helper with checked f_bavail*f_frsize; missing/unsupported measurement fails closed. Startup/creation/pre-execution/check/reset checks precede side effects. The test instance's internal floor can be raised; no webview threshold or path input exists. Add the startup spill limit.

```rust
let stat=rustix::fs::statvfs(root).map_err(|e|format!("Could not measure free disk space: {e}"))?;
stat.f_bavail.checked_mul(stat.f_frsize).ok_or_else(||"Free disk-space measurement overflowed".into())
```

- [x] Add a real replacement-setup failure regression by placing a colliding orders table only in the owned template1; failed Reset must leave the edited current database and script accessible, with no staging database. Remove the collision, Reset successfully, and verify a restricted connection reads the restored baseline and no retired database remains. Reuse database creation/seeding separately from role creation; stage with the same exercise role, promote both names in one transaction from postgres, invalidate measured proof after promotion and clean the retired database. Never drop the shared exercise role.

- [x] Implement the authorized snapshot with actual filesystem bytes, pg_database_size(current_database()), current_setting work_mem/shared_buffers/temp_file_limit and the configured floor. Exclude credentials/paths/solutions/assessment identities. Restricted client and fixed authored query only. Include both bridge registrations.

```rust
let row=client.query_one("SELECT pg_database_size(current_database()),current_setting('work_mem'),current_setting('shared_buffers'),current_setting('temp_file_limit')",&[]).await.map_err(|e|e.to_string())?;
```

- [x] Add a native details disclosure beside the SQL workbench, closed initially, with loading/error messages and actual MiB/settings plus explicit scope text. Ignore stale async results after collapse/exercise switch; key by exercise. Errors leave SQL controls usable. Unit tests cover no closed fetch, visible values/scope, error recovery and stale result rejection. Real browser test covers snapshot, collapse/reopen and unchanged SQL execution.

```tsx
useEffect(()=>{if(!open)return;let stale=false;setData(null);setError('');
call<ResourceSnapshot>('resources',{id}).then(v=>{if(!stale)setData(v);}).catch(e=>{if(!stale)setError(String(e));});
return()=>{stale=true;};},[id,open]);
```

- [x] Run focused real-PG/resource/UI checks GREEN. For actual spill cancellation, admin configures only an owned ScanOrders test role with 64kB work_mem/temp allowance and disables competing index/parallel access; restricted sort raises 53400, then SELECT1 succeeds. Normal new connections still show 128MB. SET and set_config limit escalation both fail 42501.
- [x] Run full cargo test -p academy-core, npm test, all browser checks, production and native debug --no-bundle builds. Stop only owned previews; preserve installed bundle/media. Review focus conditions must have observed tests and honest scope.
- [x] Commit implementation and verified evidence; obtain one fresh read-only review of this range. Fix material findings RED→GREEN in one pass, rerun required checks, commit review/progress notes and keep full-course goal active.

## Ledger

- Previous goal turn: progress. 00f090c records chapter10 and graph visibility/deletion fixes; 61 checks and builds passed, local reviewer finding resolved. No active preview sessions remain. Full goal is active.
- Ruling: implement disk preflights/spill allowance/snapshot as the next runtime increment — this advances the binding disk/resource requirements without claiming complete monitoring — hardware calibration, watchdogs, heavy-profile reservations and memory assessment remain required before full acceptance.
- Ruling: reuse safe installed Rustix for Unix measurement and fail closed elsewhere — avoid shell parsing and invented free-space values — Windows requires its native adapter and release validation before delivery.

- Ruling: stage a fresh baseline before Reset removes the current database — replacement seeding or a second preflight can fail — peak disk includes one extra baseline during staging; transactional promotion and cleanup must be verified, and process-crash recovery remains open.
