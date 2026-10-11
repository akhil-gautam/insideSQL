# Commerce fixtures and joins review — 2026-10-11

Fresh read-only local review: **clean**, no actionable findings. Reviewed closed fixture selection, visible/check/reset routing, original orders compatibility, cleanup ownership, restricted learner privileges, publication/revision gates, exact-six validation, generated commerce cases, independent baseline answers, all six chapter-three questions/answers/transcript, and selected schema/editor completion. Reviewer inspected source only; parent ran verification. This is a local review, not the approval-blocked external Claude review.

Verified: 15 Rust/core tests against the bundled PostgreSQL 18.3, nine frontend tests, six browser checks, production build and native compilation all pass. The original chapter-one/two files and grading revisions are unchanged. The new chapter remains unpublished.

Fixture evidence: each commerce variant has 52 customers and respectively 310, 390 or 470 orders. Real PostgreSQL tests match every advertised commerce column/type to the actual catalog and prove hot-customer activity, missing relationships, unpaid-only customers, null payment references, duplicate line values, equal captures and multiple children on both sides. Variant fingerprints differ. Reset reconstructs the same visible row counts. Learner INSERT and CREATE TABLE fail.

Assessment evidence: all six reference queries pass three fresh fixture variants. Independent hand-calculated rows in crates/academy-core/tests/fixtures/chapter03-baseline.json verify each answer's small teaching cases. Common incorrect outer-join, nullable NOT IN and raw-child fanout queries fail. Withdrawn preceding chapters block access even when earlier passes exist. A database-creation failure removes its new role while retaining the existing database; authored setup failure removes the new database and role.

New regression tests failed before their fixes: missing chapter-three catalog, missing commerce schema/completion, leaked new role, withdrawn-prerequisite access, and five-question catalog acceptance. All pass after implementation.

Native interface smoke check was blocked because the Mac was locked. The updated native executable compiled with --no-bundle; the currently running app bundle was not replaced. Chapters two/three still lack video/media delivery. Phase 1 Claude source-export approval, other fixture domains/profiles, recovery/resource monitoring and cross-platform checks remain pending.
