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
