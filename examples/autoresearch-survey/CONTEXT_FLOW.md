# Context Flow

This pipeline keeps RCM context small and treats files under `run_dir` as the source of truth.

Context is only for handoff:

- `run_dir`
- artifact paths
- small counts
- short caveats or representative IDs

Full data must be read from artifacts. Search results, PDF reference text, judge evidence, and merged tables should not be passed through RCM context.

## Handoff Envelope

Every node ends with a handoff in the shape defined by [`schema/handoff.md`](schema/handoff.md): `run_dir` first, then `artifact`, `status`, and only the small optional fields that apply (`counts`, `ids`, `verdict`, `risks`, `next`). Keep it to ~15 lines. The handoff names artifacts and signals; it never carries their contents.

## Step Contracts

| Step | Incoming context | Reads from disk | Writes to disk | Outgoing handoff |
|---|---|---|---|---|
| `anchor` | optional topic hint only | schema, env/topic file | `00_survey_spec.md` | `run_dir`, spec path, topic, 3 scope bullets |
| `query_plan` | `run_dir` and spec path | `00_survey_spec.md`, query schema | `01_query_plan.md` | `run_dir`, query plan path, counts by query type, representative query IDs |
| `method_scout` | `run_dir` | `01_query_plan.md`, candidate schema | `02a_method_candidates.md` | `run_dir`, artifact path, count, up to 5 IDs or titles |
| `benchmark_scout` | `run_dir` | `01_query_plan.md`, candidate schema | `02b_benchmark_candidates.md` | `run_dir`, artifact path, count, up to 5 benchmark names or IDs |
| `survey_scout` | `run_dir` | `01_query_plan.md`, candidate schema | `02c_survey_candidates.md` | `run_dir`, artifact path, count, up to 5 IDs or titles |
| `frontier_scout` | `run_dir` | `01_query_plan.md`, candidate schema | `02d_frontier_candidates.md` | `run_dir`, artifact path, count, up to 3 boundary risks |
| `discovery_merger` | scout handoffs only | `02a` through `02d` artifacts | `02_candidate_pool.md` | `run_dir`, candidate pool path, total count, role counts |
| `citation_seed_selector` | `run_dir` | `02_candidate_pool.md`, expansion schema | `03a_seed_papers.md` | `run_dir`, seed path, seed count, seed arXiv IDs |
| `reference_expander` | seed handoff only | `03a_seed_papers.md` | PDFs, `03b_citation_expansion.md` | `run_dir`, artifact path, PDF count, resolved and unresolved reference counts |
| `semantic_expander` | seed handoff only | `00`, `01`, `02`, `03a` artifacts | `03c_semantic_expansion.md` | `run_dir`, artifact path, new candidate count, query count, top drift risks |
| `expansion_merger` | expansion handoffs only | `03a`, `03b`, `03c` artifacts | `03_expansion.md` | `run_dir`, expansion path, seed/resolution/addition counts, drift risks |
| `rank_pool` | `run_dir` | `02_candidate_pool.md`, `03_expansion.md` | `04_ranked_pool.md` | `run_dir`, ranked pool path, core/supporting/boundary counts |
| `card_plan` | `run_dir` | `04_ranked_pool.md` | — (emits a JSON work list) | a JSON array of papers to read in full; its final message is consumed by `paper_cards` |
| `paper_cards` (map) | `card_plan`'s JSON list | each selected paper's PDF (full text) | `cards/<id>.md` per paper | gather digest of the per-paper handoffs |
| `research_map` | `run_dir` | `cards/`, `00` through `04` artifacts | `05_research_map.md` | `run_dir`, map path, taxonomy names, readiness counts |
| `coverage_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06a_coverage_judge.md` | `run_dir`, artifact path, verdict, top risks, queries |
| `scope_judge` | `run_dir` | `00`, `04`, `05` artifacts | `06b_scope_judge.md` | `run_dir`, artifact path, verdict, allowed claim types, forbidden overclaims |
| `benchmark_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06c_benchmark_judge.md` | `run_dir`, artifact path, verdict, ready and not-ready counts, queries |
| `gap_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06d_gap_judge.md` | `run_dir`, artifact path, verdict, strong and weak gap counts, queries |
| `judge_synthesizer` | judge handoffs only | `06a` through `06d` artifacts | `06_judge_panel.md` | `run_dir`, judge panel path, overall verdict, ready summary, caveats |
| `image_planner` | judge panel handoff only | `00`, `05`, `06` artifacts | `08_global_picture.png` | `run_dir`, image path, status |
| `survey_outline` | image handoff only | `05`, `06`, `cards/` | `00_outline.md` (+ JSON section list) | a JSON array of section specs; its final message is consumed by `section_expand` |
| `section_expand` (map) | `survey_outline`'s JSON list | each section's `cards/` | `sections/<n>_<slug>.md` per section | gather digest of the per-section handoffs |
| `survey_assembler` | section handoff | `00_outline.md`, `sections/*`, `08_global_picture.png` if present | `08_survey.md`, `index.md` | `run_dir`, survey path, status, plus the full survey for the user |
| `survey_writer_zh` | survey assembler handoff only | `08_survey.md` | `08_survey.zh.md` | `run_dir`, zh survey path, status |

`survey_brief` is no longer in the end-to-end pipeline. The writing stage is now three steps: `survey_outline` (a macro skeleton with a clear through-line, plus a JSON section list) → `section_expand` (a `map` that writes one detailed, card-grounded section per element) → `survey_assembler` (stitch the sections, embed the figure, and build one deduped reference list). The brief unit (`survey_brief.rcm`, reading the judge panel and writing `07_survey_brief.md`) is kept for standalone audit runs.

Depth comes from reading full text, not abstracts: `card_plan` selects the papers that matter, `paper_cards` reads each PDF in full — through the research anchor's lens, noting cross-domain transfer potential — and writes a compact card, and `section_expand` writes each section from those cards. No single node holds the whole diff of papers or the whole survey.

## Wiring Rules

- Sequential edges use `context last` handoff fluxes.
- Parallel scout and judge mergers use `context last`, not `digest`, because their final handoffs are intentionally compact.
- `map` nodes (`paper_cards`, `section_expand`) fan an inner accelerator out over a runtime list: the upstream node's final message is a JSON array (one element per item), `scatter = json` runs the inner once per element, and `gather = digest` rolls the per-item handoffs into one context. Each element carries `run_dir`; per-item outputs land on disk (`cards/`, `sections/`).
- Long evidence is moved through files, never through graph context.
- If a downstream unit needs more data than the handoff provides, that data belongs in a named artifact.
- Every handoff follows [`schema/handoff.md`](schema/handoff.md): `run_dir` first, ~15 lines max.
- `run_dir` is the chain invariant. Recover it from context; if a unit falls back to the newest `runs/*` directory, it must surface that as a `risk`, never switch silently.
