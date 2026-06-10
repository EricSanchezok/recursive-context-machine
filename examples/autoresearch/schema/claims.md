# Claim-Evidence Matrix Contract

The claims are the **load-bearing arguments of the paper** — the few assertions the
whole manuscript exists to make. Phase 2 (paper) is written to argue *these*, not to
recount the technical report. They are extracted from the technical report and tied to
the grounded contribution facets (`scope` / `artifact` / `results`).

Written into `project_dir/61_claims.md`.

## Structure

- `core_thesis`: the one sentence the paper hangs on (from the grounded story /
  technical report). Every claim is a facet of this.
- `claims`: the focused set — aim for **3–5, never more than ~6** (a focused paper, not
  a feature list). Each claim:
  - `id`: `claim_NNN`.
  - `facet`: `scope` | `artifact` | `results` — which contribution facet it advances.
  - `statement`: precise, falsifiable, specific. Numbers, conditions, comparisons. Not
    "significantly outperforms" — rather "on X, achieves A vs B for the strongest
    baseline, a C% relative gain (n seeds)".
  - `evidence`: the backing from the technical report — `{ result, source =
    results/<file> via 53_tech_report.md }`. A claim with no report-backed evidence is
    not a claim.
  - `strength`: `strong` | `moderate` | `weak`, and it must **match the evidence**:
    strong only with solid, multi-seed/effect-size evidence; partial/weak evidence →
    `moderate`/`weak` with caveats. Never upgrade strength to sound better.
  - `caveats`: the conditions and limits (especially for `moderate`/`weak`).
- `hierarchy`: the primary claim (usually the `scope` or lead `artifact` facet) and the
  supporting claims under it — so the paper has one spine, not parallel pile-ups.
- `dropped`: assertions considered but cut (overclaimed, unsupported, or off-thesis),
  with the reason. Demote borderline claims to "observations", not claims.

## Rules

- Every claim traces to technical-report evidence, which traces to a `results/` file.
  No evidence → not a claim (move to `dropped` or future work).
- Strength matches evidence; negatives/weak results are reflected as caveats or as
  honest `weak` claims, never hidden.
- Keep it focused: 3–5 coherent claims under one `core_thesis`. If there are more, merge
  or demote — contribution sprawl makes a weaker paper.
- Claims are facets of the thesis (scope / artifact / results), reinforcing one story —
  the same coherence rule as `schema/positioning.md`.
