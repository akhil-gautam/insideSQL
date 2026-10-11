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

- Chapters 2–84, all later fixtures/lab mechanisms, production recovery/resource monitoring and cross-platform packaging remain unimplemented.
- This is a local development bundle; signing/notarization, redistribution notices and narration rights are release gates (docs/LICENSING.md).

## Decisions

- Bundle the existing PostgreSQL 18.3 build for initial macOS testing; do not attach to its running server. Pinned binaries and source licenses must be carried in runtime packaging.
- Preserve this ledger through continuation; check repository state and tests before resuming.
- Keep the full 84-chapter scope; an outline or generated exercise placeholders never count as delivered chapters.
- Design: quiet slate-blue application chrome, white lab surface, deep ink text, cobalt execution paths and violet planner stages. System sans for UI; monospace only for SQL and evidence. A large readable execution journey anchors the lesson, with compact questions beside the working editor.

## Cinematic engine revision — 2026-10-11

- Replaced the six flat boxes with an exploded procedural Three.js machine: SQL token wafers, mounted parser tree, analyzer catalog banks, rewrite gates, physical path-selector pods, and stacked heap pages.
- Theatre frame progression and Anime.js camera easing share the Remotion frame clock. The interactive view offers overview, stage inspection, pause, and reduced motion. Closed replay does not mount a second graphics renderer.
- Sharp DOM annotations: 12px interactive and 20px exported video; every stage also has readable input/output, internal structure and explanation text outside the canvas. A narrow-screen overview uses a steeper diagonal assembly.
- Production TypeScript/Vite build and four frontend tests pass. Six browser checks pass, including all six views, mobile/reduced motion/graphics loss, draft persistence, immediate cancellation, all six real SQL grades, lesson playback/captions, and stable pause using the shared replay player clock.
- Eleven Rust tests pass against the absolute bundled PostgreSQL path. Cancellation is registered before lab preparation and retained across assessment fixtures; duplicate active run identifiers are rejected and registrations clear on all return paths.
- Both chapter and cinematic repeated-frame hashes match. The complete narrated chapter is rendering; the native app has been rebuilt and its 3D parser/planner views and pause controls verified in WebKit. A final rebundle will include the completed video export.
- Fresh scoped finish review found annotation readability, mobile composition, mechanical depth and documentation gaps. All four changes were scored resolved by the fresh reviewer; disposition: ship for those four fixes, with no visible regressions. Video output and broader course surfaces were outside that review. PRODUCT.md, DESIGN.md and the design sidecar now describe the built scoped world.
- Chapter 2 has an unpublished authored draft and six query exercises; access gates and draft persistence are implemented and tested. No additional chapter is counted as delivered until its media and full acceptance gates pass.
