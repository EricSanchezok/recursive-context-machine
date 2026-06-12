# Paper Contract

The paper is **compose Phase 2's deliverable**: a self-contained, claim-driven,
de-engineered manuscript for an external reader. It is a full rewrite built on the
PaperSpine — not the technical report with the engineering trimmed by hand.

Assembled into `project_dir/68_paper.md` from the per-section drafts in
`project_dir/sections/`.

## Structure (sections, in reading order)

- `abstract` (150–250 words): problem → gap → method (mechanism, not details) → key
  result (with number) → broader implication (what it MEANS, for whom). No citations,
  no undefined acronyms, no overclaim.
- `introduction`: follows the spine's `intro_arc` (field_assumption → pain → gap →
  insight → method → evidence → implication). Contributions list is the coherent ~3-facet
  set (scope / artifact / results), numbered, ≤ ~3 bullets. Never opens with "In recent
  years…".
- `related_work`: organised by approach, each block ending in positioning ("In contrast,
  we…"). Honest and specific.
- `method`: intuition before formalism; the idea, not the codebase. Mark novel vs reused.
- `experiments` / `results`: setup (datasets, baselines, metrics), then lead with the key
  finding, then ablations and analysis. **Every number traces to the technical report**
  (which traces to `results/`). Negatives reported honestly.
- `discussion`: main finding in context; a "Broader Perspective" paragraph (what
  assumption this challenges, what it enables, what new questions it opens); honest,
  specific limitations; concrete future work.
- `conclusion`: brief, forward-looking, no new claims, no overclaim.
- `references`: one deduplicated list; every citation real and accurately characterised.

## Quality bars (enforced by the polish node + the gate)

- **Claim-grounded**: every assertion maps to a `claim_NNN`; every strong claim has
  strong technical-report evidence; caveats from the claims appear in the prose.
- **Numerical integrity**: every number in the text matches the technical report and is
  internally consistent (abstract = results table; percentages recomputed).
- **5-pass prose**: clutter removed, active voice, sentences < ~40 words, keyword
  consistency (the Banana Rule — one term per concept), AI-isms removed
  (delve/pivotal/landscape/leverage/underscore/…).
- **Reverse-outline**: the first sentence of each paragraph, read in sequence, tells the
  whole story (problem → gap → method → evidence → conclusion).
- **De-engineered**: no raw config dumps, file paths, or infra notes in the main text —
  those live in the technical report / supplementary.

## Rules

- The paper is self-contained for an external reader: it never mentions this pipeline,
  its stages, the technical report, or internal artifact names.
- It claims nothing the technical report does not support. When in doubt, weaken the
  claim — never strengthen the prose past the evidence.
