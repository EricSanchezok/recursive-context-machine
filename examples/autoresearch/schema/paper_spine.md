# PaperSpine Contract

The PaperSpine is the **claim-driven backbone** of the paper: the structure that turns
a set of core claims into a manuscript whose every part argues those claims. Phase 2 is
a *rewrite around the spine*, not a reformat of the technical report.

This is where **de-engineering** happens: the technical report is engineering-complete;
the paper keeps only what serves a claim and moves the rest out. The spine decides what
stays.

Written into `project_dir/62_spine.md`. It also drives the per-section writers.

## Required sections

- `core_thesis`: the one sentence the paper makes (from `61_claims.md`).
- `claim_spine`: the primary claim and the supporting claims beneath it (from
  `61_claims.md`), in the order the paper will argue them. This is the load-bearing
  structure — sections exist to serve these.
- `narrative_strategy`: the chosen arc and why — one of:
  `problem→mechanism→evidence` (method paper) | `observation→investigation→insight`
  (analysis) | `challenge→benchmark→evaluation` (benchmark) | `deficiency→fix→proof`
  (theory).
- `intro_arc`: the topic-sentence sequence the introduction will follow —
  `field_assumption → pain_point → gap → insight → method → evidence → implication` —
  grown from the grounded StorySpine, not invented at writing time.
- `section_plan`: the section list. For each section: `name`, the `claims` it argues,
  the `evidence` it draws from the technical report (`results/` refs), and a one-line
  `job` (what this section must accomplish for the spine). Section order for writing:
  Method → Results → Introduction → Related Work → Discussion → Abstract → Conclusion.
- `de_engineering`: the explicit keep/cut decision —
  - `keep`: the minimal method + evidence each claim needs.
  - `cut_to_supplementary`: implementation detail, full config dumps, exhaustive
    ablations, infra notes — present in the technical report, not in the main paper.
  - `cut_entirely`: engineering work that serves no claim.
  A reader of the paper should see the *idea and its evidence*, not the codebase.

## Rules

- The spine is the claims. Every section's `job` must serve a claim on the spine; a
  section that serves no claim is cut or merged.
- De-engineer ruthlessly but honestly: moving detail to supplementary is fine; deleting
  a caveat or a negative result that bounds a claim is not.
- Keep contributions to the coherent ~3-facet set — the paper argues one thesis, it is
  not an `A + B` pile-up (same rule as `schema/positioning.md`).
- Nothing enters the spine that the technical report does not support.
