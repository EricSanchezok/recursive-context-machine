# 02d_frontier_candidates.md

**run_dir**: `.`
**source**: `01_query_plan.md`
**generated**: 2026-06-06T12:35:18+08:00
**scout**: FrontierScout

---

## Overview

This file collects candidate papers from the 4 frontier and cross-domain queries
defined in QueryPlan. Each entry is marked with its query ID and a relevance
signal: **core** (directly relevant to survey generation), **supporting** (adjacent
method or framework), or **boundary** (likely out of scope — kept for conscious exclusion).

---

## F-01: Living and longitudinal literature survey updates with LLMs

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | **core** | Multi-stage pipeline + real-time retrieval of recent publications for topical freshness |
| 2 | 2502.18791v3 | Can LLMs Help Uncover Insights about LLMs? A Large-Scale, Evolving Literature Analysis of Frontier LLMs | **core** | Semi-automated + "automatically updatable dataset enables continuous tracking" — directly addresses longitudinal updates |
| 3 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | **core** | Interactive, allows user refinement during generation — relevant to living/steerable surveys |
| 4 | 2412.15249v2 | LitLLMs, LLMs for Literature Review: Are we there yet? | **supporting** | Decomposition of review into retrieval + planning, zero-shot evaluation protocol |
| 5 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | **supporting** | Multi-agent survey generation, 12-dimension evaluation |
| 6 | 2406.10252v2 | AutoSurvey: Large Language Models Can Automatically Write Surveys | **supporting** | Foundational automated survey system |
| 7 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | **supporting** | RAG toolkit for literature review generation |
| 8 | 2408.13450v1 | vitaLITy 2: Reviewing Academic Literature Using Large Language Models | **supporting** | RAG architecture for literature search and summarization |
| 9 | 2409.04600v1 | The emergence of Large Language Models (LLM) as a tool in literature reviews | **supporting** | Systematic review of LLM-based review automation |
| 10 | 2502.16868v1 | Graphy'our Data: Towards End-to-End Modeling, Exploring and Generating Report from Raw Data | **boundary** | End-to-end platform for document investigation; focuses on graph modeling rather than survey generation per se |

---

## F-02: User-steerable and interactive survey generation systems

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | **core** | Directly addresses user steerability: customize reference categorization, outline, content through intuitive interface |
| 2 | 2508.17647v1 | SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models | **core** | Compares fully automatic vs human-guided writing; semi-automatic pipelines achieve partially competitive outcomes |
| 3 | 2205.10977v2 | What should I Ask: A Knowledge-driven Approach for Follow-up Questions Generation in Conversational Surveys | **boundary** | Conversational surveys (human subjects), not literature surveys |
| 4 | 2401.12986v2 | Crowdsourced Adaptive Surveys | **boundary** | Public opinion surveys, not academic literature surveys |
| 5 | 1406.5572v1 | SurveyMan: Programming and Automatically Debugging Surveys | **boundary** | Pre-LLM survey debugging tool |
| 6 | 2510.27126 | AURA: A Reinforcement Learning Framework for AI-Driven Adaptive Conversational Surveys | **boundary** | Adaptive human-subject surveys, not literature surveys |
| 7 | 2003.02537v1 | Submitting surveys via a conversational interface | **boundary** | Human-subject survey UX |
| 8 | 2205.02370v2 | PREME: Preference-based Meeting Exploration through an Interactive Questionnaire | **boundary** | Meeting exploration, unrelated |
| 9 | 2512.08646 | QSTN: A Modular Framework for Robust Questionnaire Inference | **boundary** | Questionnaire generation for LLM-based surveys (human subjects) |
| 10 | 2503.09311v1 | Adaptive political surveys and GPT-4 | **boundary** | Political science surveys |

---

## CD-01: Multi-agent coordination frameworks for knowledge-intensive tasks

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2502.14743v2 | Multi-Agent Coordination across Diverse Applications: A Survey | **core** | Survey of multi-agent coordination; identifies LLM-based MAS as a promising direction |
| 2 | 2605.27466 | AgensFlow: A Coordination-Policy Substrate for Multi-Agent Systems | **supporting** | Learned routing for multi-agent coordination; evaluated on knowledge-intensive tasks |
| 3 | 2507.17061v1 | Parallelism Meets Adaptiveness: Scalable Documents Understanding in Multi-Agent LLM Systems | **supporting** | Dynamic task routing + bidirectional feedback for document understanding |
| 4 | 2502.07350v2 | KABB: Knowledge-Aware Bayesian Bandits for Dynamic Expert Coordination | **supporting** | Semantic understanding + dynamic adaptation for multi-agent expert selection |
| 5 | 2406.20041v3 | BMW Agents -- A Framework For Task Automation Through Multi-Agent Collaboration | **supporting** | Multi-agent framework for complex task automation |
| 6 | 2509.20175 | Federation of Agents: A Semantics-Aware Communication Fabric for Large-Scale Agentic AI | **supporting** | Semantic routing + dynamic task decomposition for agent coordination |
| 7 | 2404.11943v1 | AgentCoord: Visually Exploring Coordination Strategy for LLM-based Multi-Agent Collaboration | **supporting** | Visual exploration framework for designing coordination strategies |
| 8 | 2603.25268 | CRAFT: Grounded Multi-Agent Coordination Under Partial Information | **boundary** | 3D structure construction benchmark; tests pragmatic communication but not document-focused |
| 9 | 2603.22791 | ABSTRAL: Automatic Design of Multi-Agent Systems Through Iterative Refinement and Topology Optimization | **supporting** | Evolves MAS architecture as natural-language document through contrastive trace analysis |

---

## CD-02: Self-RAG and RankRAG retrieval mechanisms for factuality

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2310.11511v1 | Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection | **core** | Foundational Self-RAG paper; adaptive retrieval + reflection tokens for factuality |
| 2 | 2504.01018v1 | Self-Routing RAG: Binding Selective Retrieval with Knowledge Verbalization | **core** | LLM dynamically decides between external retrieval and parametric knowledge; reduces retrievals by 29% |
| 3 | 2406.13779v1 | FoRAG: Factuality-optimized Retrieval Augmented Generation for Web-enhanced Long-form Question Answering | **supporting** | Factuality optimization via doubly fine-grained RLHF for long-form QA |
| 4 | 2410.22954v4 | RA-RAG: Retrieval-Augmented Generation with Estimation of Source Reliability | **supporting** | Cross-checks source reliability across multiple sources for robust RAG |
| 5 | 2505.10792v2 | Finetune-RAG: Fine-Tuning Language Models to Resist Hallucination in Retrieval-Augmented Generation | **supporting** | Fine-tuning approach to resist hallucination under imperfect retrieval |
| 6 | 2406.13629v3 | InstructRAG: Instructing Retrieval-Augmented Generation via Self-Synthesized Rationales | **supporting** | Explicit denoising via self-synthesized rationales; improves accuracy by 8.3% |
| 7 | 2411.01022v1 | Provenance: A Light-weight Fact-checker for Retrieval Augmented LLM Generation Output | **supporting** | NLI-based factuality checker for RAG output |
| 8 | 2405.00175v1 | uRAG: Towards a Search Engine for Machines: Unified Ranking for Multiple Retrieval-Augmented Large Language Models | **boundary** | Infrastructure for serving multiple RAG systems, not directly about factuality |
| 9 | 2509.00100v1 | MODE: Mixture of Document Experts for RAG | **boundary** | Lightweight clustering-based retrieval for small corpora |
| 10 | 2005.11401v4 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | **supporting** | Foundational RAG paper — contextual but pre-dates Self-RAG/RankRAG era |

---

## Summary

| Query | Core | Supporting | Boundary | Total |
|-------|------|------------|----------|-------|
| F-01  | 3    | 6          | 1        | 10    |
| F-02  | 2    | 0          | 8        | 10    |
| CD-01 | 1    | 7          | 1        | 9     |
| CD-02 | 2    | 5          | 3        | 10    |
| **Total** | **8** | **18** | **13** | **39** |

Note: CD-01 returned 9 unique results (2509.20175 appeared as both v1 and plain ID; deduplicated).

---

## Candidates for downstream scouts

The following 26 papers (core + supporting) are recommended for downstream scouts:

- **AutoSurvey2** (2510.26012) — F-01 core
- **Evolving Literature Analysis** (2502.18791v3) — F-01 core
- **InteractiveSurvey** (2504.08762v1) — F-01 core, F-02 core
- **LitLLMs** (2412.15249v2) — F-01 supporting
- **Agentic AutoSurvey** (2509.18661v1) — F-01 supporting
- **AutoSurvey** (2406.10252v2) — F-01 supporting
- **LitLLM** (2402.01788v2) — F-01 supporting
- **vitaLITy 2** (2408.13450v1) — F-01 supporting
- **LLM in literature reviews survey** (2409.04600v1) — F-01 supporting
- **SurveyGen** (2508.17647v1) — F-02 core
- **Multi-Agent Coordination Survey** (2502.14743v2) — CD-01 core
- **AgensFlow** (2605.27466) — CD-01 supporting
- **Parallelism Meets Adaptiveness** (2507.17061v1) — CD-01 supporting
- **KABB** (2502.07350v2) — CD-01 supporting
- **BMW Agents** (2406.20041v3) — CD-01 supporting
- **Federation of Agents** (2509.20175) — CD-01 supporting
- **AgentCoord** (2404.11943v1) — CD-01 supporting
- **ABSTRAL** (2603.22791) — CD-01 supporting
- **Self-RAG** (2310.11511v1) — CD-02 core
- **Self-Routing RAG** (2504.01018v1) — CD-02 core
- **FoRAG** (2406.13779v1) — CD-02 supporting
- **RA-RAG** (2410.22954v4) — CD-02 supporting
- **Finetune-RAG** (2505.10792v2) — CD-02 supporting
- **InstructRAG** (2406.13629v3) — CD-02 supporting
- **Provenance** (2411.01022v1) — CD-02 supporting
- **RAG foundational** (2005.11401v4) — CD-02 supporting
