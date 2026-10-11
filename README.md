# SQL Internals Academy

Lab resources opens an actual storage/settings snapshot for the current exercise. The owned runtime requires 512 MiB of available disk space before initialization, lab allocation, execution and Reset, and limits temporary spills to 128MB per PostgreSQL process. Reset prepares a fresh baseline before replacing the old database. These are preflight and spill safeguards, not total-memory quotas or hardware calibration; larger profiles, continuous monitoring and recovery/release checks remain required.

Measured-plan eligibility is bound to the actual owned PostgreSQL cluster and exercise database as well as SQL and grader revision. Progress and Check inspect the current lab identity, so old evidence cannot satisfy Check after database replacement even if deleting its SQLite record fails. Existing scripts, unlocks and passes are preserved; measurements saved before identity binding need Analyze again. Restarting the same cluster/database preserves current evidence. General interrupted Reset and orphan recovery remain required.

A native Tauri/Rust learning workbench with real, disposable PostgreSQL labs, Theatre.js mechanism scenes and Remotion chapter videos.

The complete approved scope is [docs/PLAN.md](docs/PLAN.md). Verified delivery status is [docs/PROGRESS.md](docs/PROGRESS.md). The full course is **not complete**: chapter 1 is delivered, chapters 2–10 have unpublished authored drafts and tested SQL exercises, and later chapters remain to author.

## Local macOS development

Requires Rust, Node.js, Chrome, ffmpeg, macOS speech synthesis, and the relocatable PostgreSQL 18.3 build in Postgres.app. The learner-facing packaged app will bundle the PostgreSQL runtime and media and needs none of the authoring tools.

```sh
npm ci
npm run runtime:bundle
python3 scripts/narrate.py
npm run video:render
npm run tauri -- build --debug --bundles app
```

The desktop bundle is created under `target/debug/bundle/macos/`. App-owned PostgreSQL data and progress are separate from existing PostgreSQL installations. No Docker or existing server connection is used.

For browser development, start the Rust test bridge in one terminal and Vite in another:

```sh
cargo run -p academy-core --bin preview
```

```sh
npm run dev
```

Open `http://127.0.0.1:1420`. The browser bridge is a local development tool; the desktop application calls Rust directly through Tauri.

## Verification

```sh
cargo test -p academy-core
npm test
npm run build
npm run test:e2e
```

The PostgreSQL tests create fresh temporary clusters. Set `ACADEMY_POSTGRES` to the bundled runtime directory to verify the distribution itself. Tests require permission to start local PostgreSQL shared-memory processes and loopback connections.

The video renderer verifies repeated frame hashes before producing the complete 1080p H.264 lesson. English captions use the exact boundaries of independently validated narration clips. Generated media and runtime binaries are local build artifacts, not checked-in source.

## Learning contract

- A first learner-written submission unlocks a collapsed answer, including invalid SQL.
- Empty SQL, comments alone and unchanged starter SQL do not unlock answers.
- All six exercises must pass across fresh fixture variants to complete the chapter.
- Grading preserves duplicates, nulls, numeric values and specified row order.
- Learner connections have no administrator or filesystem privileges.
- Explain estimates and Analyze execution evidence are labeled separately.
- Write exercises permit DML only on their authored disposable tables. Run retains successful changes; Analyze measures inside a rolled-back transaction; Check uses separate fresh learner/reference databases and verifies both returned rows and resulting state. Reset restores starting data.

Current mutation-state comparison is complete within a 10,000-row/8 MB bound and fails rather than accepting truncated state. Larger write profiles still require streaming comparison before delivery.

## Authored catalog

Rust embeds contiguous `course/chapterNN.json` files at build time, starting at `01` and ending no later than `84`. Adding the next authored file requires no runner source-list edit. Malformed names, missing numbers and symlinked chapter files fail the build. Runtime validation still requires six exercises, valid identifiers and a published introductory chapter.

Keep new chapters unpublished until their narrated media and acceptance gates pass. The current narration/render scripts produce chapter 1; adding catalog content does not generate another chapter's video or scene.

## Review

Each completed phase receives a read-only Claude review. Review findings, evidence and unavailable/time-out results belong in `docs/reviews/` and the progress ledger. Do not mark a phase complete solely because tests or a chapter outline exist.
