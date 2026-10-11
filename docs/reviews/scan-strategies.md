# Scan strategies review — 2026-10-11

Scope: unpublished chapter 8, closed read-only scan_orders fixture, metadata/editor wiring, independent raw-row oracles and real PostgreSQL assessment/reset/permission tests.

Fresh local reviewer found no actionable findings in the initial source review. Parent then added an assertion requiring an all-null daily group; the existing fixture failed it. Reserved ids 97–99 now form customer 777's all-null January 11 UTC group. They remain outside every hot bucket and leave the original time boundaries intact. A seventh wrong report without COALESCE is rejected. Catalog and narration were synchronized. The reviewer inspected this correction and reported a clean follow-up; they did not run tests.

Verified: exactly 100k rows per variant; hot shares 65/45/25%; fixed payload width; exact numeric/UTC metadata; valid primary/customer/time/status indexes; deterministic reconstruction; denied UPDATE/CREATE INDEX/DROP INDEX; six complete typed report oracles across three variants; predicted/measured separation; measured output grain; correct Analyze+Check; seven incorrect reports rejected; reset and grading cleanup. No named-path or elapsed-time assessment gate was added. Frontend test checks payload completion through the actual CodeMirror schema source with an explicit cursor; the existing commerce test retains popup coverage.

Full verification: 32 core/PostgreSQL/catalog tests, 16 frontend checks, seven browser checks, production and native --no-bundle builds pass. This includes the concurrent cinematic depth pass. No chapter-eight video was rendered or published. Native GUI remains locked; specific Claude source-export approval remains pending. This local review does not satisfy the external phase-review gate.
