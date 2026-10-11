# Sort and aggregate review — 2026-10-11

Scope: unpublished chapter 10, six measured SQL ordering/grouping reports, independent raw-row calculations and test-owned aggregation/spill experiments, plus the shared plan-inspector dimension repair. Fresh local read-only source review covered a0ae45f..e96a97e; the review fix is f30f38c.

The reviewer confirmed exact decimal and unknown-average semantics, conditional populations, grouping masks, physical duplicate multiplicity, complete ordered reports across all three distributions, and isolated observed memory/disk work. Publication gates, measured wrong reports, reset and assessment cleanup remain covered. Reusing ScanOrders and retaining actual measured dimensions were accepted.

One Important finding: forwarding React Flow node changes also enabled its default Backspace removal, letting users delete measured operations from the read-only graph while the selector and JSON retained them. The new browser regression failed with one node instead of two. Disabling deletion with deleteKeyCode=null passes that same regression, preserving both nodes and their edge under Backspace and Delete. No Critical or Minor findings. No second review was dispatched; the fix is verified by RED→GREEN and final suite checks.

The reviewer set aside later chapters/domains/profiles/resource guards/calibration and native GUI/media/external Claude acceptance. Disposition: keep all those gates open, report only local observed mechanism evidence, and preserve the full 84-chapter/504-exercise scope. Local source review and native compilation do not establish release or external phase acceptance.

Before the review fix, 36 core/PostgreSQL/catalog, 16 frontend and eight browser checks passed (60 total). After the fix, the full core rerun passes 36/36 (25 unit, three catalog, six progress and two runtime tests); frontend 16/16 and browser 9/9 also pass (61 total), along with production and native --no-bundle builds. The unit portion took 186.77 seconds and runtime integration 7.31 seconds. Owned preview processes stopped; installed bundle and delivered media were preserved. The existing bundle-size warning remains open.

Native GUI remains locked. Automatic approval review blocks exporting source to Anthropic pending specific approval; no source was sent. This local review does not satisfy external phase acceptance or complete the course goal.
