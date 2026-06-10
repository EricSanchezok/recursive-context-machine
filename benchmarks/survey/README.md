# Survey Benchmark Framework

Generate surveys via RCM pipelines and evaluate them against academic survey/related-work benchmarks.

## Quick Start

```bash
cd benchmarks/survey
pip install rank-bm25 openai

# Set an API key for LLM-as-judge evaluation
export EVA_API_KEY=sk-...   # preferred, supports custom endpoints
# or
export OPENAI_API_KEY=sk-...

# Fetch benchmark reference data
python run.py setup surveybench

# Generate surveys for a topic
python run.py generate --pipeline autoresearch --benchmark surveybench \
  --topics "3D Gaussian Splatting"

# Evaluate the generated surveys
python run.py evaluate --pipeline autoresearch --benchmark surveybench

# Aggregate results into a summary report
python run.py report --pipeline autoresearch --benchmark surveybench
```

## Benchmarks

| Benchmark | Paper | Type | Topics | Reference Data |
|-----------|-------|------|--------|---------------|
| SurveyBench | [arXiv:2510.03120v2](https://arxiv.org/abs/2510.03120) | Full survey | 20 CS topics | Human-written surveys (GitHub) |
| DeepSurvey-Bench | [arXiv:2601.15307v1](https://arxiv.org/abs/2601.15307) | Full survey | LLM-as-Judge rubric | Prompt-only |
| DeepScholar-Bench | [arXiv:2508.20033v2](https://arxiv.org/abs/2508.20033) | Related work | 63 papers | HuggingFace |

- **SurveyBench**: Evaluates full surveys on content quality (6 dimensions), outline quality (3 dimensions), and quiz answerability. Reference human-written surveys from OpenDataBox/SurveyBench.
- **DeepSurvey-Bench**: Evaluates full surveys on surface quality (outline, content, references) and academic value (informational, scholarly communication, research guidance). No public test set — purely LLM-as-Judge.
- **DeepScholar-Bench**: Evaluates related-work sections on organization, nugget coverage, reference coverage, citation precision, verifiability, and document importance. Ground-truth from HuggingFace (deepscholar-bench/DeepScholarBench).

## Commands

### `python run.py list [benchmarks|pipelines|topics]`

```bash
# List available benchmarks and their setup status
python run.py list benchmarks

# List available pipeline configs (configs/*.toml)
python run.py list pipelines

# List topics for a specific benchmark (requires --benchmark)
python run.py list topics --benchmark surveybench
python run.py list topics --benchmark deepscholar
```

### `python run.py setup <benchmark>`

Fetches benchmark reference data. Supported values:

- `surveybench` — clones the SurveyBench repo as a git submodule (OpenDataBox/SurveyBench)
- `deepscholar` — downloads the DeepScholar-Bench dataset from HuggingFace (requires `pip install datasets`)
- `all` — runs both

```bash
python run.py setup surveybench
python run.py setup deepscholar
python run.py setup all
```

### `python run.py generate --pipeline <name> --benchmark <name> [--topics ...]`

Runs the RCM pipeline to generate surveys for each benchmark topic. Each topic is generated independently, and the final output is saved to `generated/<pipeline>/<topic>/output.md`.

Options:
- `--pipeline` — pipeline config name (without `.toml` extension)
- `--benchmark` — benchmark to generate for: `surveybench`, `deepscholar`
- `--topics` — optional topic name/ID filter (space-separated, supports partial match)
- `--max-workers` — parallel generation workers (default: 4)

```bash
# Generate for all SurveyBench topics
python run.py generate --pipeline autoresearch --benchmark surveybench

# Generate for specific topics only
python run.py generate --pipeline autoresearch --benchmark surveybench \
  --topics "3D Gaussian Splatting" "LLM Reasoning"

# Generate for specific DeepScholar paper IDs
python run.py generate --pipeline autoresearch --benchmark deepscholar \
  --topics arxiv_id_1 arxiv_id_2
```

### `python run.py evaluate --pipeline <name> --benchmark <name> [--topics ...]`

Evaluates generated surveys against the benchmark's metrics. Results are saved to `results/<pipeline>/<benchmark>/<topic>/report.json` and `report.md`.

Options:
- `--pipeline` — pipeline config name
- `--benchmark` — benchmark to evaluate against: `surveybench`, `deepsurvey`, `deepscholar`
- `--topics` — optional topic filter
- `--model` — override LLM judge model (default: `gpt-4o`)
- `--max-workers` — parallel evaluation workers (default: 4)

```bash
# Evaluate all generated surveys
python run.py evaluate --pipeline autoresearch --benchmark surveybench

# Use a custom judge model
python run.py evaluate --pipeline autoresearch --benchmark deepsurvey \
  --model gpt-4o-mini
```

### `python run.py report --pipeline <name> --benchmark <name> [--compare ...]`

Aggregates per-topic evaluation results into a Markdown summary table at `results/<pipeline>/<benchmark>/summary.md`.

Options:
- `--compare` — list of method names to compare against official baselines (not yet implemented; reserved for future use)

```bash
python run.py report --pipeline autoresearch --benchmark surveybench
```

## Pipeline Configs

Pipeline configurations are TOML files in `configs/`. Each config describes how RCM runs a survey generation pipeline.

### Included Pipelines

| Config | Description | Working Directory |
|--------|-------------|------------------|
| `autoresearch` | Coverage-guided evidence-constrained survey pipeline | `examples/autoresearch-survey` |
| `masav2` | Multi-Agent Survey Automation with iterative refinement | `examples/MASA v2.0` |
| `custom` | Template for custom pipelines | — |

### Config Format

```toml
[pipeline]
name = "AutoResearch Survey"
description = "Coverage-guided evidence-constrained survey pipeline"

[generation]
working_dir = "examples/autoresearch-survey"   # relative to repo root
entry_graph = "rcm/autoresearch_survey.rcm"     # RCM entry-point file
final_output = "08_survey.md"                    # file the pipeline writes
timeout_seconds = 7200                           # per-topic timeout
env = ["DEEPSEEK_API_KEY", "OPENAI_API_KEY"]    # env vars to forward

[output]
format = "markdown"                              # or "json"
target_name = "output.md"                        # saved to generated/<name>/<topic>/
copy_artifacts = false
```

### Adding Your Own Pipeline

1. Copy `configs/custom.toml` to a new name, e.g. `configs/my_pipeline.toml`
2. Set `entry_graph` to your RCM graph entry point and `final_output` to your pipeline's output file
3. Run with `--pipeline my_pipeline`

```bash
python run.py list pipelines          # should show "my_pipeline"
python run.py generate --pipeline my_pipeline --benchmark surveybench
```

## Evaluation Metrics

### SurveyBench

**Content Quality** (6 dimensions, weighted → score /5):

| Dimension | Weight | Description |
|-----------|--------|-------------|
| Coverage Breadth | 15% | Does the outline cover all major sub-areas? |
| Logical Coherence | 15% | Is the organization logical and progressive? |
| Synthesis Granularity | 25% | Depth of comparison and grouping across papers |
| Clarity of Insights | 25% | Original insights beyond paraphrasing |
| Reference Relevance | 10% | Are citations relevant and comprehensive? |
| Non-textual Elements | 10% | Use of figures, tables, structured comparisons |

**Outline Quality** (3 dimensions, weighted → score /5): coverage, relevance, structure.

**Quiz Answerability** (→ score /100): LLM-generated questions checked against survey via BM25 retrieval + LLM grading. Combines correctness (60%) and key-point coverage (40%).

### DeepSurvey-Bench

**Surface Quality** (40% weight):

| Dimension | Description |
|-----------|-------------|
| SQ1. Outline Quality | Organization, logical progression |
| SQ2. Content Quality | Accuracy, depth, coverage |
| SQ3. Reference Quality | Coverage, balance, recency |

**Academic Value** (60% weight):

| Dimension | Description |
|-----------|-------------|
| AV1. Informational Value | Accuracy, comprehensiveness |
| AV2. Scholarly Communication | Contextualization, balanced comparison |
| AV3. Research Guidance | Future directions, actionable suggestions |

Overall score = Surface × 0.4 + Academic × 0.6 (all /5).

### DeepScholar-Bench

The implementation covers 6 of the 7 dimensions from the paper protocol (RQ1/Relevance Rate is not implemented).

**Knowledge Synthesis** (40% weight):

| Metric | Range | Evaluator | Description |
|--------|-------|-----------|-------------|
| Organization (KS1) | 1–5 | `organization.py` | Structure, flow, coherence (LLM judge) |
| Nugget Coverage (KS2) | 0–100% | `nugget.py` | Coverage of ground-truth information nuggets (LLM judge) |

**Retrieval Quality** (30% weight):

| Metric | Range | Evaluator | Description |
|--------|-------|-----------|-------------|
| Document Importance (RQ2) | 0–100 | `document_importance.py` | Citation counts + venue tier via Semantic Scholar API |
| Reference Coverage (RQ3) | 0–100% | `reference.py` | Coverage of key reference titles (LLM judge) |

**Verifiability** (30% weight):

| Metric | Range | Evaluator | Description |
|--------|-------|-----------|-------------|
| Citation Precision (V1) | 0–100% | `reference.py` | LLM judge checks if each citation supports its claim |
| Claim Coverage (V2) | 0–100% | `verifiability.py` | % of claims verifiable via Semantic Scholar abstracts |

## Comparison with Official Baselines

`--compare` is reserved for computing official benchmark scores (e.g., ROUGE-L, BLEU, or leaderboard comparisons). Currently emits a placeholder:

```
## Comparison with Baselines

_Baseline comparison requires benchmark official baselines. Not yet implemented._
```

This will be populated once official baseline data is integrated.

## Limitations

- **DeepSurvey-Bench has no public test set** — evaluation is LLM-as-Judge only, with no reference surveys or ground-truth data. The `setup` command does not support `deepsurvey`.
- **Mock mode** — when neither `EVA_API_KEY` nor `OPENAI_API_KEY` is set, all evaluators return placeholder scores. A warning is printed at the start of evaluation.
- **Semantic Scholar API** — Document importance and verifiability evaluators query the Semantic Scholar API (public tier, no auth). Rate-limited to ~1 request per 3 seconds; batch evaluations on DeepScholar-Bench (63 papers × multi-citation surveys) can be slow.
- **Quiz evaluator** — requires `rank-bm25` for passage retrieval; falls back to keyword overlap without it.
- **DeepScholar setup** — requires `pip install datasets` for HuggingFace dataset download.
- **Contrastive evaluators** — `evaluate_reference_coverage` and `evaluate_citation_precision` pass an empty ground-truth list (`[]`) when the benchmark's important-citations data is unavailable, defaulting to 100% coverage.

## Env Vars

| Variable | Default | Purpose |
|----------|---------|---------|
| `EVA_API_KEY` | — | Primary LLM judge API key |
| `OPENAI_API_KEY` | — | Fallback LLM judge API key |
| `EVA_MODEL` | `gpt-4o` | LLM judge model override |
| `EVA_ENDPOINT` | `https://api.openai.com/v1` | Custom API endpoint |
