# Context Flow

This pipeline keeps RCM context small and treats files under `run_dir` as the source of truth.

Context is only for handoff:

- `run_dir`
- artifact paths
- small counts
- short caveats or representative IDs

Full data must be read from artifacts. Search results, PDF reference text, judge evidence, and merged tables should not be passed through RCM context.

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
| `research_map` | `run_dir` | `00` through `04` artifacts as needed | `05_research_map.md` | `run_dir`, map path, taxonomy names, readiness counts |
| `coverage_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06a_coverage_judge.md` | `run_dir`, artifact path, verdict, top risks, queries |
| `scope_judge` | `run_dir` | `00`, `04`, `05` artifacts | `06b_scope_judge.md` | `run_dir`, artifact path, verdict, allowed claim types, forbidden overclaims |
| `benchmark_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06c_benchmark_judge.md` | `run_dir`, artifact path, verdict, ready and not-ready counts, queries |
| `gap_judge` | `run_dir` | `05_research_map.md` and supporting artifacts | `06d_gap_judge.md` | `run_dir`, artifact path, verdict, strong and weak gap counts, queries |
| `judge_synthesizer` | judge handoffs only | `06a` through `06d` artifacts | `06_judge_panel.md` | `run_dir`, judge panel path, overall verdict, ready summary, caveats |
| `survey_brief` | judge panel handoff only | all final artifacts as needed | `07_survey_brief.md`, `index.md` | final brief plus `run_dir` |

## Wiring Rules

- Sequential edges use `context last` handoff fluxes.
- Parallel scout and judge mergers use `context last`, not `digest`, because their final handoffs are intentionally compact.
- Long evidence is moved through files, never through graph context.
- If a downstream unit needs more data than the handoff provides, that data belongs in a named artifact.
