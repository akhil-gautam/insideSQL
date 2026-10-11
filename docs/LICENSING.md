# Dependency licensing record

Recorded 2026-10-11 for this development build. The installed Remotion and Theatre license texts are preserved in `docs/licenses/`; dependency versions are pinned by the lockfiles.

- Remotion 4: [official license](https://www.remotion.dev/license). Free eligibility includes individuals, eligible small organizations, nonprofits, and noncommercial evaluation. A company license is required outside those categories. This record does not establish the eventual publisher's eligibility; check it before distribution.
- Theatre Core: the installed Apache 2.0 license is preserved with its notice.
- Anime.js: installed MIT license text is preserved in docs/licenses/. The former Three.js renderer was removed in the database-atlas redesign; its license copy remains with the historical implementation records.
- Local Space Grotesk and IBM Plex Mono: installed font license texts are preserved in docs/licenses/.
- PostgreSQL: [official license](https://www.postgresql.org/about/licence/). The development runtime is PostgreSQL 18.3 from Postgres.app, with its original bin/lib/share directories and included third-party notices. That distribution also includes optional libraries outside the course's current requirements. A release needs a runtime dependency inventory, retained notices and removal of unrelated extensions.
- macOS narration currently uses the installed Samantha voice. Verify redistribution terms or replace the voice with owned narration before publishing the course.

The current bundle is a local development artifact. Public signing, notarization, distribution licensing and a complete third-party notice manifest remain release gates.
