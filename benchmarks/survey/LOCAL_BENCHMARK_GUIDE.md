# Local Benchmark Guide

This guide explains how to run the SurveyBench evaluation locally using surveys you generate from your own frontend or pipeline.

## 1. Prerequisites

- Python 3.10+
- Clone or open this repository at `c:\Users\朱汉文\RCM`
- Install evaluation dependencies:
  ```powershell
  cd benchmarks/survey
  pip install rank-bm25 openai datasets
  ```
- An evaluation API key (for LLM-as-judge or generation scoring). Examples below use DeepSeek; any OpenAI-compatible endpoint works.

## 2. SurveyBench Topics

Generate one survey for each topic you want to evaluate. The 20 official SurveyBench topics are:

| # | Topic Title |
|---|-------------|
| 0 | 3D Gaussian Splatting |
| 1 | 3D Object Detection in Autonomous Driving |
| 2 | Agentic Reinforcement Learning |
| 3 | Alignment of Large Language Models |
| 4 | Efficient Inference for Large Language Models |
| 5 | Evaluation of Large Language Models |
| 6 | Explainability for Large Language Models |
| 7 | Generative Diffusion Models |
| 8 | Graph Neural Networks |
| 9 | Hallucination in Large Language Models |
| 10 | Large Language Models for Recommendation |
| 11 | Large Language Models for Time Series |
| 12 | LLM-based Multi-Agent |
| 13 | Multimodal Large Language Models |
| 14 | Reinforcement Learning for Large Language Models |
| 15 | Retrieval-Augmented Generation for Large Language Models |
| 16 | Safety in Large Language Models |
| 17 | Scientific Large Language Models |
| 18 | Vision Transformers |
| 19 | Vision-Language-Action Models |

## 3. Where to Place Generated Surveys

Each generated survey must be saved as a Markdown file at the exact path:

```text
benchmarks/survey/generated/autoresearch/<topic_name>/output.md
```

`<topic_name>` is the topic title with spaces and special characters replaced by underscores (`_`). For example:

```text
generated/autoresearch/3D_Gaussian_Splatting/output.md
generated/autoresearch/LLM_based_Multi_Agent/output.md
generated/autoresearch/Retrieval_Augmented_Generation_for_Large_Language_Models/output.md
```

### Tips
- Use one directory per topic.
- The filename must be exactly `output.md`.
- Only include the rendered survey body; no YAML front matter or extra metadata is required.
- You can evaluate a single topic or all 20 topics at once.

## 4. Verify the Reference Data Is Available

Run the SurveyBench setup once to download the reference surveys and topic lists:

```powershell
cd benchmarks/survey
python run.py setup surveybench
```

After setup, confirm the topics load:

```powershell
python run.py list topics --benchmark surveybench
```

You should see the 20 topics listed above.

## 5. Run the Evaluation

### Option A: Evaluate a Single Topic

```powershell
cd benchmarks/survey
$env:EVA_API_KEY="sk-..."
$env:EVA_ENDPOINT="https://api.deepseek.com"
$env:EVA_MODEL="deepseek-v4-pro"
python run.py evaluate --pipeline autoresearch --benchmark surveybench --topics "3D Gaussian Splatting"
```

### Option B: Evaluate All 20 Topics

```powershell
cd benchmarks/survey
$env:EVA_API_KEY="sk-..."
$env:EVA_ENDPOINT="https://api.deepseek.com"
$env:EVA_MODEL="deepseek-v4-pro"
python run.py evaluate --pipeline autoresearch --benchmark surveybench
```

### Using Other Evaluation Providers

The framework is provider-agnostic. Change the endpoint and model name only:

| Provider | Endpoint | Model example |
|----------|----------|---------------|
| DeepSeek | `https://api.deepseek.com` | `deepseek-v4-pro` |
| Zhipu (智谱) | `https://open.bigmodel.cn/api/paas/v4` | `glm-5.2` |
| SiliconFlow | `https://api.siliconflow.cn/v1` | `deepseek-ai/DeepSeek-V3` |
| OpenRouter | `https://openrouter.ai/api/v1` | `openai/gpt-4o` |

## 6. View Results

Per-topic reports and a summary are written to:

```text
benchmarks/survey/results/autoresearch/surveybench/
```

Files include:

- `summary.md` — overall scores and ranking across all evaluated topics
- `<topic_name>/report.md` — detailed reasoning for a single topic
- `<topic_name>/report.json` — structured scores and rubric breakdowns

Example summary structure:

```text
results/autoresearch/surveybench/
├── summary.md
├── 3D_Gaussian_Splatting/
│   ├── report.md
│   └── report.json
├── LLM_based_Multi_Agent/
│   ├── report.md
│   └── report.json
└── ...
```

## 7. Scoring Dimensions

SurveyBench evaluates two aspects:

- **Content Score**: Compares the generated survey's structure and coverage against the reference survey (via LLM-as-judge and keyword overlap).
- **Quiz Score**: Measures how well the generated survey can support answers to a set of benchmark multiple-choice questions.

Final results show per-topic scores and the average across all evaluated topics.

## 8. Quick Checklist

- [ ] `pip install rank-bm25 openai datasets` completed
- [ ] `python run.py setup surveybench` ran successfully
- [ ] Generated surveys placed in `generated/autoresearch/<topic_name>/output.md`
- [ ] Evaluation API key set in `EVA_API_KEY`
- [ ] `EVA_ENDPOINT` and `EVA_MODEL` configured
- [ ] Ran `python run.py evaluate --pipeline autoresearch --benchmark surveybench`
- [ ] Checked `results/autoresearch/surveybench/summary.md`

## 9. Troubleshooting

- **Topic not found**: Make sure the directory name exactly matches the expected `<topic_name>` with underscores replacing spaces.
- **Missing output.md**: The evaluator looks specifically for `output.md` inside each topic directory.
- **Module not found**: Ensure you are running commands from the `benchmarks/survey` directory.
- **API errors**: Verify the key, endpoint, and model name. For Zhipu, use `https://open.bigmodel.cn/api/paas/v4`; for DeepSeek, `https://api.deepseek.com`.

## 10. Optional: DeepSurvey-Bench (Single Survey)

If you have a single survey that does not match one of the 20 SurveyBench topics, use the standalone script:

```powershell
cd benchmarks/survey
$env:EVA_API_KEY="sk-..."
$env:EVA_ENDPOINT="https://api.deepseek.com"
$env:EVA_MODEL="deepseek-v4-pro"
python run_single_eval.py
```

This script evaluates `output/survey.md` against the DeepSurvey-Bench LLM-as-judge rubric and prints Surface, Academic, and Overall scores directly to the terminal.
