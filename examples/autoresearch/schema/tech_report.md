# Technical Report Contract

The technical report is **compose Phase 1**: the complete, honest, engineering-grounded
record of what was built and what was run. It is *not* the paper — it is not
narrative-optimised, it hides nothing, and it is the single source of truth the paper
(Phase 2) draws its claims and numbers from.

Its two jobs are **rigor** and **alignment**: the method description must match the
actual `code/`, and every result must trace to a file in `results/`.

Written into `project_dir/53_tech_report.md`. Inputs per `schema/external_inputs.md`.

## Required sections

- `inputs`: what was available to read (the `code/` paths, the `results/` files, the
  grounded story, any `method.md`/`experiments.md`) and what was missing — the honesty
  anchor. (Mirrors `50_inputs.md`.)
- `problem_and_thesis`: the problem and the `paper_thesis` (from the grounded story if
  present), stated plainly.
- `method`: the method **as actually implemented**. Describe the algorithm, the key
  components, and how they work, cross-referenced to the real files/functions in
  `code/`. Mark anything that is designed-but-not-in-code explicitly. Include the
  configs/hyperparameters that the code actually uses.
- `experimental_setup`: datasets (with splits/stats), baselines, metrics, hardware,
  seeds, and the command/config that produced each run — as found in `results/` and
  `experiments.md`.
- `results`: **all** results, each with provenance `(metric = value, source =
  results/<file>)`. Include ablations, robustness, seeds, and **negative/weak
  results**. Report variance/CI where the data supports it. Never round away or invent;
  if a number is only in a log, cite the log.
- `analysis`: what the results mean technically — which design choices drove which
  outcomes, what the ablations isolate, where the method breaks.
- `limitations`: honest, specific, evidence-based limits (what was not tested, where it
  fails, what is assumed).
- `evidence_index`: a compact table mapping each key result → its `results/` file →
  the claim facet (`scope`/`artifact`/`results`) it could support. This is what Phase 2
  turns into the claim-evidence matrix.

## Rules

- Rigor over narrative: completeness and traceability matter more than flow here.
  Engineering detail belongs in this report (it gets *removed* in the paper, not added).
- Every number traces to a `results/` file; every method claim aligns to `code/`. State
  gaps; never fabricate to fill them.
- Negative and weak results are first-class — they bound the paper's claims later.
