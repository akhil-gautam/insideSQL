# Interrupted Reset recovery — 2026-10-11

This increment implements the Reset crash/unknown-outcome part of docs/PLAN.md. It does not complete a course phase or replace external Claude review.

Reset writes an admin-only intent in the owned cluster before staging allocation. PostgreSQL18 [CREATE DATABASE OID](https://www.postgresql.org/docs/18/sql-createdatabase.html) binds that allocation to a reserved exact OID, including a crash before seeding. Both catalog renames and committed=true share the same transaction. Startup and operation-error recovery validate canonical/temporary names, exact OIDs and administrator ownership before removing only the recorded obsolete copy. Missing retired copies are harmless; ambiguous or malformed records retain all copies and refuse recovery. Recovery retains the learner role, drafts/unlocks/passes, with measurement binding strengthened through durable generation by the review fix below.

RED before production edits: after a real worker exit86 and immediate shutdown/restart of its explicitly owned server, the pending reserved row remained1 rather than0. The bundled runtime path was corrected from relative to absolute after an earlier missing-runtime setup failure; that setup error is not RED evidence.

GREEN covers six subprocess states: reservation only; unseeded allocation; seeded staging; an open first rename transaction; committed promotion followed by actual backend self-termination and lost batch response; retired already dropped before journal cleanup. Parent verifies the worker-owned data directory before immediate server shutdown and normal startup. Every state preserves999.99 in the uncommitted lab or restores125.50 after commit, cleans the exact obsolete copy/journal, preserves scripts/passes and retains or invalidates analysis according to actual catalog identity. Repeated restart is idempotent. The ignored worker is explicitly invoked six times by the parent test, not skipped acceptance.

Five corrupt/ambiguous catalog cases refuse Reset/startup before deleting either copy. Unjournaled prefix lookalikes survive startup and a normal Reset. Normal learner connections cannot reach postgres; even after a controlled test-only CONNECT grant, SELECT/DELETE/CREATE in the journal schema fail42501. A retired-template restriction reproduces cleanup failure, retains the new baseline/journal, and allows successful cleanup/retry after the restriction is removed.

A separate real public Reset test uses catalog and row locks to observe promotion waiting at journal UPDATE, terminates only that test-owned backend, and verifies fresh reconciliation preserves original data/identity/draft, removes staging, and allows another Reset. No production failpoint, mocked PostgreSQL behavior or test-only production API was added.

Final pre-review evidence: core/PostgreSQL54/54 (41 unit in293.94s, three catalog, eight progress and two runtime in6.63s), frontend23/23, browser11/11 in39.7s; 88 checks. Production/native debug --no-bundle builds pass. Existing size warning remains. Earlier intermittent graph failures remain unresolved despite this green run. Native GUI was not repeated for this runtime-only increment; previous current-bundle evidence remains separately scoped.

The fresh read-only local review (gpt-6-astra/high,848c948..0786626) found one Important issue and no Critical or Minor issues: deleting completed reservations lets random explicit OIDs eventually repeat, and cluster+OID alone could resurrect retained SQLite measurement proof. The reviewer inspected production, tests and callers without modifying the checkout or rerunning suites. Its verdict was ready with fixes.

One fix pass adds administrator-only durable lab generations. Ordinary creation registers a fresh UUID; Reset commits its fresh generation together with both renames and committed=true. Cleanup retains the promoted generation and invalidates metadata before ordinary nontransactional deletion. Startup initializes missing legacy/interrupted lab generations once and preserves matching generations on restart. Eligibility binds the measured connection's actual cluster/OID plus generation; mismatched or unavailable metadata fails closed. Existing two-field analysis requires fresh Analyze, while raw history, scripts, unlocks and passes remain intact. Learners cannot read or modify the generation table, including under a controlled test-only postgres CONNECT grant.

Two service regressions first failed with historical OID reuse and legacy evidence reported as analyzed=true; all five real identity cases then passed. A separate actual production promotion test deterministically forces A→B→A allocation: it failed against pre-fix production with identical full identities, then passed with the generation fix. The forced allocation uses only a test-owned private operation/journal fixture, with no production failpoint or extra public interface. Existing six subprocess crash states verify unchanged generation before commit and rejected former evidence after commit; the committed worker fixture includes the generation in the same transaction. Initial sandbox shmget failures were setup errors, not RED evidence.

Final post-fix evidence: complete core/PostgreSQL57/57 (44 unit in299.68s, three catalog, eight progress and two runtime in6.72s), frontend23/23 in2.29s, browser11/11 in37.1s:91 checks. One ignored worker is explicitly invoked six times by the crash parent. Production and native debug --no-bundle builds pass. The focused Reset set also passes12/12 (11 unit plus one integration). Only the two owned previews were stopped; their listeners are gone. No native GUI repeat, graph repair, mass media production or external export was claimed. Existing size warning and prior intermittent graph observation remain open.

No second review was dispatched: the single material finding is verified through RED→GREEN and the final suites. Deferred minors: none. External Claude source-export approval remains pending after automatic approval rejection. No source was sent and no Claude nonresponse was claimed.

Rulings before review:

- Use explicit PostgreSQL18 staging OID to identify allocation before seeding. Runtime/OID collision incompatibility would reject Reset, so actual pinned-runtime tests verify it.
- Commit journal outcome with catalog renames. If atomicity were wrong, recovery could choose the wrong copy; real transactional interruption tests cover it.
- Do not adopt live orphan servers or delete old unjournaled prefixes in this increment. These remain broader required recovery gates; old temporary allocations or a live orphan can still require separate handling.
- Use an absolute bundled-runtime path for Cargo tests. A wrong path would test another runtime or fail setup; all reported database evidence uses the bundled path.

Final rulings, including every behavior the reviewer declined to judge:

- Use durable generations rather than OID tombstones: tombstones cannot reconstruct deleted history or protect ordinary allocation. If wrong, migration could unnecessarily invalidate analysis or reject Reset; real reuse/restart evidence covers it.
- Keep live orphan-server adoption/watchdogs as a later required gate. Tests stop their owned server first; a crashed app's live server still needs separate recovery.
- Preserve unjournaled historical temporary databases: names do not prove ownership. Old copies can continue consuming disk until separate safe reconciliation.
- Privileged external catalog races remain outside serialized app-owned self-assessment. Restricted learners cannot make those changes; an external administrator could race recovery or reproduce private metadata, so this is not tamper-proof certification.
- Keep native GUI, installed-bundle and cross-platform acceptance separately scoped. Prior native evidence remains valid for that build; these runtime checks/builds could miss a platform-only defect.
- Keep remaining course/media delivery in the active full goal. This increment does not complete84/504 or phase acceptance; calling the whole course delivered would be false.
- External Claude review remains unsatisfied after source-transfer approval rejection. Local review cannot replace it; treating rejection as nonresponse would omit the required gate.

No delivered media, authored fixture or grader revision, native command interface, learner privilege or existing installed bundle/server changes. Full84-chapter/504-exercise, privileged inspections, further resource/recovery work, media delivery and cross-platform acceptance remain active.
