# Extended Survey Candidates

**run_dir**: `.`
**topic**: automated literature survey agents with citation graph expansion
**date**: 2026-06-06
**Source**: Extended survey-oriented queries (15 queries from 01b_query_plan_extended.md)

---

## Surveys / Reviews / Taxonomies / Tutorials

Papers that are themselves surveys, reviews, tutorials, or taxonomies discovered through the extended query plan. These fill gaps in the existing pool's coverage.

| # | arXiv ID | Year | Type | Title (abbreviated) | Source Query | Relevance |
|---|----------|------|------|---------------------|-------------|-----------|
| 1 | 2503.21460v1 | 2025 | **survey** | Large Language Model Agent: A Survey on Methodology, Applications and Challenges | esv-01 | Comprehensive survey of LLM agents organized around architecture, collaboration, and evolution. Covers the methodological foundations that survey agents inherit. |
| 2 | 2508.17281v1 | 2025 | **review** | From Language to Action: A Review of LLMs as Autonomous Agents and Tool Users | esv-01 | Structured review of LLM agents (single-agent, multi-agent, tool integration). Analyzes 68 datasets; covers reasoning, planning, memory. Relevant for agentic patterns in survey generation. |
| 3 | 2504.19678v1 | 2025 | **review** | From LLM Reasoning to Autonomous AI Agents: A Comprehensive Review | esv-01 | Side-by-side comparison of ~60 benchmarks (2019-2025); surveys AI-agent frameworks (2023-2025) with modular toolkits. Covers agent-to-agent protocols (ACP, MCP, A2A). |
| 4 | 2308.11432v7 | 2023 | **survey** | A Survey on Large Language Model based Autonomous Agents | esv-01 | Widely-cited survey proposing unified agent framework. Covers construction, applications (social science, natural science, engineering), and evaluation. Foundational reference for agent architectures. |
| 5 | 2503.05659v2 | 2025 | **survey** | A Survey of LLM Empowered Agents for Recommendation and Search | esv-01 | First systematic survey of LLM agents for information retrieval (recommendation + search). Directly relevant to the retrieval component of survey agents. |
| 6 | 2309.07864v3 | 2023 | **survey** | The Rise and Potential of Large Language Model Based Agents: A Survey | esv-01 | 3-component framework (brain, perception, action) for LLM agents. Covers single-agent, multi-agent, and human-agent cooperation. High citation count; foundational perspective. |
| 7 | 2503.23037v2 | 2025 | **survey** | Agentic Large Language Models, a survey | esv-01 | Organizes literature around three capabilities: reason, act, interact. Discusses retrieval enabling tool use, reflection improving collaboration. Research agenda included. |
| 8 | 2508.05668v3 | 2025 | **survey** | A Survey of LLM-based Deep Search Agents: Paradigm, Optimization, Evaluation, and Challenges | esv-01 | First systematic survey of deep search agents (OpenAI Deep Research, etc.). Covers architecture, optimization, evaluation. Directly fills the "deep research" paradigm gap. |
| 9 | 2201.01880v1 | 2022 | **meta-study** | Automatic Related Work Generation: A Meta Study | em-01 | Meta-study comparing related work generation formulations, datasets, approaches, evaluation. Covers pre-LLM and early LLM approaches. Fills the "pre-2020" gap through its literature tracing. |
| 10 | 2210.00105v1 | 2022 | **survey** | A Decade of Knowledge Graphs in Natural Language Processing: A Survey | emc-02 | Systematic analysis of 507 papers on KGs in NLP. Provides taxonomy of tasks and research streams. Relevant for KG-driven survey methods gap. |
| 11 | 2003.02320v6 | 2020 | **tutorial** | Knowledge Graphs | emc-02 | Comprehensive introduction to knowledge graphs — data models, query languages, schema, identity, context, extraction, quality. Foundational tutorial for understanding KG infrastructure used by survey systems. |
| 12 | 2002.00388v4 | 2020 | **survey** | A Survey on Knowledge Graphs: Representation, Acquisition and Applications | emc-02 | Broad survey of knowledge graphs covering representation learning, completion, temporal KGs, and applications. Less directly relevant but provides KG context. |
| 13 | 1806.00089v1 | 2018 | **methodology** | Cascading Citation Expansion | emc-03 | Introduces cascading citation expansion using Dimensions API, integrated in CiteSpace. Fills the pre-2020 cocitation/snowballing methodology gap. Important for citation traversal foundations. |
| 14 | 2402.08339v1 | 2024 | **methodology** | Interleaved snowballing: Reducing the workload of literature curators | emc-03 | Formal definition of snowballing method with an improved algorithm (LitBall tool). Directly relevant to the snowballing methodology gap — provides algorithmic formulation of backward/forward expansion. |
| 15 | 2010.03001v5 | 2020 | **review** | A Review on Fact Extraction and Verification | epr-01 | Survey of fact extraction and verification (FEVER task). Covers document retrieval, sentence selection, and veracity classification. Relevant for the citation hallucination detection gap. |
| 16 | 2605.07723 | 2026 | **audit** | LLM hallucinations in the wild: Large-scale evidence from non-existent citations | epr-01 | Large-scale audit of 111M references across 2.5M papers. Finds ~147K hallucinated citations in 2025 alone. Crucial evidence paper for citation hallucination gap — establishes real-world magnitude. |
| 17 | 2603.03299 | 2026 | **audit** | How LLMs Cite and Why It Matters: A Cross-Model Audit of Reference Fabrication | epr-01 | Audits 10 LLMs across 4 domains (69,557 citations). Hallucination rates span 11.4%-56.8%. Proves hallucination is prompt-induced, not intrinsic. Practical multi-model consensus filter (95.6% accuracy with 3+ models). |
| 18 | 2508.00838v1 | 2025 | **analysis** | The Attribution Crisis in LLM Search Results | epr-01 | Analysis of ~14,000 LMArena conversation logs. Documents three exploitation patterns (no search, no citation, high-volume low-credit). Relevant for understanding survey agent attribution failures. |
| 19 | 2204.04991v3 | 2022 | **survey/benchmark** | TRUE: Re-evaluating Factual Consistency Evaluation | epr-02 | Comprehensive survey and assessment of factual consistency metrics across 11 datasets. Standardized evaluation protocol. Already in broader awareness but fills factual consistency evaluation gap. |
| 20 | 2405.02228v3 | 2024 | **benchmark** | REASONS: Attribution in Scientific Literature — New Benchmark and Methods | epr-01 | New dataset for sentence-level citation attribution across 12 scientific domains. Metadata-augmented approach reduces hallucination rates by 42%. Directly relevant to citation quality evaluation. |
| 21 | 2110.12490v3 | 2021 | **tool/methodology** | Paperfetcher: A tool to automate handsearch for systematic reviews | emc-03 | Open-source Python package + web-app incorporating snowballing in both directions. First tool automating handsearch with high usability. Fills the systematic review methodology gap. |

---

## Benchmark / Evaluation Frameworks (Extended)

| # | arXiv ID | Year | Type | Title (abbreviated) | Source Query | Relevance |
|---|----------|------|------|---------------------|-------------|-----------|
| 22 | 2602.11238 | 2026 | **benchmark** | SurveyLens: A Research Discipline-Aware Benchmark | ebm-01 | First discipline-aware benchmark for ASG. 1,000 surveys across 10 disciplines. Dual-lens evaluation. Directly fills the "non-CS evaluation" gap. |
| 23 | 2508.15658v1 | 2025 | **benchmark** | SurGE: Benchmarking Computer Science Survey Generation | ebm-01 | Evaluation benchmark for CS survey generation with 1M-paper corpus. 4 dimensions: coverage, referencing, organization, content quality. |
| 24 | 2508.11310v1 | 2025 | **benchmark** | SGSimEval: Multifaceted and Similarity-Enhanced Benchmark for ASG Systems | ebm-01 | Evaluates outline, content, references. Combines LLM-based scoring with quantitative metrics. Introduces human preference metrics. |
| 25 | 2601.15307 | 2026 | **benchmark** | DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys | ebm-01 | Evaluates deep "academic value" (informational, scholarly communication, research guidance) beyond surface quality. Novel criteria. |
| 26 | 2512.02763 | 2025 | **benchmark** | SurveyEval: Towards Comprehensive Evaluation of LLM-Generated Academic Surveys | ebm-01 | 3-dimension evaluation (overall quality, outline coherence, reference accuracy) across 7 subjects. Augments LLM-as-a-Judge with human references. |
| 27 | 2506.01829v1 | 2025 | **benchmark** | CiteEval: Principle-Driven Citation Evaluation for Source Attribution | ebm-02 | Fine-grained citation evaluation framework. CiteBench multi-domain benchmark. Multi-faceted citation quality assessment. Directly fills citation quality evaluation gap. |
| 28 | 2602.16942 | 2026 | **benchmark** | SourceBench: Can AI Answers Reference Quality Web Sources? | ebm-02 | 8-metric framework for cited source quality. 100 real-world queries. Evaluates 8 LLMs + 3 AI search tools. Relevant for citation quality benchmarking. |
| 29 | 2407.12861v2 | 2024 | **benchmark** | CiteME: Can Language Models Accurately Cite Scientific Claims? | ebm-02, eby-02 | Benchmark evaluating LM citation attribution. LMs achieve 4.2-18.5% vs humans 69.7%. CiteAgent system bridges gap to 35.3%. Directly relevant to citation accuracy evaluation. |
| 30 | 2602.11685 | 2026 | **benchmark** | DRACO: Cross-Domain Benchmark for Deep Research Accuracy | ebm-02 | 4-dimension evaluation (accuracy, completeness/objectivity, presentation, citation quality) across 10 domains and 40 countries. Real-world Perplexity Deep Research tasks. |
| 31 | 2405.05583v2 | 2024 | **framework** | OpenFactCheck: Building, Benchmarking Customized Fact-Checking Systems | ebm-02 | Unified framework for building fact-checking systems + evaluate LLM factuality. Three modules: CUSTCHECKER, LLMEVAL, CHECKEREVAL. |

---

## Adjacent / Boundary Papers (Extended)

| # | arXiv ID | Year | Type | Title (abbreviated) | Source Query | Relevance |
|---|----------|------|------|---------------------|-------------|-----------|
| 32 | 2601.18207 | 2026 | **system** | PaperSearchQA: Learning to Search and Reason over Scientific Papers with RLVR | eby-01 | Trains search agents using RL with verifiable rewards over 16M biomedical abstracts. Demonstrates planning, reasoning, self-verification. Directly relevant to RL-based traversal gap. |
| 33 | 2509.05874v1 | 2025 | **method** | Learning to Construct Knowledge through Sparse Reference Selection with RL | eby-01 | Deep RL framework for sparse reference selection prioritizing papers under time/cost constraints. Evaluated on drug-gene discovery. Novel for survey retrieval prioritization. |
| 34 | 2205.15281v1 | 2022 | **method** | Learning Open Domain Multi-hop Search Using Reinforcement Learning | eby-01 | Actor-critic RL for multi-hop entity-relation search. Learns to direct IR resources to relevant corpus regions. Relevant for citation traversal pathfinding. |
| 35 | 2501.10120v2 | 2025 | **system** | PaSa: An LLM Agent for Comprehensive Academic Paper Search | eby-01, emc-04, em-02 | RL-optimized paper search agent. Autonomously invokes tools, reads papers, selects references. +37.78% recall over Google+GPT-4o. Fills the RL-for-paper-search gap. |
| 36 | 2004.14974v6 | 2020 | **benchmark** | SciFact: Fact or Fiction — Verifying Scientific Claims | eby-02 | Introduces scientific claim verification task + SciFact dataset (1.4K claims with evidence). Foundational for claim verification methods transferable to citation grounding. |
| 37 | 2305.18265v1 | 2023 | **benchmark** | Check-COVID: Fact-Checking COVID-19 News Claims with Scientific Evidence | eby-02 | 1,504 expert-annotated news claims paired with scientific evidence. Bridges everyday language claims with formal academic evidence. Relevant for cross-domain verification transfer. |
| 38 | 2407.10652v2 | 2024 | **tool** | Cutting Through the Clutter: LLMs for Efficient Filtration in SLRs | eby-03 | Open-source LLMSurver tool for literature filtration. Consensus scheme achieves >98.8% recall. Human-AI collaboration model. Fills the semi-automated screening gap. |
| 39 | 2510.06708 | 2025 | **tool** | AISysRev: LLM-based Tool for Title-abstract Screening | eby-03 | LLM-based screening tool with OpenRouter support. Identifies Easy Includes/Excludes vs Boundary cases needing human intervention. Practical approach to hybrid screening. |
| 40 | 2412.15247v1 | 2024 | **system** | Streamlining Systematic Reviews: A Novel Application of LLMs | eby-03 | LLM system covering both title/abstract and full-text screening. 99.5% exclusion rate, 95.5% reduction in manual screening time. Significant efficiency gains. |
| 41 | 2411.02451v2 | 2024 | **system** | High-performance automated abstract screening with LLM ensembles | eby-03 | Tests 6 LLMs + 66 ensembles on Cochrane Library reviews. Perfect sensitivity achievable with ensembles. Precision trade-off documented. Important for understanding screening automation limits. |

---

## Summary Statistics

| Query Source | Total Retrieved | Survey/Review/Taxonomy/Tutorial | System/Method/Tool/Benchmark | Out-of-Scope |
|--------------|----------------|-------------------------------|------------------------------|--------------|
| em-01 | 10 | 1 (meta-study) | 8 | 1 |
| em-02 | 10 | 0 | 9 | 1 |
| em-03 | 10 | 0 | 8 | 2 (v1/v2 dup) |
| emc-01 | 10 | 0 | 7 | 3 |
| emc-02 | 10 | 3 | 6 | 1 |
| emc-03 | 10 | 0 | 6 | 4 |
| emc-04 | 10 | 0 | 7 | 3 |
| epr-01 | 10 | 2 (audits) | 6 | 2 |
| epr-02 | 10 | 1 (survey) | 7 | 2 |
| ebm-01 | 10 | 0 | 7 | 3 |
| ebm-02 | 10 | 0 | 9 | 1 |
| esv-01 | 10 | 7 | 2 | 1 |
| eby-01 | 10 | 0 | 4 | 6 |
| eby-02 | 10 | 1 (review) | 6 | 3 |
| eby-03 | 10 | 0 | 6 | 4 |

**Total unique extended candidates**: 41 (after cross-query deduplication + overlap with existing pool)

**Of which, surveys/reviews/tutorials/taxonomies**: 12

**New benchmarks/evaluation frameworks**: 10

**Boundary/adjacent papers**: 10

---

## Gaps Addressed

| Gap (from 01b_query_plan_extended_analysis.md) | Filled By | Confidence |
|-----|-----------|------------|
| Pre-2020 cocitation/snowballing | Cascading Citation Expansion, Interleaved Snowballing, Paperfetcher | High |
| Knowledge-graph-driven survey methods | KG surveys, KG-EmpiRE, Scholarly KG methodology | Medium |
| Deep Research paradigm | Deep Search Agents survey, DRACO benchmark, SourceBench | High |
| RL-based citation traversal | PaperSearchQA, Sparse Reference Selection RL, PaSa | Medium |
| Broader citation hallucination | Hallucination audits, REASONS, Attribution Crisis analysis | High |
| Factual consistency evaluation | TRUE survey, PlainQAFact, ReFACT | High |
| Non-branded evaluation benchmarks | SurveyLens, SurGE, SGSimEval, DeepSurvey-Bench, SurveyEval | High |
| Human-in-the-loop SLR tools | LLMSurver, AISysRev, LLM ensembles | Medium |
| Scientific claim verification transfer | SciFact, Check-COVID, CiteME | Medium |
