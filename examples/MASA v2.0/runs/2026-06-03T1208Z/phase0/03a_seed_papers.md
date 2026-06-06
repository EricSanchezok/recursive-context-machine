# Seed Papers — Automated Literature Survey Generation

- **run_dir**: `examples/MASA/runs/2026-06-03T1208Z`
- **source_agent**: `citation_seed_selector` (simulated)
- **generated_at**: 2026-06-03T20:08:00Z
- **seed_count**: 6

Selection criteria: dedicated survey-generation systems first (broadest relevance), then foundation RAG/multi-agent frameworks, then one evaluation benchmark.

---

## Selected Seeds

| # | arXiv ID | Title (abbreviated) | Year | Category | Selection Rationale |
|---|----------|---------------------|------|----------|---------------------|
| 1 | 2502.13965 | AutoSurvey: Large Language Model Powered Survey Generation | 2025 | foundation | Most direct match; end-to-end survey generation system with hierarchical outline and multi-source synthesis |
| 2 | 2308.08155 | STORM: Assisting in Writing Wikipedia-like Articles From Scratch with LLMs | 2023 | foundation | Pioneer automated article-writing system; outline-driven RAG with citation graph expansion |
| 3 | 2402.14207 | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | 2024 | foundation | RAG agent purpose-built for scientific literature; citation-grounded answers; strong evaluation framework |
| 4 | 2402.14829 | AutoGen: Enabling Next-Gen LLM Applications via Multi-Agent Conversation | 2024 | foundation | Leading multi-agent framework; enables collaborative writing/editor/reviewer agent patterns |
| 5 | 2404.16130 | Self-RAG: Learning to Retrieve, Generate, and Critique | 2024 | foundation | Learned retrieval and self-critique; directly applicable to survey factuality and citation quality |
| 6 | 2402.05680 | Evaluating LLM-Generated Scientific Surveys: Metrics and Benchmarks | 2024 | benchmark | Dedicated evaluation framework for automated surveys; coverage, citation, factuality dimensions |

---

## Omitted Considerations

- **Original RAG (2005.11401)**: Foundational but too generic; Self-RAG and RankRAG cover the survey-relevant retrieval advances more directly.
- **AutoSci (2406.03666)**: Multi-agent research framework covering literature review; slightly broader scope than survey generation.
- **ChatPaper (2406.18676)**: Paper analysis tool; valuable component but not a full survey generation system.

---

## Risks

1. **No pre-existing survey on automated survey generation exists** — this survey would be among the first dedicated to this topic.
2. **Simulated seeds** — seeds were manually selected based on agent knowledge, not through automated citation graph expansion.
3. **Recent (2025) papers** — some seeds lack established citation histories; forward expansion may yield limited results.
