# Interrupted Reset recovery — 2026-10-11

This increment implements the Reset crash/unknown-outcome part of docs/PLAN.md. It does not complete a course phase or replace external Claude review.

Reset writes an admin-only intent in the owned cluster before staging allocation. PostgreSQL18 [CREATE DATABASE OID](https://www.postgresql.org/docs/18/sql-createdatabase.html) binds that allocation to a reserved exact OID, including a crash before seeding. Both catalog renames and committed=true share the same transaction. Startup and operation-error recovery validate canonical/temporary names, exact OIDs and administrator ownership before removing only the recorded obsolete copy. Missing retired copies are harmless; ambiguous or malformed records retain all copies and refuse recovery. Recovery retains the learner role, drafts/unlocks/passes, and the existing cluster+OID analysis binding.

RED before production edits: after a real worker exit86 and immediate shutdown/restart of its explicitly owned server, the pending reserved row remained1 rather than0. The bundled runtime path was corrected from relative to absolute after an earlier missing-runtime setup failure; that setup error is not RED evidence.

GREEN covers six subprocess states: reservation only; unseeded allocation; seeded staging; an open first rename transaction; committed promotion followed by actual backend self-termination and lost batch response; retired already dropped before journal cleanup. Parent verifies the worker-owned data directory before immediate server shutdown and normal startup. Every state preserves999.99 in the uncommitted lab or restores125.50 after commit, cleans the exact obsolete copy/journal, preserves scripts/passes and retains or invalidates analysis according to actual catalog identity. Repeated restart is idempotent. The ignored worker is explicitly invoked six times by the parent test, not skipped acceptance.

Five corrupt/ambiguous catalog cases refuse Reset/startup before deleting either copy. Unjournaled prefix lookalikes survive startup and a normal Reset. Normal learner connections cannot reach postgres; even after a controlled test-only CONNECT grant, SELECT/DELETE/CREATE in the journal schema fail42501. A retired-template restriction reproduces cleanup failure, retains the new baseline/journal, and allows successful cleanup/retry after the restriction is removed.

A separate real public Reset test uses catalog and row locks to observe promotion waiting at journal UPDATE, terminates only that test-owned backend, and verifies fresh reconciliation preserves original data/identity/draft, removes staging, and allows another Reset. No production failpoint, mocked PostgreSQL behavior or test-only production API was added.

Final pre-review evidence: core/PostgreSQL54/54 (41 unit in293.94s, three catalog, eight progress and two runtime in6.63s), frontend23/23, browser11/11 in39.7s; 88 checks. Production/native debug --no-bundle builds pass. Existing size warning remains. Earlier intermittent graph failures remain unresolved despite this green run. Native GUI was not repeated for this runtime-only increment; previous current-bundle evidence remains separately scoped.

Fresh local review follows the committed complete increment; findings and every disposition will be recorded here. External Claude source-export approval remains pending after automatic approval rejection. No source was sent and no Claude nonresponse was claimed.

Rulings before review:

- Use explicit PostgreSQL18 staging OID to identify allocation before seeding. Runtime/OID collision incompatibility would reject Reset, so actual pinned-runtime tests verify it.
- Commit journal outcome with catalog renames. If atomicity were wrong, recovery could choose the wrong copy; real transactional interruption tests cover it.
- Do not adopt live orphan servers or delete old unjournaled prefixes in this increment. These remain broader required recovery gates; old temporary allocations or a live orphan can still require separate handling.
- Use an absolute bundled-runtime path for Cargo tests. A wrong path would test another runtime or fail setup; all reported database evidence uses the bundled path.

No delivered media, authored fixture or grader revision, native command interface, learner privilege or existing installed bundle/server changes. Full84-chapter/504-exercise, privileged inspections, further resource/recovery work, media delivery and cross-platform acceptance remain active.
