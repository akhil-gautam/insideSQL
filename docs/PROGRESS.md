# Verified progress

Goal: implement the entire docs/PLAN.md and request Claude review after each completed phase.

## Current phase: 1 — verification passed, review pending

- New isolated repository initialized on build/full-course; no existing user work modified.
- Available: Rust 1.94.1, Node 24, PostgreSQL 18.3 (relocatable Postgres.app build), ffmpeg, macOS narration and Claude Code 2.1.296.
- Chapter 1 has its complete narrated video, transcript, timed captions, six SQL questions/reference explanations and packaged native workbench. Final phase acceptance waits for Claude review resolution or recorded nonresponse.

## Verified evidence so far

- Rust: 7 passing tests across progress, typed multiset comparison, real PostgreSQL execution/permissions/grading/reset/reopening/cancellation and credential separation. Database tests run outside the filesystem sandbox because PostgreSQL needs shared memory.
- Frontend: 2 passing behavior tests for collapsed-answer unlocking and independent deterministic Theatre scene seeking.
- Browser integration: all six actual queries passed three fixture variants each; chapter completion survives reload; Explain estimates and Analyze measurements differ; interactive stage selection works.
- TypeScript/Vite production build passes. Current bundle-size warning is recorded for Phase 2 code splitting.
- Native macOS bundle builds from final sources, starts its bundled PostgreSQL, grades actual SQL through Rust commands, unlocks an answer after invalid SQL, resumes saved scripts/passes after reopening, and shuts its cluster down on quit.
- Narration: 416.95 seconds, 110 caption segments whose boundaries come from validated audio clips. Seven-minute introductory chapter is intentionally shorter than deeper chapters; no required topic was removed.
- Remotion: repeated frame 90, after rendering frame 330, produced identical PNG hashes. Evidence is under artifacts/media-verification/.
- Full video: 1920×1080 H.264/AAC, 417.94 seconds, 110 contiguous captions; audio/video durations agree within 0.01 seconds. Browser playback, seeking and active caption cues pass an integration check.
- Packaged native playback initially stalled using the embedded page protocol. Fixed by serving bundled media through Tauri's byte-range-capable asset protocol, scoped only to $RESOURCE/media/**. Native playback advances and displays the complete 6:57 duration.
- Final verification: seven Rust tests using ACADEMY_POSTGRES=src-tauri/resources/postgres, two frontend tests, two browser integration tests, TypeScript/Vite builds and packaged native smoke checks pass.
- Initial shared admin/learner password was caught by a failing regression test, replaced with distinct SHA-256-derived role credentials, and the test now passes.

## Outstanding Phase 1 gates

- Request Claude review, then resolve material findings or record nonresponse.

## Remaining scope

- Chapters 2–84 remain undelivered. Chapters 2–5 have unpublished authored drafts and tested SQL exercises; chapters 3–5 use commerce fixtures. Other fixture domains/profiles, advanced lab mechanisms, production recovery/resource monitoring and cross-platform packaging remain unimplemented.
- This is a local development bundle; signing/notarization, redistribution notices and narration rights are release gates (docs/LICENSING.md).

## Decisions

- Bundle the existing PostgreSQL 18.3 build for initial macOS testing; do not attach to its running server. Pinned binaries and source licenses must be carried in runtime packaging.
- Preserve this ledger through continuation; check repository state and tests before resuming.
- Keep the full 84-chapter scope; an outline or generated exercise placeholders never count as delivered chapters.
- Design: preserve slate-blue application chrome and white SQL lab surfaces. The journey now uses the scoped charcoal/copper mechanical world documented in DESIGN.md, with local Space Grotesk and IBM Plex Mono and detailed 3D internals. Compact questions remain beside the working editor.

## Cinematic engine revision — 2026-10-11

- Replaced the six flat boxes with an exploded procedural Three.js machine: SQL token wafers, mounted parser tree, analyzer catalog banks, rewrite gates, physical path-selector pods, and stacked heap pages.
- Theatre frame progression and Anime.js camera easing share the Remotion frame clock. The interactive view offers overview, stage inspection, pause, and reduced motion. Closed replay does not mount a second graphics renderer.
- Sharp DOM annotations: 12px interactive and 20px exported video; every stage also has readable input/output, internal structure and explanation text outside the canvas. A narrow-screen overview uses a steeper diagonal assembly.
- Production TypeScript/Vite build and four frontend tests pass. Six browser checks pass, including all six views, mobile/reduced motion/graphics loss, draft persistence, immediate cancellation, all six real SQL grades, lesson playback/captions, and stable pause using the shared replay player clock.
- Eleven Rust tests pass against the absolute bundled PostgreSQL path. Cancellation is registered before lab preparation and retained across assessment fixtures; duplicate active run identifiers are rejected and registrations clear on all return paths.
- Both chapter and cinematic repeated-frame hashes match. The complete cinematic chapter is exported: 1920×1080 H.264/AAC, 417.941 seconds, 191,911,787 bytes, synchronized audio/video durations. Browser playback, seeking and all 110 caption cues pass. The final native bundle includes this video; WebKit playback advances past 56 seconds and seeking reaches the planner at 3:28. Native parser/planner inspection and pause controls also pass.
- Fresh scoped finish review found annotation readability, mobile composition, mechanical depth and documentation gaps. All four changes were scored resolved by the fresh reviewer; disposition: ship for those four fixes, with no visible regressions. Video output and broader course surfaces were outside that review. PRODUCT.md, DESIGN.md and the design sidecar now describe the built scoped world.
- Chapter 2 has an unpublished authored draft and six query exercises; access gates and draft persistence are implemented and tested. No additional chapter is counted as delivered until its media and full acceptance gates pass.

## Course foundation continuation — 2026-10-11

- Workbench chapter navigation now consumes Rust's authored catalog. A published chapter opens only after every exercise in all preceding chapters has a current pass; unpublished chapters stay disabled. Chapter 2 is still an unpublished draft.
- Selected chapter controls video/captions, transcript, exercise counters, completion and continuation. Saved SQL restores across chapter/question changes, including deliberately blank drafts. Navigation stays fixed while a query runs.
- Native media_directory now points at the shared media root; the selected chapter identifier is appended by the frontend. Asset access remains scoped to bundled media only. No fixture or progress migration was needed.
- Eight frontend tests, six browser checks and eleven real PostgreSQL/core tests pass. Production and native macOS builds pass. Fresh local code review found no actionable findings (docs/reviews/course-navigation.md). This is foundation progress, not a completed Phase 2 or additional delivered chapter.
- Rebuilt native app restores existing SQL/passes, leaves the chapter-two draft disabled, plays the verified bundled video past 20 seconds with a 6:57 duration, and renders detailed planner inspection in WebKit. Source and bundled cinematic video hashes match.
- The automatic approval review rejected exporting tutorial source to Anthropic for Claude review. Specific destination/data approval remains pending; no source was sent. This is an approval block, not Claude nonresponse.

## Commerce fixtures and joins — 2026-10-11

- Chapter 3 is fully authored as an unpublished draft: 1,553 transcript words in 14 sections, objectives, primary PostgreSQL 18 references, six SQL questions/reference explanations. Topics cover order/customer grain, outer-join match restrictions, existence, line identity, absence with nullable references, and independent fanout-safe totals.
- A closed Rust fixture selector now routes visible labs, every assessment variant and reset to the selected authored SQL. Original chapters 1–2 retain their exact orders fixture and revisions. Commerce uses constrained customers/orders/order_items/payments; fixed UTC data, hot customers, missing matches, repeated line values, equal captures and unattached payments.
- Three commerce variants have 52 customers and 310/390/470 orders. Actual PostgreSQL catalogs match advertised columns/types; invariants and differing fingerprints pass. All six query references pass all three variants and independently match hand-calculated baseline cases. Three common wrong queries fail.
- Schema panel, exercise table hints and CodeMirror completions now use the chapter fixture. Learners retain SELECT-only access. Failed database creation removes its new role; failed authored setup removes its new database and role without dropping a preexisting database. Withdrawn prerequisites lock downstream labs even with saved passes; authored catalogs require exactly six questions.
- Verification: 15 Rust/PostgreSQL tests, nine frontend tests and six browser checks pass. Production and native --no-bundle builds pass. Fresh local review clean (docs/reviews/commerce-joins.md). Native UI recheck is pending because the Mac was locked; the running app bundle was preserved.
- Chapters 2/3 are not counted as delivered video chapters. Full Phase 2 and the original 84-chapter/504-exercise scope remain open. Claude export approval is still pending.

## Grouping and compiled catalog — 2026-10-11

- Chapter 4 is an unpublished authored draft with 1,472 transcript words in 14 sections, objectives, primary PostgreSQL 18 references and six complete SQL questions/reference explanations. It covers paid-customer summaries, all customers with zero activity, HAVING, UTC date/status pairs, deterministic grouped ranking with LIMIT and conditional daily aggregates.
- Independent Rust calculations over raw customer/order rows match every reference on all three commerce variants. They assert tied activity counts and all-null paid amount groups. Reference queries return correctly typed UTC dates even after the test session changes to Asia/Kolkata. Five common wrong queries fail; the measured ranking plan exposes actual rows/loops at its aggregate, sort and limit operations.
- Rust now discovers and embeds the contiguous authored catalog at build time. Empty catalogs, malformed/out-of-range names, gaps, directories masquerading as chapters and symlinks fail discovery. Course changes trigger rebuilds; existing runtime identifier, exact-six and publication validation remain in force. No first-three-chapter revision or fixture changed.
- All chapter-four operations remain locked while unpublished, including Run/Explain/Analyze, Check, solution, save and reset. Verification: 20 core/PostgreSQL/catalog tests, nine frontend tests and six browser checks pass. Production and native --no-bundle builds pass; fresh local source review clean (docs/reviews/grouping-catalog.md).
- No new video was rendered or published. Native UI recheck remains pending because the Mac was locked; the running app bundle was preserved. Specific Claude source-export approval remains pending, with no source exported. The full course goal remains active.

## Writes and mutation labs — 2026-10-11

- Chapter 5 is an unpublished authored draft: 1,475 transcript words in 14 sections, objectives, primary PostgreSQL 18 references and six complete INSERT/UPDATE/DELETE/upsert/writable-CTE exercises. It teaches returned rows versus stored effects, constraints, precise targets, conflict branches and statement/later-read visibility.
- The closed commerce_writes fixture reuses unchanged commerce data and adds a conflict-key example. Orders counts are 311/390/471; order 900000 starts at 25/absent/45. Only orders receives INSERT/UPDATE/DELETE. Other tables, schema ownership, roles and other database connections remain restricted.
- Mutation assessment now executes learner and reference SQL on separate fresh databases and compares both typed output and complete writable-table contents. Both cleanup calls are attempted before errors propagate. State comparison fails closed at its 10,000-row/8 MB bound; larger write profiles still need streaming comparison. Read-only grading is unchanged.
- Run retains successful writes in that exercise only. Mutating Analyze executes inside a rolled-back transaction; Explain does not execute. Check leaves visible data alone, and Reset restores the baseline. The workbench explains these effects for write fixtures.
- Independent imperative calculations verify all six returned relations and final states on every variant (18 fresh fixture/exercise combinations). SELECT spoofing, an extra hidden write with correct RETURNING, overbroad null updates, wrong upserts and base-table change summaries fail. Real errors preserve state for key/status/null constraints and restricted actions. Tests wait for a real post-update PgSleep before cancelling Run/Analyze/Check, then verify rollback and assessment cleanup.
- Red evidence preceded each fix: missing chapter, falsely accepted SELECT insert spoof, persisted Analyze update and missing workbench notice. Final verification: 25 core/PostgreSQL/catalog tests, ten frontend tests and six browser checks pass (41 total). Production and native --no-bundle builds pass; fresh local source review clean (docs/reviews/mutation-labs.md).
- Chapters 2–5 still lack delivered media and remain unpublished. Native GUI recheck remains pending because the Mac was locked; the running bundle was preserved. Specific Claude export approval remains pending and no source was exported. The full 84-chapter/504-exercise objective remains active.

## First plan investigation — 2026-10-11

- Chapter 6 is an unpublished authored draft: 1,462 transcript words in 14 sections, objectives, primary PostgreSQL 18 references and six complete SQL questions. It covers selective filtering, a hot customer population, absence, a scalar UTC summary, all-customer paid summaries and deterministic top-five ranking. Narration distinguishes illustrations, estimates, measured rows/loops/buffers and observer overhead.
- An optional require_analyze flag gates Check on a successful measurement of the current submitted SQL and grader revision. Additive SQLite storage retains measured server JSON and preserves existing progress. Correctness still uses fresh variants and typed results, accepts independently measured equivalent formulations and does not require a named physical node or universal time limit. Attempts unlock answers before this gate.
- Tests verify estimates, invalid SQL and cancellation supply no new evidence; edits/revisions/reset invalidate current proof; reopening preserves it. Reset keeps scripts and earned passes. A real PgSleep is observed before cancellation. The workbench shows current measurement status, clears it on edits and refreshes it after Reset.
- Independent Rust calculations over raw rows match all six references across every commerce variant. Measured root output counts match the required result grain, including the one-row scalar aggregate, 52-customer summary and tied top-five amounts. Every reference passes Analyze followed by the three-variant public Check flow.
- Red evidence preceded implementation: accepted correct query without required measurement, missing chapter, absent notice and stale Reset notice. A fresh local reviewer caught an incorrect repeated-order teaching claim; corrected it to repeated customer identities. Review details: docs/reviews/first-plans.md.
- Verification: 29 core/PostgreSQL/catalog tests, eleven frontend tests and six browser checks pass (46 total); production and native --no-bundle builds pass. Existing cinematic layers, replay pause, local video/captions, real SQL grading, draft persistence and cancellation remain verified. Owned preview servers were stopped; the running native bundle was preserved.
- Chapters 2–6 still lack delivered videos and remain unpublished. Native GUI recheck remains pending because the Mac was locked. Specific Claude source-export approval remains pending, with no source exported. The full 84-chapter/504-exercise goal remains active.

## Plan trees, estimates and operation inspection — 2026-10-11

- Chapter 7 is an unpublished authored draft: 1,467 words in 14 transcript sections, objectives, primary PostgreSQL 18 references and six measured SQL questions. It starts the Plans module with status populations, a cold customer, paid-order/item existence, absence of paid activity, qualified customer groups and combined disjoint order populations.
- Independent Rust calculations from raw customer/order/item rows match every reference on all three commerce variants. Explain contains estimated rows without actual fields; measured root rows match required result grain. Each reference passes public Analyze and three-variant Check. Six measured incorrect reports fail, including item fanout, lost nulls, wrong absence scope, group qualification and branch population errors.
- Per-operation inspection now shows complete reported properties in graph and accessible collapsible tree views. The native selector gives keyboard access to every occurrence; graph clicks select details; new plans reset selection. Estimates and actual values stay separately named, zeros stay visible, and structured worker data remains readable. Raw JSON is unchanged. Child-to-parent arrows use direct upward routes. No invented totals or fixed node/time gates were added.
- Navigation groups the contiguous authored catalog by the approved six-chapter modules, labels chapter 7 Plans and scrolls its sidebar list. Existing publication and all-prior-pass restrictions remain enforced.
- Fresh local review identified that customer 240 could not exercise unknown-total preservation. An added oracle assertion reproduced the gap; the contract/reference/transcript now use cold customer 202 and an incorrect nonnull filter is rejected. The reviewer verified the correction with no outstanding findings. See docs/reviews/plan-trees.md.
- Verification: 30 core/PostgreSQL/catalog tests, fourteen frontend tests and seven browser checks pass (51 total). Production and native --no-bundle builds pass. Real browser evidence covers zero-output operation details, selection, tree/JSON, estimates without actual fields and all existing cinematic/media/SQL flows. Owned preview servers stopped; the running native bundle was preserved.
- Chapters 2–7 remain unpublished and still need delivered videos. Native GUI verification remains blocked by the locked Mac. Specific Claude source-export approval remains pending; no source was exported. Full 84-chapter/504-exercise, fixture, advanced-lab and cross-platform requirements remain open.

## Mechanical depth in every layer — 2026-10-11

- All six shared interactive/Remotion chambers now have bevelled flanges, toothed bezels, bolts, partial jackets and cooling fins. Stage-specific internals add guide rails/contact traces, geared syntax collars, catalog handles/backplanes, rewrite gate teeth, planner cooling ribs/gears, and heap-board connectors/read head.
- Feed wafers, syntax cores, catalog drawers, rewrite gates, selector gears and the executor read head move deterministically from the shared frame. Returning to an earlier frame restores identical transforms. Settled inspection isolates the selected chamber; Overview restores its neighbours and connecting rails. The existing Anime.js camera easing and Theatre progression remain shared with Remotion.
- Verified Overview and all six rendered inspection frames, desktop/mobile browser captures, stable pause pixels for each stage, reduced-motion stillness, replay pause and WebGL fallback. Repeated Remotion frame 90 hashes match after seeking elsewhere. A fresh local source/capture review found no actionable regression (docs/reviews/cinematic-depth.md).
- Current workspace verification: 32 core/PostgreSQL/catalog tests, 16 frontend checks and seven browser checks pass (55 total), including the concurrent scan chapter draft. Production and native --no-bundle builds pass. Native GUI playback/performance remains unverified while the Mac is locked. No delivered lesson video or installed bundle was replaced; production bundle-size warning remains open.
- Claude source export is still blocked by automatic approval review pending specific user approval. No source was sent. This change does not complete Phase 1 acceptance or the full course goal.

## Scan strategies and 100k interactive fixture — 2026-10-11

- Chapter 8 is an unpublished authored draft: 1,523 transcript words in 14 sections, primary PostgreSQL 18 references and six measured SQL exercises. Reports cover point lookup, cold-customer UTC grouping, a half-open time interval, a broad amount summary, hot-customer paid activity and descending primary-key top 20. Narration covers index/heap work, bitmap access, sequential scanning, ordering/early termination, visibility caveats and measurement scope without requiring a named node or speed.
- A new closed, read-only scan_orders fixture contains exactly 100,000 rows per variant. Hot customer 1 owns 65/45/25% respectively. Exact numeric amounts, unknown values, duplicate values, UTC start/end/before events, 192-character synthetic payloads and four valid B-tree indexes are verified against metadata. Learner UPDATE/CREATE INDEX/DROP INDEX are denied. Each exercise retains on-demand private lab creation and existing result bounds.
- Independent imperative calculations over all raw rows match every complete typed reference result on all three distributions. Estimates lack actual fields; measured root rows match the requested grain. All six references pass Analyze then fresh three-variant Check. Seven measured wrong reports fail, including lost unknown events, SUM without zero conversion, both interval boundaries, restricted status population, incorrect count grain and ascending top 20. Reset restores an altered private baseline and clears measured proof; assessment databases are cleaned up.
- An additional all-null-day oracle assertion initially failed. Reserved cold events 97–99 now supply customer 777's all-null January 11 UTC group without changing 100k counts, hot shares or time boundary cases. Catalog/transcript agree. Fresh local review and follow-up are clean (docs/reviews/scan-strategies.md).
- Final measured heap size is 26,427,392 bytes in each variant; total relation sizes are 32,423,936 / 32,415,744 / 32,399,360 bytes. Fixture setup plus checks took 954–1,037 ms on this machine. These figures describe this fixture/run, not universal speed or complete application resource usage. Optional 2m/10m profiles and runtime resource/recovery guards remain outstanding.
- Verification including the cinematic depth pass: 32 core/PostgreSQL/catalog tests, 16 frontend checks and seven browser checks pass (55 total), with production and native --no-bundle compilation. Earlier fixtures and chapter revisions remain unchanged. Chapters 2–8 still need delivered videos and remain unpublished. Native GUI remains locked; specific Claude export approval remains pending. No new media publication or source export. The full 84-chapter/504-exercise goal remains active.


## Join strategies and large relational fixture — 2026-10-11

- Chapter 9 is an unpublished authored draft: 1,515 transcript words in 14 sections, primary PostgreSQL 18 references and six measured SQL exercises. It investigates matching, outer preservation, line existence, nullable-key absence, physical line aggregation and independent child rollups. Nested-loop/hash/merge comparisons preserve the same result in isolated test-owned sessions; learners are not required to obtain a named path or time.
- Closed, read-only join_orders composes the unchanged 100k scan fixture with 1,000 customers, constrained line items/payments, duplicate-valued children, mixed payment statuses and one unassigned captured payment. Hot shares remain 65/45/25%; customers 999/1000 have no orders. Three validated foreign keys and nine valid indexes are verified against live PostgreSQL. DELETE is denied on all four tables, each exercise owns its lab, and reset restores the baseline.
- Independent imperative calculations from all raw base rows match complete reference reports in all three variants, including exact output types. A new type assertion initially reproduced numeric quantity output instead of the declared bigint; references now sum integer quantities to produce bigint. All six Analyze+Check paths pass, seven measured wrong reports fail, and reset clears analysis proof. Estimated plans omit actual fields; measured root rows match report grain. Assessment copies are cleaned up.
- Measured populations: 214,290 / 214,293 / 214,292 lines and 181,825 / 181,826 / 181,824 payments. Total relation sizes: 72,327,168 / 72,318,976 / 72,302,592 bytes. Setup plus fixture checks took 3,372–4,531 ms in this focused run; these are local observations, not speed requirements or complete application resource usage.
- Verification: 34 core/PostgreSQL/catalog tests, 16 frontend checks and seven browser checks pass (57 total); production and native --no-bundle builds pass. One fresh local read-only review found no actionable issues (docs/reviews/join-strategies.md). Exact reference OIDs are verified; existing numeric-compatible learner grading remains unchanged. Owned preview servers stopped; the installed app bundle and delivered media remain unchanged.
- Chapters 2–9 remain unpublished and still need delivered videos. Native GUI remains locked. Automatic approval review still blocks source export to Anthropic pending specific user approval; no source was sent. Larger fixture profiles, full resource/recovery guards, advanced chapters and cross-platform delivery remain open within the full 84-chapter/504-exercise goal.
