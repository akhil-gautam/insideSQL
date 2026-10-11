# Silent replay media-policy review — 2026-10-11

Fresh local read-only review covers 25f577e..1f63469. No Critical, Important or Minor findings in this increment; no deferred minors. The reviewer inspected source, installed dependency implementation and recorded logs without mutations, exports or rerunning tests. No re-review or fix pass was needed.

The installed WebKit console rejects five Remotion silent data:audio/mp3 tags under the existing media-src directive. Replay contains only visual QueryJourney content. Player now requests zero shared audio tags, leaving its frame clock/pause path, narrated chapter video/captions, native policies, Rust/learner permissions, fixtures, grader and media revisions unchanged.

The real-browser regression reproduced five media-src SecurityPolicyViolation events before the one-line fix. It now applies the exact production media directive, requires no violations or unused audio, observes actual progression to Parser and checks a stable paused canvas. Focused GREEN passes1/1. Final frontend23/23 and uninstrumented browser11/11 (38.1s), production/native debug --no-bundle builds pass. Core was not rerun for this frontend-only increment. Only owned previews were stopped; installed bundle/server remained untouched.

Two earlier full browser runs failed the existing deletion-keys check: edge count0 instead of1, while both nodes remained. Three focused repeats, the instrumented full run and final uninstrumented run passed without any graph repair. The diagnostic timer did not establish root cause; temporary diagnostics were removed. This observed issue remains open despite later green runs.

Every ruling and disposition:

- Remove unused audio tags rather than relax native CSP: the current replay is silent. Future authored audio must revisit media/autoplay behavior.
- Installed replay at0:00 and blank chapter video remain unresolved. The installed binary predates this fix; current-binary GUI behavior is unproven, so stalled/blank playback may remain.
- Complete native CSP/WebKit behavior remains unproven. Chrome exercises only the authored media directive; decoding, policy integration and performance can still fail.
- Intermittent missing graph edge remains a required diagnostic follow-up. Later green runs without a fix do not explain or repair missing measured-plan connections.
- Future narrated replay must revisit zero shared tags and authored media policy. Added audio may otherwise encounter autoplay or policy failures.
- All remaining84-chapter/504-exercise delivery, runtime recovery/resources and cross-platform acceptance remain active. Shipping now would leave explicit course/runtime requirements incomplete.
- Local review is separate from requested external Claude phase review. Automatic approval rejected Anthropic source transfer pending specific approval; no source was sent and phase acceptance remains pending.

Native chapter media requests reach the asset handler with206 responses; source/bundle hashes match and bundled ffprobe identifies1920x1080 H264/AAC,417.941s. This does not prove WebKit decode or explain the blank frame. Remotion documents its [shared audio tags](https://www.remotion.dev/docs/options/number-of-shared-audio-tags) and [autoplay warmup](https://www.remotion.dev/docs/player/autoplay); its installed implementation was inspected directly.
