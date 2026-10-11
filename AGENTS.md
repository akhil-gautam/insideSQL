# insideSQL — project guide

## The app

insideSQL is an offline desktop course for developers who already know basic SQL and want to understand PostgreSQL execution and optimization. Start with SQL statements and their journey through the engine; do not spend the introductory chapter defining databases or tuples.

The approved target is **84 narrated chapters and 504 required SQL exercises**, six per chapter. Each lesson combines a cinematic engine explanation with real PostgreSQL practice. Solutions stay collapsed and locked until the first nonempty learner-written Run or Check; syntax errors count, empty SQL/comments/unchanged starters do not. All six exercises must pass, and the next lesson must be published, before advancing. Keep unlimited retries and local drafts/progress.

## Sources of truth

- `docs/PLAN.md`: binding product scope, curriculum, architecture and phase gates. Preserve the whole approved scope.
- `docs/PROGRESS.md`: dated evidence, completed increments and unresolved work. Later entries supersede older observations.
- `docs/reviews/`: review findings, dispositions and verification limits.
- `DESIGN.md`: visual direction and scene/workbench conventions.
- `docs/LICENSING.md`: dependency and distribution obligations.

Never count an outline as an authored chapter, or an unpublished draft as delivered. Update progress when behavior or delivery status changes.

## Stack and layout

| Location | Responsibility |
| --- | --- |
| `crates/academy-core/` | Rust service, PostgreSQL runtime/reset, fixtures, grading, cancellation, resources and SQLite progress |
| `src-tauri/` | Tauri 2 desktop bridge, application policy and packaging |
| `web/src/` | React 19/TypeScript/Vite workbench; CodeMirror editor, React Flow plans, TanStack results and Radix controls |
| `web/src/DatabaseDiagram.tsx`, `engineMotion.ts`, `journey.ts`, `QueryJourney.tsx` | SVG database structures, Anime.js easing and Theatre progression |
| `web/src/video/` | Remotion compositions and shared replay/lesson scenes |
| `course/chapterNN.json` | Authored contiguous chapter catalog, exercises, references and transcripts |
| `course/fixtures/`, `course/theatre/` | Synthetic SQL fixtures and exported Theatre state |
| `scripts/` | PostgreSQL bundling, local narration and media rendering |
| `web/e2e/`, Rust test modules | Browser behaviors and real database regression coverage |

The Rust core uses Tokio, tokio-postgres, rusqlite, Serde and exact numeric handling. Current development uses PostgreSQL 18.3, Rust 1.94.1 and Node 24 on Apple Silicon macOS. Native commands call Rust directly; the Axum preview bridge is only for local browser development.

## Current progress — October 11, 2026

- Chapter 1 is delivered with a narrated 1080p lesson, captions/transcript and six working exercises.
- Chapters 2–10 are authored, with 54 more exercises, but unpublished and unavailable to learners. Chapters 11–84 still need authoring; 83 chapter videos remain.
- The journey has six inspectable SVG database diagrams, shared deterministic replay, pause and reduced-motion handling. Paper/green styling joins the prompt, editor, actions and results; narrow screens put practice first.
- Real SQL, typed grading across fixture variants, write-state assessment, cancellation, draft persistence, plan inspection and lab-resource snapshots work.
- Reset has durable interruption reconciliation and generation-bound measurement evidence. Runtime ownership rejects competing launches and shutdown targets the exact owned child.
- Latest recorded checks: 64 core/PostgreSQL, 23 frontend and 12 browser checks pass. This is increment evidence, not complete-course acceptance.
- Open: intermittent missing graph edge, stale-marker/live-orphan recovery, watchdogs/calibration/heavy profiles, assessment leftovers, remaining fixture domains, privileged inspection/scenarios, curriculum/media and cross-platform delivery.
- Phase 1 external Claude review is pending specific source-transfer approval after an automatic rejection. No source was sent; this is not nonresponse. Mass video production remains gated.

## Working rules

1. Read the affected callers and existing patterns. Make the smallest complete change; keep code, fixture definitions, references, lesson evidence and content revisions synchronized.
2. Learner SQL always executes as a restricted PostgreSQL role. Administrator connections execute only authored setup/recovery or narrowly scoped inspection. Credentials never enter the webview. Never grant broad filesystem, OS or database inspection privileges.
3. Never attach to an existing server, import customer data or copy production dumps. Use synthetic fixtures and explicitly owned temporary clusters. Preserve installed apps, existing databases, saved scripts and passes.
4. Do not infer ownership from a PID or naming prefix. Do not delete server markers or signal an existing process to get startup working. Current marker refusal is deliberate until safe recovery is implemented.
5. Grade full typed results, including duplicate multiplicity, nulls, decimals and required order. Mutation tasks also check persisted state. Use independent expected-result calculations and fresh assessment variants, not SQL text matching or copied reference-query logic.
6. Label illustrations, planner estimates and execution measurements separately. Animation/video must share deterministic frame timing; no independent clocks or invented performance counters.
7. Use meaningful regression tests for nontrivial changes. Execution, permissions, grading, cancellation and reset changes require real PostgreSQL tests. Browser evidence does not replace native playback/lifecycle verification.
8. Add contiguous chapter files without gaps. Keep new chapters unpublished until complete narration, captions, fixtures, grading and acceptance checks pass. Current narration/render scripts are chapter-1-specific.
9. After each completed phase, request read-only Claude review and resolve material findings. Skip only for evidenced unavailability/nonresponse; an approval rejection must stay recorded as a blocker. Local reviews are complementary.
10. Keep generated runtimes/media, local lab data, credentials, logs, traces and scratch out of Git. Small intentional documentation screenshots belong in `docs/images/`. Record licensing and platform limitations before release.

## Commands

```sh
npm ci
npm run runtime:bundle
python3 scripts/narrate.py
npm run video:render
```

For browser work, run `cargo run -p academy-core --bin preview` and `npm run dev` in separate terminals. Vite is at port 1420; its local Rust bridge is at port 4318. Set `ACADEMY_HOME` for an isolated preview and `ACADEMY_POSTGRES` for its runtime.

```sh
ACADEMY_POSTGRES="$PWD/src-tauri/resources/postgres" cargo test -p academy-core -- --test-threads=1
npm test
npm run build
npm run test:e2e
npm run tauri -- build --debug --bundles app
```

Browser tests require both preview processes and Google Chrome. Native builds require the bundled runtime/media and macOS development tools. Narration needs macOS speech synthesis and ffmpeg/ffprobe. Generated assets must exist before packaging; a source checkout alone has no bundled lesson video or PostgreSQL executable.
