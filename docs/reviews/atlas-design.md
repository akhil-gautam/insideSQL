# insideSQL atlas finish review

## Disposition

**Ship — both scoped findings resolved.** Re-reviewed the updated source and recaptured desktop/mobile surfaces after the finish batch. The chosen direction succeeds; no further redesign or polish batch is requested.

## Brief fidelity

Reviewed against `artifacts/atlas-redesign/direction.md`. The paper, pine, restrained rules, and typography form a coherent quiet study surface. The exercise prompt sits directly above the editor; Run, Explain, Analyze, and Check are together. At 1440px the first viewport contains the working editor, actions, result rows, and the illustrated mechanism. At 390px practice comes first and the primary actions remain in the initial viewport. The curriculum disclosure is visible.

The six stage captures show concrete PostgreSQL structures: raw parse-tree branches, catalogs, rewriting, alternative scans, buffer frames, and heap pages. These satisfy the requested move away from decorative machinery. The distinction between illustration and measured execution is explicit. The interface is restrained without sacrificing recognizable controls or the core learning workflow.

## Craft and accessibility

Desktop hierarchy, grouping, whitespace, and alignment are consistent. The prompt and editor read as one work surface. None of the eight reviewed captures show clipped primary copy or horizontal page overflow. Selection, caret, scrollbar, focus, disabled, error, and result styles are present in source. QueryJourney provides pause and reduced-motion handling; the screenshots also show paused/reduced states.

The follow-up mobile capture preserves the drawing at its 600px authored width inside a local horizontal scroller. Labels are now visibly readable, an explicit scroll hint explains the interaction, and source makes the viewport keyboard-focusable. The page itself remains contained. Both previously failing secondary text colors have been corrected.

## Findings and actions

1. **Resolved — Mobile diagram labels.** The mobile rule now sets the SVG to 600px × 425px, matching its viewBox at 1:1 scale. The 11–12-unit labels therefore retain 11–12 CSS pixels instead of shrinking to approximately 6–7px. The recaptured 390px page shows readable labels, the “Scroll across to explore the diagram” hint, and the motion control at the visible edge. `overflow-x:auto` confines horizontal movement to the drawing; `tabIndex=0` provides keyboard access. The shared drawing rule applies to all six stages. This resolves the requested readability fix while preserving the complete mechanism.

2. **Resolved — Secondary text contrast.** `.engine-contract>span` and `.engine-internals` now both use `#617168` on `#fdfefa`. Independently recalculated sRGB contrast is **5.09:1**, exceeding 4.5:1. The global rule also applies to the video styles. No additional color change is requested.

## Verification limits

Read-only review of App.tsx, style.css, QueryJourney.tsx, and DatabaseDiagram.tsx; opened the desktop and mobile full-page screenshots plus all six stage captures. The contrast figures above were independently calculated from source colors. The detector was not rerun: the parent reports 109 advisory differences against the discarded visual world and no nonadvisory findings. The parent also reports frontend 23/23 and browser 12/12 passes, regenerated deterministic video, and observed SQL/grading/play/seek behavior; those checks were not repeated here.

Follow-up verification was restricted to the two reported findings: reopened the updated desktop and mobile captures, inspected the shared source rules, and recalculated contrast. I did not interactively scroll every stage again; the common 1:1 SVG sizing provides source evidence for consistent label scale.

Screenshots establish browser appearance, not packaged native-app appearance. I did not perform a screen-reader session, replay the full video, or review backend correctness. The parent reports the full video is regenerating with the corrected colors; completion and verification of that output remain with the parent. Mobile requires horizontal exploration of the drawing by design, clearly disclosed by its hint.
