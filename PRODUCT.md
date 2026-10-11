# SQL Internals Academy

An offline native Rust/Tauri learning lab for people who know basic SQL and want to understand PostgreSQL internals and improve queries. The app manages its own disposable PostgreSQL 18 runtime, local fixtures and progress; learning requires no account or existing database server.

The learning loop combines a narrated lesson, captions/transcript, an inspectable mechanism illustration, and SQL exercises against a real engine. Solutions start collapsed and unlock after the first nonempty learner-written Run or Check, including invalid SQL. Unchanged starters and comments alone do not count. All six exercises must pass to complete a chapter; retries are unlimited. Estimates, measured execution and illustrations are labeled separately.

The approved scope is 84 chapters and 504 required exercises. Chapter 1 is delivered as the current development chapter with a packaged macOS workbench, narrated video and six exercises. Chapters 2–7 have authored drafts and six working SQL exercises each; all remain unpublished pending media and acceptance gates. Chapters 3–7 use constrained commerce fixtures with customers, orders, lines and payments. Chapter 5 permits writes only to its disposable orders table and grades both output and resulting state. Chapter 6 requires successful Analyze of the current SQL before grading, with evidence tied to the query and grader revision. Chapter 7 starts the Plans module, with inspectable per-operation evidence in graph/tree views and six measured report exercises. The full course is **not complete**; later chapters, advanced labs and cross-platform delivery remain open. Phase review and release gates are tracked separately from chapter availability.

The journey's charcoal/copper mechanical world is scoped to the lesson illustration and video. The surrounding slate-blue/white workbench stays focused on writing, checking and inspecting SQL. The scene is an illustrated mechanism, not a measured executor trace.

Authoritative scope: [docs/PLAN.md](docs/PLAN.md). Verification and remaining work: [docs/PROGRESS.md](docs/PROGRESS.md). Development and learning contract: [README.md](README.md).
