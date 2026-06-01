# Survey Contract

`Survey` is the final narrative product: the research map and brief projected
into a readable, section-structured survey article. It is the only long-form
artifact in the pipeline; everything upstream stays compact and auditable.

The survey is an evidence-constrained projection, not free writing. Every claim
must trace back to a paper card, benchmark row, or gap already recorded in the
ResearchMap and gated by the JudgePanel.

Required sections:

- `title` and a 3-5 sentence `abstract`.
- `introduction`: scope, intended reader, and the anchor questions from the spec.
- `taxonomy`: one subsection per method family, with prose that explains what
  distinguishes the family and walks through its representative papers.
- `benchmarks`: the datasets, metrics, and evaluation settings that matter, and
  what they can and cannot measure.
- `comparison`: only comparisons the BenchmarkJudge marked ready. Everything
  else must be stated as not-yet-comparable, with the reason.
- `open_problems`: the evidence-backed gaps, each tied to its supporting evidence.
- `conclusion`: a short synthesis and the most useful next directions.
- `references`: the cited papers, taken from the ResearchMap — never invented.

Evidence rules:

- Obey the JudgePanel `forbidden_overclaims` verbatim. Do not restate any claim
  it banned.
- Prefer a caveat over a strong statement whenever evidence is abstract-only or
  the comparison is not benchmark-ready.
- If the run is `partial` (missing upstream artifacts), say so near the top and
  scope the survey to what the artifacts actually support.
- Do not invent citations, results, or papers that are absent from the artifacts.
