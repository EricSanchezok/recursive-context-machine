# RCM Auto Research Survey

这个 example 展示如何用 RCM 把自动文献综述拆成可复用、可调试的单元级 context machines。核心思路不是让 LLM 一上来就写长文，而是先构建可审计的 research map、由多个独立 judge 汇总成短小凝练的 survey brief，最后再把这份地图投影成一篇有证据约束的叙事长文 survey 交给用户。

## 设计原则

- 先建图，再写作：candidate pool、citation expansion、semantic expansion、research map 都落盘。
- 不做伪精确总分：ranker 和 judge 只输出结构化信号、风险、证据和建议。
- citation graph expansion 先基于 arXiv：下载 seed paper PDF，抽 References 和 arXiv ID，再用 Holos arXiv embedding search 解析相邻论文。
- 每个单元都是独立 `.rcm` 文件：可以单跑、复用，也可以由 end-to-end graph 串联。
- 每次运行写入新的 `runs/<utc timestamp>/`，不会覆盖旧结果。

## 输入 Topic

topic 就是整张图的 **purpose**：

- 用 CLI 的 `--purpose` 覆盖，或写在 `anchor.rcm` 的 `purpose` 字段里（默认是一个 smoke-test topic）。

purpose 通过 `input.purpose -> anchor.purpose` 注入到 anchor 节点。不再使用 `AUTORESEARCH_TOPIC` 环境变量或 `topic.md` 文件。

示例：

**从本 example 目录运行**（`cd` 进来）。prompt 里的 `schema/`、`runs/` 都是相对当前目录的路径；从这里跑还能让各节点读到本目录的 `AGENTS.md`、而不是仓库根的开发指南。

```sh
cd examples/autoresearch-survey
export DEEPSEEK_API_KEY=sk-...
export OPENAI_API_KEY=sk-...   # 可选，仅 image_planner 生成全景图时需要
../../target/release/accelerate run rcm/autoresearch_survey.rcm \
  --purpose "KV cache compression for long-context large language model inference" \
  --speed 0 --context
```

不带 `--purpose` 时，使用 `anchor.rcm` 里声明的默认 topic。未设置 `OPENAI_API_KEY` 时，全景图步骤会跳过，survey 仍照常生成（无开头插图）。

也可以单跑某个单元（同样从本目录）：

```sh
../../target/release/accelerate run rcm/anchor.rcm \
  --purpose "KV cache compression for long-context LLM inference" --speed 0 --context
../../target/release/accelerate run rcm/discovery.rcm --speed 0 --context
```

单跑中游单元时，它会优先读取上游 context 中的 `run_dir`；如果没有，就尝试使用最近一次 `runs/*` 目录。

## Pipeline

The context contract is documented in [CONTEXT_FLOW.md](CONTEXT_FLOW.md). In short, graph context only carries handoffs; full intermediate data lives in `run_dir` artifacts.

1. `anchor.rcm`  
   读取 topic，生成 `SurveySpec`，并创建 `run_dir`。

2. `query_plan.rcm`  
   把 `SurveySpec` 转成 typed query program。

3. `discovery.rcm`  
   并行运行 method、benchmark、survey、frontier scouts，产出初始 candidate pool。

4. `expansion.rcm`  
   执行 citation graph expansion 和 semantic neighbor expansion。

5. `rank_pool.rcm`  
   只列 signals，不给总分，筛出核心论文和边界论文。

6. `research_map.rcm`  
   生成 taxonomy、method families、benchmark matrix、comparison readiness、gap evidence。

7. `judge_panel.rcm`  
   并行 coverage、scope、benchmark、gap judges，最后汇总裁决。

8. `image_planner.rcm`  
   读 research map，调用 `image_gen` 工具（gpt-image-2）生成领域全景图 `08_global_picture.png`。需要 `OPENAI_API_KEY`；缺失时报 blocked，writer 会跳过插图继续。

9. `survey_writer.rcm`  
   把 research map 和 judge panel 投影成最终的分节叙事长文 survey，开头插入全景图，并打印给用户。

10. `survey_writer_zh.rcm`  
   接在 `survey_writer` 之后，把英文成稿 `08_survey.md` 忠实翻译成中文 `08_survey.zh.md`（旁支落盘，不改变图的正式输出仍是英文版）。

`survey_brief.rcm` 仍保留为一个可单跑的单元（生成凝练的可审计简报），但已不在 end-to-end 管线中；管线由 `judge_panel` 直接进入 `survey_writer`。

## Run Artifacts

每次运行应包含：

- `00_survey_spec.md`
- `01_query_plan.md`
- `02_candidate_pool.md`
- `03_expansion.md`
- `04_ranked_pool.md`
- `05_research_map.md`
- `06_judge_panel.md`
- `08_global_picture.png`（需要 `OPENAI_API_KEY`；缺失时跳过）
- `08_survey.md`
- `08_survey.zh.md`（中文版）
- `index.md`

这些文件是 runtime artifacts，默认不进入 git。（单跑 `survey_brief.rcm` 时会额外产出 `07_survey_brief.md`。）
