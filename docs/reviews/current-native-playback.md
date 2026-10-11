# Current native playback and lifecycle evidence — 2026-10-11

This is a verification record, not a new code change or completed phase review. Source HEAD at build time: 8f764a6. All temporary graph diagnostics and CPU-throttling test edits were restored byte-for-byte before the native build; tracked application/configuration/media sources did not change.

## Isolation and build

`npm run tauri -- build --debug --config artifacts/graph-diagnosis/native-check.config.json` finished with exit 0. Its overlay changes only product name, bundle identifier and window title; it retains the authored CSP, bundled resources and native command/runtime implementation. Identifier: `dev.sqlinternals.playbackcheck20261011`; app: `SQL Academy Playback Check.app`. A fresh private app directory and PostgreSQL cluster coexist with the running older bundle. Existing learner progress, installed bundle and its private server were not modified or stopped.

Native executable SHA-256: `b8a572d1b6d36ca77c7e34fd804330e234c90f8bbaf208232882da2f67b978ce`. Bundled chapter MP4 SHA-256: `ad89943b75eefe74d588baafd31622902f7e4c37f27d619f8867e960bdd0d6de`, identical to the delivered source. Production/native builds pass; the existing large-bundle warning remains.

## Direct native observations

The current bundle was opened and operated through the native accessibility interface and screenshots, not a browser substitute. It started at 0/6 passes and used its bundled asset:// lesson.

- Lesson playback rendered the detailed mechanical scene and advanced from 0 to 3, 9 and 16 seconds. Duration was 6:57. Changing timed captions were visible.
- Native Skip Forward controls sought from paused 0:16 to 1:16, rendering the parser scene. Resuming advanced to 1:36. Slider setValue did not seek; clicking that accessibility element paused instead. No slider-based seeking claim is made.
- Shared replay changed Play to Pause, advanced to 0:03 with Parser highlighted and later 0:09 with Planner highlighted. Pausing returned the Play control. A stable paused-frame comparison was not performed in native UI; browser evidence covers that separately.
- `SELEC id FROM orders;` returned real PostgreSQL 42601 at character 1 and unlocked the previously disabled answer. A paid-order Analyze showed measured Sort/Seq Scan, four actual rows and the child-to-parent edge.
- All six authored Chapter 1 SQL references were entered through the editor and checked through native commands. Each displayed “Passed Correct across all three fixture variants.” Progress advanced 1/6 through 6/6. Chapter 2 remained disabled because its video is unpublished.
- First close removed the owned PID file, freed its port and logged “database system is shut down.” Reopening retained 6/6 and restored the first and sixth SQL drafts and unlocked answers. Second close again removed the owned PID file, freed its port and logged a clean shutdown. The older bundle’s server remained listening throughout.

This proves current-build playback, seeking through skip controls, visible captions, replay progression, native SQL/grading/progress and ordinary shutdown/reopen on this Apple Silicon Mac. It does not explain the older bundle’s previously stalled/blank playback, establish audio audibility, stress/performance/reduced-motion acceptance, or prove other desktop platforms and crash recovery.

## Graph diagnosis

An instrumented full browser run passed 11/11 in 41.5s. Twenty ordinary repetitions and three 8x CPU-throttled repetitions of the deletion-keys check also passed. Temporary logs inspected real React Flow node dimensions and connector bounds; no measured node with missing bounds was observed. This did not reproduce or identify the root cause of the two earlier full-suite edge failures. No speculative graph fix, dependency change or permanent diagnostic code was added. Original failure trace remains preserved in `artifacts/native-playback/graph-failure.zip`; current diagnostic logs stay under ignored `artifacts/graph-diagnosis/`.

Final uninstrumented full browser confirmation finished with exit 0: 11/11 in 38.6s. This later green run does not repair or explain the previously observed intermittent edge failure. Only owned browser previews were stopped afterward; the existing native bundle/server were preserved. Frontend/core suites were not rerun for this source-unchanged verification increment.

## Remaining gates

Specific Claude source-export approval is still pending after automatic approval rejection. No source was sent; this is not Claude nonresponse and local review is not substituted for it. Phase 1 review acceptance, the intermittent graph issue, runtime recovery/resource work, all remaining 84-chapter/504-exercise media/content and cross-platform delivery remain open.
