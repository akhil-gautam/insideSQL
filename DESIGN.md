---
name: "SQL Internals Academy / insideSQL"
description: "A quiet paper-and-pine workbench for exploring PostgreSQL through SQL practice."
colors:
  ink: "#26352e"
  accent: "#365e4b"
  accent-hover: "#284c3a"
  muted: "#617168"
  line: "#dce3da"
  paper: "#f7f8f4"
  surface: "#ffffff"
  soft: "#edf2eb"
  sidebar: "#f0f3ed"
  focus: "#577d66"
  diagram-ground: "#fdfefa"
  diagram-ink: "#2f3e32"
  diagram-line: "#b0c0a4"
  diagram-green: "#6d9155"
  diagram-soft: "#edf3e6"
typography:
  body:
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "14px"
  headline:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "clamp(23px, 2.05vw, 30px)"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "-0.9px"
  title:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "21px"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "-0.5px"
  prompt:
    fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "12px"
    lineHeight: 1.75
  sql:
    fontFamily: "'IBM Plex Mono', monospace"
    fontSize: "12px"
    lineHeight: 1.85
  diagram-annotation:
    fontFamily: "'IBM Plex Mono', monospace"
    fontSize: "11px"
  video-title:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "43px"
    fontWeight: 500
    lineHeight: 1.2
  video-caption:
    fontFamily: "'Space Grotesk', sans-serif"
    fontSize: "24px"
    lineHeight: 1.5
rounded:
  control: "6px"
  panel: "9px"
  stage: "4px"
  diagram-node: "5px"
  label: "3px"
spacing:
  exercise-gap: "6px"
  workbench-gap: "25px"
  main-inset: "29px 32px 38px"
  mobile-inset: "22px 16px 30px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.surface}"
    rounded: "{rounded.control}"
    padding: "8px 9px"
  button-primary-hover:
    backgroundColor: "{colors.accent-hover}"
  button-secondary:
    textColor: "{colors.ink}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  button-secondary-hover:
    backgroundColor: "{colors.soft}"
  lesson-panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.panel}"
  chapter-active:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.control}"
    padding: "11px 10px"
  sql-editor:
    backgroundColor: "{colors.surface}"
    typography: "{typography.sql}"
    height: "224px"
  stage-control:
    textColor: "#68795a"
    rounded: "{rounded.stage}"
    padding: "6px 7px"
  stage-control-active:
    backgroundColor: "#e9f1e1"
    textColor: "#3f6130"
  illustration-label:
    backgroundColor: "#f5f9ef"
    textColor: "#617554"
    rounded: "{rounded.label}"
    padding: "4px 6px"
---

# Design System: SQL Internals Academy / insideSQL

## Overview

**Creative North Star: "The Quiet Database Atlas"**

Paper, pine-green accents and precise vector drawings create a restrained study surface. Space Grotesk gives headings a clear technical voice, native sans supports reading, and IBM Plex Mono identifies SQL and database structures. The Japanese-inspired restraint is expressed through spacing, rules and limited color.

The lesson and practice share one visual world. Database illustrations show concrete structures and retain an explicit illustration label; real query results remain in the practice surface.

**Key Characteristics:**
- Pale paper, white working surfaces and pine-green selection.
- A connected prompt, SQL editor, action row and results surface.
- Inspectable database diagrams with crisp vector text.
- Shared deterministic timing for exploration and narrated video.

Recorded from the finished `web/src/style.css`, `App.tsx`, `SqlEditor.tsx`, `DatabaseDiagram.tsx`, `QueryJourney.tsx` and `engineMotion.ts`; direction and review live in `artifacts/atlas-redesign/direction.md` and `docs/reviews/atlas-design.md`.

## Colors

### Primary

Pine Accent identifies Check, current exercises, selected tabs and progress. Accent Hover deepens primary controls. The diagram's lighter green and soft fills highlight branches, catalog rows, buffers and tuple slots within the same palette.

### Neutral

Paper surrounds white Surface panels. Sidebar and Soft provide gentle grouping; Ink, Muted and Line establish readable hierarchy. The diagram uses its own near-white ground, ink and ruled edges. Error and grade feedback retain their implemented red, amber and green state treatments, accompanied by text.

**The Shared Paper Rule.** Keep the lesson, illustration and practice in the same paper-and-pine world.

## Typography

Locally imported Space Grotesk supplies identity and headings; the operating-system sans stack supplies prose and controls. IBM Plex Mono supplies code, data and SVG labels. The frontmatter records the recurring roles, not every compact caption.

Chapter headings become 26px below 980px and 25px below 650px. The prompt stays comfortably spaced and capped at 68ch. SQL text becomes 11px on narrow screens. Drawing text has an 11-unit minimum in its authored SVG; prominent SQL text is larger. Video enlarges headings, contracts and captions for its separate output dimensions.

Existing 8–10px badges, controls and metadata describe this compact implementation; they are not a recommended minimum for new prose.

## Layout

Desktop uses a sticky 216px curriculum rail, a 62px header and a main region capped at 1820px. The workbench pairs a lesson column (`minmax(340px, .92fr)`) with a larger practice column (`minmax(425px, 1.08fr)`). Prompt, editor, Run/Explain/Analyze/Check and results form one connected surface.

At 1190px, the rail and gaps narrow. At 980px, the workbench becomes one column with practice first. At 650px, curriculum becomes a disclosed fixed rail beneath the sticky 54px header; query actions can wrap. At 1600px and above, the workbench gap expands to 34px and the editor height grows to 262px. The editor is 205px tall below 650px.

On narrow screens, the interactive drawing retains its authored 600px × 425px size inside a local horizontal scroller. The visible scroll hint and keyboard-focusable viewport make exploration discoverable; page width stays contained. The motion control stays at the visible edge. Video uses separate fitted drawing and caption layouts.

## Elevation & Depth

White surfaces, fine rules and pale fills provide most separation. Selected chapters use a small shadow (`0 2px 5px #253e2210`); selected result switches use `0 1px 3px #31472114`. The mobile curriculum overlays content with `8px 0 25px #233b2218`. Avoid turning the main surfaces into floating cards.

SVG drawings use offset table sheets, stacked heap-page cutaways and restrained side-face tones. Motion highlights structures and crossfades between views. Theatre provides seekable progression and Anime supplies the 1.6-second outExpo stage blend; Remotion supplies the video frame clock. Pause freezes the interactive view, while reduced motion freezes activity and settles stage selection immediately.

## Shapes

Panels use gentle corners; compact controls are slightly tighter. Exercise selectors are rounded squares with checkmarks for passes. Stage controls and the illustration label use smaller corners. Diagram nodes are ruled rectangles, catalog tables have aligned rows, and heap pages form a layered cutaway. These shapes explain database structure.

## Components

### Buttons

Check uses the pine primary treatment; Run uses a white outlined treatment and adjacent secondary actions remain quiet. Controls share restrained hover transitions, disabled opacity and explicit keyboard focus: a 2px green outline offset by 3px. Preserve Cancel and busy-state labels when executing a query.

### Navigation

Lesson tabs use pine text and an underline for selection. Active chapters use white fill, a fine border and the small chapter shadow. Six exercise selectors distinguish the current exercise from passes using fill, position and checkmarks. The mobile curriculum toggle keeps the rail reachable.

### Cards / Containers

The exercise prompt has rounded upper corners; the adjoining editor/results surface has rounded lower corners. Their shared border reads as one work area. Lesson tabs and lesson content use the same joined construction. Table headers, utility disclosures and editor headers use subtle tonal bands.

### Inputs / Fields

CodeMirror supplies PostgreSQL syntax, line wrapping and the accessible SQL input. The editor uses a white ground, pale active line, green caret and selection, muted line numbers and restrained syntax colors. It becomes read-only while busy. The sidecar's textarea is a visual preview, not a replacement editor.

### Chips

“Illustrated mechanism” is a compact, noninteractive disclosure with a fine border and pale fill. Keep it visible beside or below the diagram title.

### Database Atlas

Overview and six stages reveal SQL tokens, parse-tree branches, catalog tables, query rewriting, competing scan paths, buffers and heap pages. SVG text stays crisp. Persistent input/output contracts and explanations complement the drawing; stage buttons expose their pressed state. The mobile scroller is focusable and retains the browser's native focus treatment; other controls use the explicit focus ring above.

**The Illustrated Evidence Rule.** Keep “Illustrated mechanism” visible and distinguish the drawing from PostgreSQL estimates and measured execution.

## Do's and Don'ts

### Do:
- Do keep prompts, editing, query actions and results together.
- Do put practice first when the workbench stacks.
- Do preserve readable diagram labels with contained horizontal exploration on mobile.
- Do retain Overview, stage selection, pause, reduced motion and persistent text explanations.
- Do distinguish illustrations, estimates and measured execution.

### Don't:
- Don't reintroduce slate-blue chrome or charcoal-and-copper machinery.
- Don't add decorative Japanese glyphs, fake wood, turbines or bolts.
- Don't shrink mobile diagrams until their source labels become unreadable.
- Don't invent execution timings or measured row counts in illustrations.
- Don't give video elements an independent animation clock.
