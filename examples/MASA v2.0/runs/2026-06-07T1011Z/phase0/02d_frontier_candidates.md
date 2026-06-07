# Frontier Candidate Pool

**run_dir**: `.`
**source**: `01_query_plan.md` (F-01, F-02, CD-01, CD-02, BX-01, BX-02)
**generated**: 2026-06-07T10:14:30+08:00
**scout**: masa-frontier

---

## Summary

| Query ID | Description | Candidates | Boundary | Total |
|----------|-------------|-----------|----------|-------|
| F-01 | Live updating survey maintenance | 5 | 5 | 10 |
| F-02 | Interactive/steerable survey generation | 7 | 3 | 10 |
| CD-01 | Knowledge graph traversal for IR | 7 | 3 | 10 |
| CD-02 | Multi-agent orchestration for research | 6 | 4 | 10 |
| BX-01 | General RAG (boundary guard) | — | 10 | 10 |
| BX-02 | Traditional bibliometrics (boundary guard) | — | 10 | 10 |
| **Total (unique)** | | **~20** | **~35** | **60** |

---

## F-01 — Live updating and longitudinal survey maintenance with citation graphs

### In-scope candidates

| arXiv ID | Title | Year | Why |
|----------|-------|------|-----|
| **2502.00881v1** | **Toward Living Narrative Reviews: An Empirical Study of the Processes and Challenges in Updating Survey Articles in Computing Research** | 2025 | **Top hit.** Directly studies the process, incentives, and pain points of keeping survey articles current. Interviews with 11 researchers. |
| **1502.01329v2** | **A proposal for regularly updated review/survey articles: "Perpetual Reviews"** | 2015 | Seminal proposal for continuously updated surveys. Older but foundational framing. |
| 2110.06595v2 | Refcat: The Internet Archive Scholar Citation Graph | 2021 | Large-scale citation graph infrastructure enabling longitudinal survey updates. |
| 1805.02262v1 | Construction of the Literature Graph in Semantic Scholar | 2018 | Deployed citation graph system at scale (280M+ nodes). Relevant for survey maintenance infrastructure. |
| 2302.07302v1 | CiteSee: Augmenting Citations with Personalized Historical Context | 2023 | Adds persistent context around citations — relevant for longitudinal survey tools. |

### Boundary (out of scope for this dimension)

| arXiv ID | Title | Boundary reason |
|----------|-------|----------------|
| 2001.11500v2 | Who Should Google Scholar Update More Often? | Resource allocation for citation index updating, not survey content maintenance. |
| 1703.04222v2 | Scholia and scientometrics with Wikidata | Scientometrics tooling without LLM survey generation. |
| 1205.1143v1 | Recommendation on Academic Networks using Citation Analysis | Pure recommendation, no survey writing. |
| 1406.5572v1 | SurveyMan: Programming and Automatically Debugging Surveys | Human-subject surveys, not literature surveys. |
| 1404.5322v1 | CitNetExplorer: Citation network visualization | Analysis/visualization tool, not automated survey generation. |

---

## F-02 — Interactive and user-steerable automated survey generation

### In-scope candidates

| arXiv ID | Title | Year | Why |
|----------|-------|------|-----|
| **2504.08762v1** | **InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System** | 2025 | **Top hit.** Directly addresses user-steerable, interactive survey generation with customizable outline, reference categorization, and continuous refinement. |
| **2508.17647v1** | **SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models** | 2025 | Large-scale dataset (4,200 human surveys) + quality-aware generation pipeline. Evaluates semi-automatic vs fully automatic modes. |
| **2508.14317v1** | **SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing** | 2025 | Memory-guided generation with adaptive planning and fine-grained retrieval. Strong coherence results. |
| **2510.15624** | **Build Your Personalized Research Group: A Multiagent Framework for Continual and Interactive Science Automation** | 2025 | **Highly relevant.** Interactive multiagent framework with human-in-the-loop, dynamic workflows, and memory persistence across sessions. |
| 2507.17718v1 | AI Telephone Surveying: Automating Quantitative Data Collection | 2025 | Peripheral — voice-based survey interviewing, not literature surveys, but relevant for interactive paradigm. |
| 2501.05985v2 | Exploring LLMs for Automated Generation and Adaptation of Questionnaires | 2025 | LLM-driven questionnaire adaptation — adjacent to user-steerable generation. |
| 2401.12986v2 | Crowdsourced Adaptive Surveys | 2024 | Crowdsourced adaptive survey methodology — adjacent adaptive paradigm. |

### Boundary (out of scope for this dimension)

| arXiv ID | Title | Boundary reason |
|----------|-------|----------------|
| 2205.10977v2 | What should I Ask: Follow-up Questions in Conversational Surveys | Human-subject surveys, not literature surveys. |
| 2205.02370v2 | PREME: Preference-based Meeting Exploration | Meeting exploration, not literature surveys. |
| 2510.27126 | AURA: RL for Adaptive Conversational Surveys | Human-subject adaptive surveys, not literature surveys. |

---

## CD-01 — Knowledge graph traversal and expansion for information retrieval

### In-scope candidates

| arXiv ID | Title | Year | Why |
|----------|-------|------|-----|
| **2605.23753** | **SeedER: Seed-and-Expand Retrieval from Knowledge Graphs** | 2026 | **Top hit.** Iterative seed-and-expand framework using RL-trained graph-aware policy. Directly applicable to citation graph expansion. |
| **2601.13969** | **ARK: Autonomous Knowledge Graph Exploration with Adaptive Breadth-Depth Retrieval** | 2026 | Agentic KG retriever with adaptive breadth-depth tradeoff. Distillable to smaller models. Highly transferable to citation graphs. |
| **2504.02112v1** | **PolyG: Effective and Efficient GraphRAG with Adaptive Graph Traversal** | 2025 | Adaptive traversal strategy selection based on question type taxonomy. 75% win rate, 4x speedup over SOTA. |
| **2410.13765v2** | **Knowledge-Aware Query Expansion with LLMs for Textual and Relational Retrieval** | 2024 | Query expansion with KG document relations. Directly relevant to citation graph expansion. |
| 1911.03868v2 | Knowledge Guided Text Retrieval and Reading for Open Domain QA | 2019 | Graph-guided passage retrieval via KB traversal. Older but foundational. |
| 1910.03262v3 | CONVEX: Conversational QA over KGs with Judicious Context Expansion | 2019 | Unsupervised graph exploration for conversational QA. |
| 2212.05189v2 | Expanding Knowledge Graphs with Humans in the Loop | 2022 | Human-in-the-loop KG expansion with provable "human-friendliness". |

### Boundary (out of scope for this dimension)

| arXiv ID | Title | Boundary reason |
|----------|-------|----------------|
| 1310.5698v1 | Massive Query Expansion by Exploiting Graph Knowledge Bases | Pre-neural query expansion, no LLM component. |
| 1609.00464v2 | The Semantic Knowledge Graph | Infrastructure paper, no retrieval or traversal algorithm. |
| 1510.04780v2 | Graph Traversal Based QA over DBpedia | SPARQL-based QA, not differentiable/agentic retrieval. |

---

## CD-02 — Multi-agent orchestration frameworks for complex research tasks

### In-scope candidates

| arXiv ID | Title | Year | Why |
|----------|-------|------|-----|
| **2603.13327** | **DOVA: Deliberation-First Multi-Agent Orchestration for Autonomous Research Automation** | 2026 | **Top hit.** Multi-agent platform for complex research tasks with deliberation-first orchestration, hybrid collaborative reasoning, and adaptive tiered thinking. Directly applicable to survey pipelines. |
| **2603.03005** | **OrchMAS: Orchestrated Reasoning with Multi Collaborative Heterogeneous Scientific Expert Structured Agents** | 2026 | Scientific-domain multi-agent orchestration with dynamic pipeline construction and heterogeneous model integration. |
| **2510.15624** | **Build Your Personalized Research Group: A Multiagent Framework for Continual and Interactive Science Automation** | 2025 | Dynamic workflows, modular architecture, human-in-the-loop, memory persistence across sessions. |
| 2507.07257v2 | Open Source Planning & Control System with Language Agents for Autonomous Scientific Discovery | 2025 | 30-agent system for end-to-end scientific research automation. |
| 2509.20175v1 | Federation of Agents: Semantics-Aware Communication Fabric for Large-Scale Agentic AI | 2025 | Distributed orchestration with semantic routing and capability vectors. |
| 2410.21784v1 | MARCO: Multi-Agent Real-time Chat Orchestration | 2024 | Multi-agent orchestration with guardrails for robust task execution. |

### Boundary (out of scope for this dimension)

| arXiv ID | Title | Boundary reason |
|----------|-------|----------------|
| 2208.09099v3 | Scalable Multi-Agent Lab Framework for Lab Optimization | Physical lab automation, not knowledge work. |
| 2510.24937 | OrchVis: Hierarchical Multi-Agent Orchestration for Human Oversight | Visualization-focused, not research task execution. |
| 2510.02557v1 | Orchestrating Human-AI Teams: The Manager Agent | Team management, not research pipeline automation. |
| 2603.13327 | DOVA (second entry already listed above) | Duplicate entry in results. |

---

## Boundary Guards (BX-01, BX-02)

### BX-01 — General-purpose RAG systems (exclusion guard)

All 10 papers from this query fall cleanly into the boundary category. Key examples:

| arXiv ID | Title | Year | Why boundary |
|----------|-------|------|--------------|
| 2005.11401v4 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks (Lewis et al., 2020) | 2020 | Foundational RAG paper — general QA, no survey generation. |
| 2406.04744v2 | CRAG — Comprehensive RAG Benchmark | 2024 | General QA benchmark, no survey focus. |
| 2402.17497v2 | REAR: Relevance-Aware RAG for Open-Domain QA | 2024 | Open-domain QA relevance assessment. |
| 2412.07420v1 | RAG-based QA over Heterogeneous Data and Text | 2024 | Heterogeneous QA, no survey component. |

### BX-02 — Traditional bibliometric citation analysis (exclusion guard)

Most papers from this query are pre-LLM or bibliometric. However, some are cross-over candidates worth noting:

| arXiv ID | Title | Year | Why boundary / note |
|----------|-------|------|---------------------|
| 2008.13020v1 | A Decade of In-text Citation Analysis based on NLP/ML | 2020 | **Borderline.** Bibliometric survey but covers citation context analysis that could inform survey agents. |
| **2309.09727v1** | **When Large Language Models Meet Citation: A Survey** | 2023 | **Important cross-over.** Survey of LLM + citation applications (classification, summarization, recommendation). Relevant to MASA scope. Tag as cross-domain rather than pure boundary. |
| 2504.02767v1 | How Deep Do LLMs Internalize Scientific Literature and Citation Practices? | 2025 | **Important cross-over.** Studies LLM citation behaviour (Matthew effect, recency bias). Relevant to citation quality in survey generation. |
| 2409.02443v2 | Exploring Applicability of LLMs to Citation Context Analysis | 2024 | **Borderline.** LLM-assisted citation context analysis — adjacent to survey evidence aggregation. |
| 2510.25378 | Hallucinations in Bibliographic Recommendation: Citation Frequency as Proxy | 2025 | **Important cross-over.** Studies hallucination rates in LLM bibliographic output tied to citation frequency. Relevant to problem dimension (P-01). |
| 2602.01686 | Unmediated AI-Assisted Scholarly Citations | 2026 | LLM + DBLP integration for verified citations. Relevant to citation accuracy in surveys. |
| 2605.24351 | How Much Structure Do LLMs Need? Evaluating LLMs for Bibliometric Cluster Description | 2026 | Hybrid bibliometric+LLM pipeline for cluster descriptions. Adjacent to survey generation. |
| 1609.05354v3 | Citation Analysis with Microsoft Academic | 2016 | Pure bibliometrics, pre-LLM. Clean boundary. |
| 2403.18838v1 | AI-Enhanced Scientometrics, Webometrics, and Bibliometrics | 2024 | Survey of AI tools for bibliometrics — boundary but informative. |
| 2304.14516v1 | pyBibX: Bibliometric/Scientometric Analysis with AI Tools | 2023 | Tool paper. |

---

## Key Frontier Signals

The following papers represent the **strongest frontier signals** that push beyond current work and should be prioritised for full-text review:

1. **2502.00881v1** (Fok et al., 2025) — *Toward Living Narrative Reviews* — Only empirical study of the process and challenges of updating survey articles. Directly addresses F-01.

2. **2504.08762v1** (Wen et al., 2025) — *InteractiveSurvey* — First system to offer genuine user steerability during survey generation (outline, categorization, content refinement). Directly addresses F-02.

3. **2601.13969** (Polonuer et al., 2026) — *ARK: Adaptive Retriever of Knowledge* — Agentic KG exploration with adaptive breadth-depth tradeoff. Highly transferable to citation graph traversal. Addresses CD-01.

4. **2603.13327** (Shen & Shen, 2026) — *DOVA* — Deliberation-first multi-agent orchestration for autonomous research. Three-phase hybrid reasoning directly applicable to survey pipelines. Addresses CD-02.

5. **2309.09727v1** (Zhang et al., 2023) — *When LLMs Meet Citation: A Survey* — Bridges LLMs and citation analysis. Needed context for citation-aware survey generation.

6. **2504.02767v1** (Algaba et al., 2025) — *How Deep Do LLMs Internalize Scientific Literature* — Critical evidence on LLM citation bias (Matthew effect, recency) relevant to survey quality.

7. **2508.17647v1** (Bao et al., 2025) — *SurveyGen* — Large-scale dataset (4,200 surveys) enabling systematic evaluation of fully-automatic vs human-guided survey generation.

## Cross-Referenced Papers

Papers appearing in multiple queries (candidates for cross-dimensional analysis):

| arXiv ID | Appears in | Dimensional overlap |
|----------|-----------|-------------------|
| 2510.15624 | F-02, CD-02 | Interactive multiagent survey generation + research automation |
| 1406.5572v1 | F-01, F-02 | (Both boundary — human-subject SurveyMan) |
