# SQL Internals Academy

A native Tauri/Rust learning workbench with real, disposable PostgreSQL labs, Theatre.js mechanism scenes and Remotion chapter videos.

The complete approved scope is [docs/PLAN.md](docs/PLAN.md). Verified delivery status is [docs/PROGRESS.md](docs/PROGRESS.md). The full course is **not complete**: the current implementation is the first chapter and application foundation, with later chapters still to author.

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

## Review

Each completed phase receives a read-only Claude review. Review findings, evidence and unavailable/time-out results belong in `docs/reviews/` and the progress ledger. Do not mark a phase complete solely because tests or a chapter outline exist.
