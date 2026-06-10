# IdeaPool Contract

The `IdeaPool` is explore's deliverable: a *ranked* set of research candidates,
each with a full StorySpine and an honest novelty read, presented for the user's
taste-selection. It is not pre-filtered to one winner — keeping diverse candidates
alive is the point.

One file at `project_dir/06_idea_pool.md`.

## Structure

- `anchor`: one-line restatement of the project anchor (for drift audit).
- `ranking_method`: `batch_ranking` (rank the set together; do not hand-pick a
  single idea).
- `top_k`: how many candidates are surfaced for selection (typically 3–5).
- `candidates`: the ranked list. Each candidate:
  - `id`: `idea_NNN`.
  - `title`: 5–10 words.
  - `core_insight`: one sentence — the hidden variable or broken assumption.
  - `wonder_signal`: which signal (from `02_wonder.md`) it traces back to. **Required**
    — an idea with no wonder origin does not belong in the pool.
  - `anchor_alignment`: one sentence on how it serves the anchor. **Required.**
  - `story_spine`: the explore StorySpine (see `schema/story_spine.md`), including
    `≥2` `candidate_paper_angles`.
  - `contribution_type`: the primary type.
  - `decisive_test`: the smallest experiment that could kill the idea.
  - `simplest_baseline`: the obvious control that might explain the result away.
  - `null_result_teaches`: why the test is decision-relevant even if it fails.
  - `novelty`: `NOVEL` | `PARTIAL` | `SCOOPED` per-claim summary (from `04_novelty.md`).
  - `scores`: `novelty`, `feasibility`, `story_fit`, `anchor_alignment`, `risk`
    (1–10 each) — signals for ranking, not a single fabricated total.
  - `review_verdict`: `pass` | `revise` | `rethink` (from `05_review.md`) + the one
    key concern.
- `eliminated`: ideas dropped earlier, each with `reason` and the step it died at
  (kept for audit and as a banlist).

## Rules

- Rank the candidates together; never collapse to a single idea before the user's
  taste-selection.
- Drop, do not fake, an idea whose StorySpine has a generic field or whose
  `wonder_signal`/`anchor_alignment` cannot be written convincingly.
- Carry honest novelty: an idea with a SCOOPED core claim is either eliminated or
  marked for redesign — never presented as novel.
