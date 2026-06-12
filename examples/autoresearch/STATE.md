# State & Context Flow

This pipeline keeps RCM context small and treats files under `project_dir` as the
source of truth. It is the autoresearch analogue of the survey example's context
contract, generalised across seven stages.

## The chain invariant: `project_dir`

Every stage operates on a single **`project_dir`** = `runs/<UTC timestamp>/`.

- The **explore** stage's `anchor` node is the *only* node that creates a new
  `project_dir`. It is created once and reused by every later stage.
- Every other node recovers `project_dir` from its incoming context. If (and only
  if) the context has none — e.g. you ran a later stage standalone — it may fall
  back to the newest `runs/*` directory and **must** surface that under `risks:`.
- `project_dir` is always written verbatim, relative to the example root (e.g.
  `runs/20260609T120600Z`). Never absolutise it, never prefix it.

## Context is only for handoff

Graph context carries a **handoff** (see `schema/handoff.md`): `project_dir`
first, then `artifact`, `status`, `verdict`, and the few small fields that apply
(`counts`, `ids`, `risks`, `next`). ≤15 lines. It names artifacts and signals; it
never carries their contents. Full data — search results, paper text, positioning
tables, experiment logs — lives in `project_dir` files and is read from there.

## `state.md` — the lean project state

`project_dir/state.md` is the one human-readable state file for the whole project.
It records:

- `anchor`: the research direction (frozen by `anchor`, refined only by the user).
- `stage`: the current stage.
- a **per-stage line**: `status` (`pending|ok|blocked`) + `verdict` + the date.
- `refs`: key pointers (selected idea id, plan, code commit, …).
- a short append-only `timeline`.

Each stage's `gate` node updates its own line in `state.md`. There is **no**
per-object YAML zoo — `state.md` plus the per-stage artifacts are the entire
state.

## Paper-card knowledge base (shared across stages)

`project_dir/papers/<arxiv_id>.md` is a growing knowledge base of **full-text
paper cards**, each read *through the research anchor's lens* (see
`schema/paper_card.md`). Depth comes from reading full text, not abstracts:

- **explore** seeds it (core papers behind the wonder signals),
- **ground** extends it (the 2–3 closest works, read in full),
- **design** reads it (novelty diff against the closest mechanisms).

Cards are produced by the shared `rcm/shared/paper_card.rcm` worker, fanned out
with `spawns` (one worker per paper, in parallel). Citation-graph expansion —
download a seed PDF, extract its References and arXiv IDs, resolve neighbours with
`arxiv_search` — is how the KB grows beyond keyword search.

## Gates are the hard constraints

Each stage ends with a `gate` node (see `schema/gate.md`). The gate:

1. reads the stage's required artifacts,
2. checks each hard rule for that stage (a per-rule pass/fail checklist),
3. emits `verdict: strong | acceptable | insufficient | blocked`,
4. writes `<NN>_<stage>_gate.md` and updates `state.md`.

- **Standalone run**: the verdict in the handoff tells you whether the stage
  succeeded and what the human checkpoint is.
- **End-to-end run** (`autoresearch.rcm`): a `condition` reads the verdict and
  stops the chain on `insufficient`/`blocked`. Because RCM graphs are acyclic,
  *iteration* happens inside a node's own loop, and *pivots* (going back a stage)
  are a human re-run of the earlier stage — not a graph cycle.

## Artifact map

Numeric prefixes keep artifacts ordered on disk. `cards`/`papers`, `pdfs`, and
`code` are directories.

| Stage | Reads | Writes (under `project_dir`) | Handoff verdict surfaces |
|---|---|---|---|
| explore | `--purpose` | `00_anchor.md`, `01_landscape.md`, `papers/*`, `02_wonder.md`, `03_ideas.md`, `04_novelty.md`, `05_review.md`, `06_idea_pool.md`, `07_explore_gate.md`, `state.md` | `taste_selection` (user picks 2–5 ideas) |
| ground | `00_anchor.md`, `06_idea_pool.md`, `state.md` | `10_ground_target.md`, `11_closest_work.md`, `papers/*`, `12_positioning.md`, `13_review.md`, `15_grounded_story.md`, `16_ground_gate.md` | `reasonableness_check` (angle reasonable?) |
| design · realize · experiment | *(external)* | the hand-in under `project_dir`: `code/`, `results/`, `method.md`, `experiments.md` (see `schema/external_inputs.md`) | — |
| compose_report (Phase 1) | `code/`, `results/`, `15_grounded_story.md`, notes | `50_inputs.md`, `51_method.md`, `52_results.md`, `53_tech_report.md`, `59_report_gate.md` | — |
| compose_paper (Phase 2) | `53_tech_report.md`, `15_grounded_story.md` | `60_paper_inputs.md`, `61_claims.md`, `62_spine.md`, `sections/*`, `68_paper.md`, `69_paper_gate.md` | — |
| review | `68_paper.md` (reviewers) + `53_tech_report.md` (auditor) | `70_review_inputs.md`, `71a–d_*.md`, `78_review.md`, `79_review_gate.md` | `submission_readiness` |

`design` / `realize` / `experiment` are done outside this pipeline; their outputs are
the hand-in `compose` consumes. `compose` is two independently-runnable phases
(`compose_report.rcm` → `compose_paper.rcm`, or `compose.rcm` for both); the technical
report is the rigorous source of truth, the paper is the claim-driven, de-engineered
rewrite. `explore`, `ground`, `compose`, and `review` are built; the numeric prefixes
keep all stages' artifacts ordered on disk.

## Wiring rules

- Sequential edges use `a.context -> b.context` + `a.done -> b.trigger`.
- Parallel scouts/judges write `flux.slot(i)`; the merge `flux` uses `mode = last`
  (their handoffs are already compact) and `arity = N`; `flux.out -> merger.context`.
- `map` fan-out is `{ spawns = ["Worker"] }`: the planner's LLM calls
  `spawn_Worker(items=[…], max_parallel=N)`, one item per paper/section; per-item
  outputs land on disk (`papers/`, `sections/`).
- Long evidence moves through files, never through context. If a downstream node
  needs more than the handoff provides, that data belongs in a named artifact.
