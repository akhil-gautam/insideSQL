---
name: "SQL Internals Academy"
description: "A quiet SQL workbench with an illustrated mechanical journey inside PostgreSQL."
colors:
  ink: "#23344d"
  blue: "#2459b8"
  blue-hover: "#19499e"
  muted: "#718097"
  line: "#dce3ed"
  workbench-ground: "#eef2f8"
  workbench-surface: "#ffffff"
  sidebar-ground: "#e5ebf4"
  button-hover: "#edf3ff"
  focus: "#8aabeb"
  engine-ground: "#201e1d"
  engine-copy: "#fff0df"
  engine-muted: "#c7b9ab"
  engine-accent: "#ffc59e"
  engine-panel: "#292624"
  engine-border: "#55504b"
  engine-hover: "#403932"
  engine-graphite: "#4c4946"
  engine-steel: "#8b8179"
  engine-edge: "#161312"
  caption-ground: "#181615"
typography:
  body:
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "14px"
  headline:
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "30px"
    fontWeight: 600
    letterSpacing: "-1px"
  workbench-label:
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "12px"
  engine-title:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "clamp(23px, 2.2vw, 32px)"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "-1px"
  engine-body:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "12px"
    lineHeight: 1.7
  sql:
    fontFamily: "ui-monospace, SFMono-Regular, Menlo, monospace"
    fontSize: "14px"
  engine-annotation:
    fontFamily: "'IBM Plex Mono', monospace"
    fontSize: "12px"
    lineHeight: 1.4
  video-title:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "54px"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "-2px"
  video-annotation:
    fontFamily: "'IBM Plex Mono', monospace"
    fontSize: "20px"
    lineHeight: 1.4
  video-caption:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "28px"
    lineHeight: 1.45
rounded:
  button: "6px"
  panel: "10px"
  chapter: "8px"
  engine-control: "2px"
  circle: "50%"
spacing:
  compact: "6px"
  control-gap: "8px"
  panel-inset: "18px"
  workbench-gap: "22px"
  engine-inset: "28px"
  main-inset: "32px 36px"
components:
  button-primary:
    backgroundColor: "{colors.blue}"
    textColor: "{colors.workbench-surface}"
    rounded: "{rounded.button}"
    padding: "7px 11px"
  button-primary-hover:
    backgroundColor: "{colors.blue-hover}"
  button-secondary:
    backgroundColor: "{colors.workbench-surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.button}"
    padding: "8px 12px"
  button-secondary-hover:
    backgroundColor: "{colors.button-hover}"
  workbench-panel:
    backgroundColor: "{colors.workbench-surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.panel}"
  chapter-active:
    backgroundColor: "{colors.workbench-surface}"
    textColor: "{colors.blue}"
    rounded: "{rounded.chapter}"
    padding: "13px 8px"
  sql-editor:
    typography: "{typography.sql}"
    height: "210px"
  engine-stage:
    textColor: "{colors.engine-muted}"
    rounded: "{rounded.engine-control}"
    padding: "7px 9px"
  engine-stage-active:
    backgroundColor: "{colors.engine-accent}"
    textColor: "{colors.engine-ground}"
  engine-annotation:
    backgroundColor: "#201e1deb"
    textColor: "{colors.engine-copy}"
    typography: "{typography.engine-annotation}"
    padding: "3px 6px"
  illustration-label:
    textColor: "{colors.engine-muted}"
    padding: "5px 8px"
---

# Design System: SQL Internals Academy

## Overview

**Creative North Star: "The Database Engine Workshop"**

A quiet slate-blue workbench surrounds a separate charcoal mechanical illustration. White editing and assessment surfaces keep real SQL work readable; the exploded engine supplies depth and cinematic attention inside the lesson.

The journey uses local Space Grotesk and IBM Plex Mono, warm copper light, and ivory text. Six mechanically distinct chambers are inspectable as SQL text, parser, analyzer, rewriter, planner and executor. The assembly is a teaching metaphor; measured query evidence belongs to the workbench.

**Key Characteristics:**
- Slate-blue application chrome and white work surfaces.
- Scoped charcoal machinery, copper edges and ivory annotations.
- Real geometry with distinct internal structures for each stage.
- One shared scene for interactive exploration and frame-driven video.

Recorded from `web/src/style.css`, `DatabaseEngine.ts`, `QueryJourney.tsx`, `App.tsx`, `SqlEditor.tsx` and `engineMotion.ts` after the cinematic correction pass. `docs/CINEMATIC-DIRECTION.md` supplies the established direction; implementation determines the tokens above.

## Colors

### Primary

Workbench Blue marks Run, Check, selected lesson tabs, current exercises and progress. Its deeper hover companion gives a modest interaction change. Warm Copper is the journey's scoped accent: selected stages, source labels, emissive traces and rim light.

### Neutral

Deep Ink, Muted Slate and Line organize text and separators on white Workbench Surface, pale Workbench Ground and Sidebar Ground. Inside the journey, Engine Ground and Panel anchor Ivory Copy, Warm Muted Copy and Machined Border. Graphite, Steel and Edge are actual 3D material colors, separate from the interface palette. Caption Ground provides the video subtitle band.

**The Scoped World Rule.** Keep the charcoal/copper palette inside the journey and its video. The surrounding workbench retains its slate-blue/white system.

## Typography

The workbench uses the operating system sans stack. CodeMirror and SQL results use the system monospace stack. Locally imported Space Grotesk supplies journey titles and explanatory text; IBM Plex Mono supplies SQL, catalog, syntax-node and operator annotations.

The frontmatter records the built hierarchy. Workbench headings step down to 25px at the medium breakpoint and 24px on narrow screens. Journey titles are fluid on desktop and 25px below 600px. Video uses its own larger title, annotation and caption roles. Source annotations remain 12px in desktop and mobile interactive views, and 20px in video; they are projected DOM elements rather than texture text.

**The Readable Annotation Rule.** Render selected-stage labels as projected DOM text using the annotation roles; keep source names and input/output contracts readable outside the 3D canvas.

Existing 9–11px auxiliary controls and metadata describe the current compact UI; they are not a new minimum-size recommendation for future screens.

## Layout

The desktop application uses a sticky 236px sidebar, a 70px header and a centered main region capped at 1700px. The workbench splits into a flexible lesson/editor column (minimum 450px) and a 340px exercise panel, separated by the workbench-gap token.

At 1250px, sidebar width becomes 210px, the exercise column 300px and grid gap 16px. At 1000px, the sidebar becomes 185px and the workbench becomes one column, with exercises above the lesson. At 650px, the sidebar hides, header height becomes 55px and main padding becomes 20px 14px.

The journey is a flex column with a title, expanding canvas and readable text contract. Its desktop minimum height is 620px; below 600px it becomes 675px with a 300px minimum canvas. Stage controls wrap. On narrow-aspect overview views the assembly rotates into a steeper diagonal; stage inspection expands chamber spacing and focuses the selected mechanism. Details stack on mobile rather than squeezing side-by-side. Video uses 64px horizontal insets and a separate 144px caption band.

## Elevation & Depth

The workbench is chiefly flat: borders, pale header fills and white surfaces separate content. Its selected chapter carries a subtle ambient shadow (`0 2px 6px #314a7210`). The engine instead uses genuine procedural Three.js geometry, orthographic projection, outlined edges and physically shaded materials. Graphite casing uses roughness 0.64/metalness 0.25; darker internals use 0.76/0.15; steel uses 0.45/0.35. Warm, cool and copper directional lights describe chambers, rails, screws and internals. ACES filmic tone mapping at exposure 1.05 shapes the final rendered color; material swatches are source colors, not screenshot samples.

Parser node collars, mounts and screws give the syntax tree physical structure. Planner route pods, pins and selector housing give alternate paths mechanical depth. Camera inspection dims unselected material and highlights the selected chamber without replacing the scene.

The depth pass adds bevelled front/back flanges, toothed bezels, bolted cutaway jackets and cooling fins to each chamber. SQL guide rails and contact traces, geared syntax collars, catalog drawer handles/backplanes, rewrite gate teeth, planner cooling ribs/gears and heap-board connectors supply distinct internal detail. Feed wafers, syntax cores, catalog drawers, rewrite gates, selector gears and the executor read head use the same deterministic frame clock. Settled inspection isolates the chosen chamber; Overview restores its neighbours and connecting rails.

**The Illustrated Evidence Rule.** Keep the “Illustrated mechanism” label visible. Geometry, signal packets and camera motion do not represent a measured PostgreSQL trace.

Camera transitions use Anime.js outExpo across 1.6 seconds. Theatre supplies seekable progression; interactive controls can pause and reduced motion settles inspection immediately. Remotion supplies the sole video frame clock. These extension tokens are recorded in the sidecar.

## Shapes

Workbench panels have gently curved panel corners; buttons use the smaller button radius and chapters the intermediate chapter radius. Exercise selectors are circular. Journey stage controls use nearly square corners and thin machined borders. The canvas assembly uses rings, angular blocks, tubes and bolted housings rather than flattened interface cards. Geometry is an illustration device, not a new application-wide container language.

## Components

### Buttons

Workbench actions use the blue primary and white secondary variants. Hover colors come from their sibling tokens. A 3px blue focus outline with 3px offset marks keyboard focus. Disabled controls retain their native disabled state and muted text. Check spans the exercise width; Run stays compact.

### Navigation

Selected lesson tabs use a blue underline and stronger text. Active chapter navigation uses white fill, blue text and the subtle chapter shadow. Locked future chapters are disabled. Six circular exercise controls distinguish current and passed states with color plus position/checkmark.

### Cards / Containers

Lesson, editor and exercise containers use white fill, line borders, panel corners and clipped edges. Pale header bands and compact inset spacing divide functional areas; these are working surfaces rather than ornamental floating cards.

### Inputs / Fields

SQL input is the installed CodeMirror editor, 210px tall with wrapping, PostgreSQL syntax and monospace text. It becomes read-only while a query is busy. The sidecar shows a static textarea sample of the editor surface for preview only; it does not replace CodeMirror's behavior.

### Chips

The illustrated-mechanism badge uses IBM Plex Mono, a thin engine border and compact padding. It is a disclosure rather than an action. Retain it beside the journey title, allowing it to fall below the title on mobile.

### Database Engine Journey

Overview reveals all six chambers; stage controls inspect one. Each chamber has distinct geometry and actual lesson names: SQL slats, mounted parser nodes, analyzer catalog banks, rewrite gates, planner selector routes and executor pages. Stage controls use copper fill when pressed and copper focus outlines. Projected annotations are noninteractive and shown only for the selected stage; accessible canvas naming and persistent text details complement them. WebGL failure retains the controls and explanations. Pause is available in interactive views; reduced motion disables the pause action with an explicit state label.

## Do's and Don'ts

### Do:
- Do retain the workbench palette and UI typography around the scoped cinematic journey.
- Do use different internal structures for the six engine stages; the shared casing is the metaphor.
- Do pair each inspection view with its input/output contract and text explanation.
- Do preserve Overview, keyboard stage selection, pause/reduced-motion behavior and the graphics fallback.
- Do distinguish illustrated mechanisms from PostgreSQL estimates and measured execution.

### Don't:
- Don’t extend the journey’s charcoal/copper treatment across the entire application.
- Don’t replace sharp DOM source annotations with rasterized text on geometry.
- Don’t add invented execution timings or measured row counts to the illustration.
- Don’t give video labels or rotors an independent animation clock.
