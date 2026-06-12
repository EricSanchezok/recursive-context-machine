# Positioning Contract

Ground's core question is not "is this idea perfectly novel?" but "in front of the
real prior work, what is the most honest, forceful, verifiable **set of
contributions** this can be?". Positioning is reframe-before-reject: most early ideas
fail because the contributions are *framed* wrong, not because there is no value.

Written by `position` (and revised once by `revise`) into
`project_dir/12_positioning.md`.

## A strong paper carries a coherent contribution set (~3 facets)

A good paper is rarely one claim, and it is never an unrelated `A + B` pile-up. It
carries **about three contributions that are facets of one thesis** — they reinforce
each other rather than sit side by side. Aim for this structure:

1. `scope` — the **first-work / scoping claim**: the territory this work stakes out
   (e.g. "first to formalize / study / enable X"; a new problem, setting, or regime).
   The conceptual frame.
2. `artifact` — the **performance / community contribution**: the reusable thing the
   community builds on — the method, benchmark, dataset, framework, or analysis tool
   — and the capability or performance it delivers.
3. `results` — the **empirical contribution**: the strong, honest evidence that the
   scope claim holds and the artifact works.

The test of quality is **coherence, not count**: all facets must grow from the same
`core_thesis`. "We propose mechanism A *and*, unrelatedly, release dataset B" is the
failure mode (sprawl) even though it is technically three bullet points. Three facets
of one idea — a claim, the artifact that realises it, the evidence that proves it —
is what a top-venue paper actually looks like.

## Required sections

- `idea_ref`: the `idea_NNN` being grounded.
- `core_thesis`: the single sentence the whole paper hangs on — the spine every
  contribution is a facet of. This is what makes the set coherent rather than A+B.
- `contributions`: the coherent set (aim for 3), each:
  `{ facet: scope | artifact | results, claim: <one line>, differs_from: <closest
  work + the precise difference>, status: supported | planned | gap }`. Attempt all
  three facets. If a facet cannot be honestly filled yet, mark it `planned`/`gap` and
  say why — never fabricate a first-claim or a community artifact to fill a slot. Cut
  any "contribution" that does not share the `core_thesis` (that is A+B sprawl, not a
  third contribution).
- `contribution_type`: the **lead** facet's type — `new_method` | `new_problem` |
  `new_analysis` | `method_transfer` — used by the design stage for routing.
- `contribution_statement`: 4–8 sentences telling the contributions as ONE story: the
  `core_thesis`; the scope it stakes; the artifact that realises it; the evidence that
  proves it; what this enables that was not possible before; and the scope limits
  (when it holds, when it doesn't).
- `positioning_table`: 2–3 rows, one per closest work:
  `{ paper, overlap, our_difference, why_it_matters }`. Each difference concrete and
  verifiable against the actual paper (grounded in its `papers/<id>.md` card), not
  "more efficient".
- `reframes_tried`: **≥2** alternative framings attempted (a different lead facet or
  contribution type), each with a one-line outcome (chosen / rejected-because). This
  prevents premature rejection.
- `honest_strength`: `strong` | `moderate` | `weak`, per contribution and overall —
  the truthful calibration after positioning. A `weak` reading is allowed and should
  drive a reframe, not a hidden overclaim.

## Rules

- Aim for a **coherent ~3-facet contribution set**; the failure mode to reject is an
  *incoherent* pile-up (A+B padding), not "too many contributions". Cut a facet that
  does not share the `core_thesis` rather than staple it on.
- Each facet must be honestly supportable. Do not invent a "first to…" claim or a
  community artifact the work does not actually provide; mark honest gaps instead.
- Try at least two framings before concluding anything is unviable.
- Reject (route a pivot to explore) **only** when the `core_thesis` is fully scooped
  with no shrinkable contribution, when reframes leave only a trivial variant, or when
  the honest direction has drifted off the anchor. Otherwise reframe or narrow.
- Every difference must be checkable against a real paper card. No vague or invented
  differences.
