# Runtime disk resources review — 2026-10-11

Fresh local read-only review covered 00f090c..48ed64e: Unix free-space measurement, owned-runtime spill limits, authorized resource snapshots, the collapsed workbench disclosure and staged Reset. This source review does not satisfy external Claude phase acceptance or native release acceptance.

The reviewer confirmed the six-field restricted snapshot, authorization, real PostgreSQL spill/permission checks, honest resource scopes and stale-response protection. No Critical or Minor findings; two Important findings were retained and fixed in one pass (3811029).

- Reset checked disk space before staging but not after staging consumed space. A behavior-preserving extraction exposed the exact promotion/cleanup boundary to a deterministic real-PostgreSQL regression. After a real replacement was seeded, raising only the test-owned floor reproduced successful promotion when rejection was required. The repair checks space before terminating either database's connections. Rejection cleans staging and preserves the original connection, edited data and role.
- Reset assumed an existing role/database, so a newly unlocked, unvisited later exercise failed before its first Run or resource inspection. The regression first failed with a database error. Reset now uses the shared lazy-creation result: a newly created baseline only needs proof invalidation; existing labs take the staged replacement path. Publication/prior-pass gates, low-space no-allocation behavior and saved scripts remain covered.

Both regressions failed in the observed RED run (six passing, two failing), then all eight focused resource checks passed in 3.14 seconds. Final full core checks pass 44/44 (33 unit, three catalog, six progress and two runtime); frontend 20/20 and browser 10/10 pass, 74 total. Production and native debug --no-bundle builds pass. Only owned previews were stopped, with no preview listeners remaining. No second reviewer was dispatched.

Rulings carried from implementation:

- Disk preflights, a spill allowance and the snapshot are a foundation; calibration, continuous monitoring, heavyweight reservations and memory assessment remain required.
- Reuse installed Rustix for Unix measurement; unsupported platforms fail closed. Windows requires its native adapter and release validation.
- Stage before replacing the current lab to preserve it on setup failure. Peak storage includes another baseline; transactional promotion, cleanup and interrupted recovery require separate evidence.

The reviewer set aside five wider gates. Disposition of every line:

- Windows/cross-platform release remains open; failing closed means Windows cannot initialize until its native measurement is delivered.
- Calibration, disk/memory watchdogs, heavyweight reservations and whole-application memory limits remain open. Aggregate consumption can exceed per-process allowances and preflight margins.
- Crash/unknown-commit recovery and orphan reconciliation remain open; an interrupted promotion may need reconciliation.
- PostgreSQL promotion and SQLite proof invalidation are not coordinated across stores. Recovery when proof invalidation fails remains an explicit release follow-up; such a failure can leave stale measurement proof after Reset.
- Remaining curriculum/media delivery and native GUI acceptance remain incomplete. Local compilation/source review does not prove playback, performance or external phase acceptance.

No deferred minors. Automatic approval review rejected source transfer to Anthropic pending specific approval; no source was sent. Installed bundle and delivered media remain unchanged.
