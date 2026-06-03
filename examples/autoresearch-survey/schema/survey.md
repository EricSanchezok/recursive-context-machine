# Survey Contract

`Survey` is the final product: a self-contained, publishable survey article that
projects the research map and judge panel into flowing, logically argued prose.
It is the only long-form artifact in the pipeline; everything upstream stays
compact and auditable.

The survey is an evidence-constrained projection, not free writing — but it is a
*narrative*, not a list. A reader who finishes it should understand the field's
central problem, how the field got to where it is, what the main approaches are
and why they differ, and where the open frontiers lie.

## Self-contained document rule

The survey is written for an external reader who has never seen this pipeline.
It must read like a survey published in a venue, with no trace of the machinery
that produced it.

- Never name internal objects or stages: no "ResearchMap", "BenchmarkMatrix",
  "RelationGraph", "RankedPool", "JudgePanel", "CoverageJudge", "BenchmarkJudge",
  "GapJudge", "ScopeJudge", "run_dir", "candidate pool", "handoff", "pipeline".
- State the substance instead. Not "the BenchmarkJudge marked this not ready",
  but "these methods are evaluated on different benchmarks, so a direct
  comparison is not yet possible."
- Do not describe how the survey was assembled, retrieved, or judged.

## Narrative requirements

The article must argue, not enumerate. Required spine:

- `title` and a 4-6 sentence `abstract` that states the problem, the organizing
  thesis, and what the reader will take away.
- `global_picture` (when available): immediately after the abstract, embed the
  generated landscape figure `08_global_picture.png` with a relative-path image
  link and a sentence orienting the reader. Omit cleanly if the figure was not
  produced — never leave a broken link.
- `introduction`: define the field's central problem and why it matters; state
  the scope and intended reader; lay out the questions the survey answers and the
  organizing logic (the "story") the rest of the article follows.
- `evolution` / research arc: a connected narrative of how the field developed —
  the early approaches, the bottleneck each hit, what the next wave changed and
  why, up to the current frontier. This is the backbone that makes the survey a
  story rather than a catalog. Every transition needs a *reason* grounded in
  evidence (a benchmark appeared, a method exposed a limit, a need shifted).
- `taxonomy` sections, one per method family: for each family explain *why it
  exists* (what problem or insight defines it), walk through its representative
  work as a developing idea (not a bullet list of papers), and connect it to the
  families before and after it — how it improves on, reacts to, or trades off
  against them.
- `comparison`: synthesize across families with reasoning. Where methods are
  evaluated on shared benchmarks and settings, compare them and explain the
  trade-offs. Where they are not comparable, say so and explain why the field
  cannot yet answer the question.
- `open_problems`: the evidence-backed gaps, each argued from the limitations
  and contradictions the literature itself reveals — not speculation.
- `conclusion`: synthesize the arc into a mental model of the field and name the
  most consequential next directions.
- `references`: the cited papers, taken from the upstream artifacts — never invented.

## Evidence rules

- Obey the upstream `forbidden_overclaims` verbatim. Do not restate a banned claim.
- Prefer a caveat over a strong statement when evidence is abstract-only or a
  comparison is not benchmark-ready.
- If the upstream artifacts are incomplete, scope the survey honestly to what the
  evidence supports, and fold that limitation into the prose — do not pretend to
  coverage you do not have.
- Do not invent citations, results, or papers absent from the artifacts.
