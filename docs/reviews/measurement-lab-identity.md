# Measurement lab identity review — 2026-10-11

Fresh local read-only source review covered 802f500..9e50428: PostgreSQL cluster/database identity, additive SQLite evidence binding and asynchronous Progress/Check consumers. It does not establish external Claude phase acceptance or native GUI/release acceptance.

The reviewer confirmed actual cluster-system-identifier/database-OID binding, fixed authored catalog queries, existing lifecycle locks, additive history-preserving migration, synchronized bridges and real recovery/restart/equal-OID tests. No Critical or Minor findings. One Important end-to-end finding was retained and fixed in one pass (5200c46).

The backend rejected former-lab evidence after promoted Reset plus SQLite invalidation failure, but the workbench's Reset catch skipped Progress and left the old measured status and plan displayed. Two frontend regressions reproduced retained status. A real-browser regression completed an actual owned Reset, deliberately lost its response and reproduced the former plan remaining visible. A jsdom ResizeObserver adapter supplies only absent layout callbacks; the real plan component remains rendered.

The fix refreshes eligibility after Reset success and error. Inspection confirming current bound evidence preserves it; changed/unverified evidence is cleared. A failed Progress request centrally invalidates displayed measurement flags and results while preserving draft and earned-pass fields. Reset errors remain visible, with inspection errors appended when necessary. App's twelve checks and the focused browser regression pass, including successful inspection, failed inspection, preserved current evidence after resource rejection, unchanged draft/pass display and continued SQL execution. No second review was dispatched.

Initial full verification passes 49 core/PostgreSQL checks (36 unit, three catalog, eight progress and two runtime), 20 frontend and ten browser checks: 79 total, plus production/native debug --no-bundle builds. Final verification after the review fix passes 49 core/PostgreSQL, 23 frontend and eleven browser checks: 83 total. Production/native debug --no-bundle builds also pass; only owned previews were stopped.

Implementation rulings:

- Bind measurement to actual cluster/database identity before broader recovery. Unbound legacy records require Analyze again; general interrupted Reset/orphan recovery remains required.
- Inspect current identities in async Progress instead of caching them. Progress requires an available owned runtime; both Rust bridges and callers await it.
- Identity is an internal evidence binding, not secret metadata or a certification token. Ordinary PostgreSQL metadata is available to deliberate learner SQL; automatic Progress/resources payloads exclude it, and no broad inspection privilege was granted.

Disposition of every behavior the reviewer set aside:

- General interrupted Reset, unknown commits and orphan reconciliation remain open. Identity eligibility does not reconcile interrupted database state.
- Continuous monitoring, resource reservations and hardware calibration remain open. Aggregate consumption remains incompletely bounded by current allowances/preflights.
- Remaining curriculum/media, installed native GUI behavior and cross-platform acceptance remain incomplete. Source inspection, browser checks and compilation cannot prove those deliverables.
- Deliberate local database/progress tampering remains outside the self-assessment contract. A determined learner can edit their local evidence/history; this is not certification.

No deferred minors. PostgreSQL's cluster control metadata is documented in [PostgreSQL 18 system information](https://www.postgresql.org/docs/18/functions-info.html#FUNCTIONS-CONTROLDATA); the additive column migration follows [SQLite ALTER TABLE](https://sqlite.org/lang_altertable.html). Actual identities, permissions and migration behavior were also verified against the owned runtimes.

Automatic approval review previously rejected Anthropic source transfer pending specific approval; no source was sent. After unlock, the existing installed native bundle renders the overview/six layers and responds to pause/resume; replay remained at 0:00 and local video remained blank. Bundled/source media hashes match. Native playback/performance and current-binary GUI acceptance remain unresolved. Installed bundle and delivered media remain unchanged. Full 84-chapter/504-exercise scope remains active.
