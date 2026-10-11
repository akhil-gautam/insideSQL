# Silent replay and native media policy

Spec: docs/PLAN.md; preserve full 84/504 delivery and phase acceptance requirements.

Goal: remove unnecessary embedded audio from the silent interactive replay and verify real playback under the native media policy. This increment does not claim to fix the separately observed blank native lesson video or prove current-binary native acceptance.

Review Focus: The replay has no audio content; removing shared audio tags must preserve frame advancement and pause. Keep narrated chapter video and its captions unchanged. Do not loosen script/media/network policy, change learner SQL permissions, copy learner data or replace/stop the existing bundle/server. Browser validation exercises the production media-src directive but is not complete WebKit/native acceptance.

### Task 1: Remove blocked audio warmup from the silent replay

Files: web/src/App.tsx, web/e2e/cinematic.spec.ts, docs/PROGRESS.md, docs/reviews/replay-media-policy.md.

Interfaces: Existing Remotion Player renders Replay; Replay accepts its frame clock through useCurrentFrame and contains only QueryJourney. Native media-src remains unchanged. No Rust/fixture/grader/media revisions.

- [x] Add a real-browser regression that applies the native media-src to the document and records SecurityPolicyViolation events. Opening/playing replay must produce no media-src violations, its clock must reach Parser, and paused frames must remain stable. Expected RED: five blocked data: audio preloads.
- [x] Set Player numberOfSharedAudioTags=0 for the silent Replay. Expected GREEN: no unused audio elements/policy violations; Parser reached and paused frame stable. Narrated local video remains separate.
- [x] Run frontend/browser suites and production/native debug --no-bundle builds; stop only owned previews. Expected: all pass; native acceptance remains separately tracked.
- [x] Commit, obtain one fresh read-only review, disposition all findings, fix material issues once RED→GREEN and rerun required checks. Record external Claude block truthfully; no source export. Full course goal remains active.
