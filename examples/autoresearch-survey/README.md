# RCM Auto Research Survey

这个 example 展示如何用 RCM 把自动文献综述拆成可复用、可调试的单元级 context machines。核心思路不是让 LLM 一上来就写长文，而是先构建可审计的 research map、由多个独立 judge 汇总成短小凝练的 survey brief，最后再把这份地图投影成一篇有证据约束的叙事长文 survey 交给用户。

## 设计原则

- 先建图，再写作：candidate pool、citation expansion、semantic expansion、research map 都落盘。
- 不做伪精确总分：ranker 和 judge 只输出结构化信号、风险、证据和建议。
- citation graph expansion 先基于 arXiv：下载 seed paper PDF，抽 References 和 arXiv ID，再用 Holos arXiv embedding search 解析相邻论文。
- 每个单元都是独立 `.rcm` 文件：可以单跑、复用，也可以由 end-to-end graph 串联。
- 每次运行写入新的 `runs/<utc timestamp>/`，不会覆盖旧结果。

## 输入 Topic

入口从下面两个位置读取 topic，优先级从高到低：

1. 环境变量 `AUTORESEARCH_TOPIC`
2. 本地文件 `examples/autoresearch-survey/topic.md`

如果两者都不存在，系统会使用一个 smoke-test topic。

示例：

```sh
export DEEPSEEK_API_KEY=sk-...
export AUTORESEARCH_TOPIC="KV cache compression for long-context large language model inference"
cargo run --bin accelerate -- run examples/autoresearch-survey/rcm/autoresearch_survey.rcm --speed 0 --context
```

也可以单跑某个单元：

```sh
cargo run --bin accelerate -- run examples/autoresearch-survey/rcm/anchor.rcm --speed 0 --context
cargo run --bin accelerate -- run examples/autoresearch-survey/rcm/discovery.rcm --speed 0 --context
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

8. `survey_writer.rcm`  
   把 research map 和 judge panel 投影成最终的分节叙事长文 survey，并打印给用户。

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
- `08_survey.md`
- `index.md`

这些文件是 runtime artifacts，默认不进入 git。（单跑 `survey_brief.rcm` 时会额外产出 `07_survey_brief.md`。）
