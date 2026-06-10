# External Inputs Contract

The `design`, `realize`, and `experiment` stages are done **outside** this pipeline
(by the user / their tooling). `compose` therefore consumes a hand-in left under
`project_dir`. This contract defines what compose looks for and how it stays honest
when something is missing.

Compose never invents what is not here. A missing input becomes a stated gap and a
`partial`/`blocked` status — never a fabricated method detail or result number.

## What compose reads (any that exist)

- `project_dir/15_grounded_story.md` — the grounded StorySpine from the `ground`
  stage: the `paper_thesis` and the coherent ~3-facet contribution set
  (`scope` / `artifact` / `results`), `expected_main_claims`, `minimum_evidence`.
  This is the spine the paper is built around. (Optional if ground was not run; then
  compose derives a thesis from whatever method/experiment notes are provided and
  says so.)
- `project_dir/code/` — the implementation. Compose reads it to align the technical
  report's method description with **what was actually built** (key modules, the
  algorithm as coded, configs/hyperparameters). Aspirational design that is not in
  the code is marked as such.
- `project_dir/results/` — experiment outputs: metric files (json/csv), logs, and
  per-run result files. **Every number in the technical report must trace to a file
  here.** Includes negatives and ablations.
- `project_dir/method.md` *(optional)* — the external method spec / design notes
  (FormalMethodSpec-style: problem, mechanism, assumptions, complexity).
- `project_dir/experiments.md` *(optional)* — the external experiment matrix and
  setup: which experiments were run, configs, datasets, hardware, seeds.

## Honesty rules

- A result number with no backing file in `results/` is not reported as a result.
  State it as "not available" and lower the report's status.
- A method claim not reflected in `code/` is marked "designed, not in the provided
  code" — do not present it as implemented.
- If `results/` is empty or absent, the technical report's results section is a
  documented gap and the report gate verdict is `insufficient` (or `blocked`), so the
  paper phase does not build claims on nothing.

## Minimal hand-in for a useful run

At least one of `{code/, results/, method.md, experiments.md}` plus, ideally,
`15_grounded_story.md`. With only the grounded story and no code/results, compose can
produce a *structure-and-plan* technical report but must mark all empirical content as
pending — and the paper phase will be claim-light by design.
