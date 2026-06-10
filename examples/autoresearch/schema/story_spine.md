# StorySpine Contract

A `StorySpine` is the paper-angle backbone of an idea. It is *state*, not writing
polish: an idea carries it from the moment it enters the top-K. The same object has
two versions — explore drafts it; ground grounds it against real prior work.

Story is not scored by grandeur. It is scored by: does it pressure a real
assumption, name a specific pain, carry a non-obvious insight, and imply a concrete
consequence if true — honestly, within reach of the evidence?

## Explore version (drafted in the idea pool)

- `field_assumption`: the default the field currently assumes.
- `pain_point`: the specific failure that assumption creates — concrete to a person
  and a setting, not "useful for many applications".
- `non_obvious_insight`: the surprising mechanism or connection — not "combine A and
  B", not the obvious post-survey conclusion.
- `why_now`: what changed (a new benchmark, capability, measurement) that makes this
  possible or urgent.
- `what_changes_if_true`: who updates what if the idea holds.
- `candidate_paper_angles`: **≥2**, each a different contribution `type`
  (`new_method` | `new_problem` | `new_analysis` | `method_transfer` | `diagnostic`)
  with a one-line `title_sketch` and `promise`.
- `story_risks`: how the story could collapse (e.g. into a small heuristic).

## Grounded version (added by ground) — `project_dir/15_grounded_story.md`

Keeps the explore fields, adds:

- `status: grounded`.
- `grounded_angle`: the chosen `type` + `title_sketch` + `paper_thesis` (the
  one-sentence claim an abstract would open with).
- `closest_work_positioning`: 2–3 rows of `{ paper, overlap, difference, why_it_matters }`
  against the closest prior work — each difference concrete and verifiable.
- `expected_main_claims`: the coherent contribution set (~3 facets — `scope` /
  `artifact` / `results`) the paper would make, each a facet of `paper_thesis`, not an
  unrelated pile-up. See the contribution model in `schema/positioning.md`.
- `minimum_evidence`: the smallest experiment set that would support each claim —
  especially the `results` facet and any performance part of the `artifact` facet.
- `reframe_history`: the contribution types tried and why the chosen one won.

## Rules

- Every field must be concrete. If a field reads as generic ("improves
  performance", "opens future work"), it is not done — sharpen or drop the idea.
- The grounded version must not claim novelty the closest-work positioning does not
  support. If the honest calibration is weak, say so and reframe the type.
