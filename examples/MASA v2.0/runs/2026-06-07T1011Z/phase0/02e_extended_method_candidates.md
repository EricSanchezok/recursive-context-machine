# Extended Method & Mechanism Candidate Pool

**run_dir**: `.`
**generated**: 2026-06-07T10:24:00+08:00
**scout_agent**: masa-extended-method-scout
**source_plan**: 01b_query_plan_extended.md

**Total unique candidates**: 60 (after deduplication across 7 extended queries and against main pool)

---

## Candidate Entries

### EM-01: Interactive human-guided literature exploration & survey generation

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 1 | 1706.08094 | 2017 | Interactive Exploration and Discovery of Scientific Publications with PubVis | EM-01 | mechanism | Early interactive visualisation tool combining ML-based recommendations, full-text search, and personalised exploration |
| 2 | 2110.14060 | 2021 | Argo Scholar: Interactive Visual Exploration of Literature in Browsers | EM-01 | mechanism | Web-based visual exploration tool on Semantic Scholar's live 200M paper graph; interactive incremental exploration |
| 3 | 2110.06354 | 2021 | Tell Me How to Survey: Literature Review Made Simple with Automatic Reading Path Generation | EM-01 | mechanism | Reading Path Generation (RPG) task; SurveyBank dataset; graph-optimisation approach for prerequisite chains |
| 4 | 2010.04147 | 2020 | Automatic generation of reviews of scientific papers | EM-01 | method | BERT-based extractive summarisation pipeline with co-citation graph for key paper identification |
| 5 | 2302.06754 | 2023 | Relatedly: Scaffolding Literature Reviews with Existing Related Work Sections | EM-01 | mechanism | Scaffolding tool that dynamically re-ranks and highlights divergent vs redundant information across multiple related work sections |

*Overlaps with main pool excluded: InteractiveSurvey, LitLLM, SciLit, SurveyForge, Instruct LLMs Step by Step, IterSurvey, DimInd*

### EM-02: LLM agents for academic scholarly writing & synthesis

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 6 | 2409.13740 | 2024 | Language agents achieve superhuman synthesis of scientific knowledge (PaperQA2) | EM-02 | method | Frontier language agent matching/exceeding domain experts on literature search, summarisation, and contradiction detection; PaperQA2 system |
| 7 | 2501.10120 | 2025 | PaSa: An LLM Agent for Comprehensive Academic Paper Search | EM-02 | mechanism | RL-optimised paper search agent; AutoScholarQuery dataset; 37.78% recall improvement over Google+GPT-4o |
| 8 | 2403.09733 | 2024 | OverleafCopilot: Empowering Academic Writing in Overleaf with Large Language Models | EM-02 | mechanism | Bridge framework for LLM integration into Overleaf; agent command system for customisable writing assistants |
| 9 | 2506.18149 | 2025 | CoachGPT: A Scaffolding-based Academic Writing Assistant | EM-02 | mechanism | Scaffolding-based AI agent with sub-task decomposition from educator instructions; real-time feedback loop |
| 10 | 2505.11336 | 2025 | XtraGPT: Context-Aware and Controllable Academic Paper Revision via Human-AI Collaboration | EM-02 | mechanism | Human-AI collaboration framework for academic paper revision; 7,040 paper dataset with 140K instruction-response pairs |
| 11 | 2603.14629 | 2026 | ResearchPilot: A Local-First Multi-Agent System for Literature Synthesis and Related Work Drafting | EM-02 | method | Local-first multi-agent system combining DSPy, SQLite, Qdrant for citation-aware related-work drafting |
| 12 | 2404.07738 | 2024 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature with Large Language Models | EM-02 | method | Idea generation agent with collaborative LLM-based reviewing agents; iterative refinement via human preference-aligned evaluation |
| 13 | 2312.07559 | 2023 | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | EM-02 | method | RAG agent for scientific QA across full-text articles; LitQA benchmark; matches expert human performance |
| 14 | 2311.12315 | 2023 | AcademicGPT: Empowering Academic Research | EM-02 | method | Domain-specific LLM (LLaMA2-70B) continual-trained on academic papers; applications for paper reading, review, title/abstract generation |
| 15 | 2411.14199 | 2024 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | EM-02 | method | Specialised RAG-LM over 45M open-access papers; ScholarQABench; GPT-4o outperformed on correctness despite smaller size |

*Overlaps with main pool excluded: ScholarCopilot, SPAR*

### EM-03: LLM agents with tool use for scientific research

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 16 | 2502.11705 | 2025 | LLM Agents Making Agent Tools (ToolMaker) | EM-03 | mechanism | Agentic framework autonomously transforming papers-with-code into LLM-compatible tools; 80% task success vs SOTA SE agents |
| 17 | 2603.00084 | 2026 | DeepXiv-SDK: An Agentic Data Interface for Scientific Literature | EM-03 | mechanism | Three-layer agentic data interface (Data/Service/Application) for structured scientific literature access; CLI, MCP, Python SDK |
| 18 | 2402.11451 | 2024 | SciAgent: Tool-augmented Language Models for Scientific Reasoning | EM-03 | mechanism | Tool-augmented scientific reasoning; MathFunc corpus (30K samples, 6K tools); SciToolBench benchmark across 5 domains |
| 19 | 2503.24047 | 2025 | Towards Scientific Intelligence: A Survey of LLM-based Scientific Agents | EM-03 | survey | Survey of scientific agent architectures, benchmarks, applications; distinguishes scientific agents from general-purpose agents |
| 20 | 2307.16789 | 2023 | ToolLLM: Facilitating Large Language Models to Master 16000+ Real-world APIs | EM-03 | mechanism | Large-scale tool-use framework; ToolBench dataset (16K+ APIs); DFS-based decision tree for multi-tool tasks |
| 21 | 2406.05804 | 2024 | A Review of Prominent Paradigms for LLM-Based Agents: Tool Use (Including RAG), Planning, and Feedback Learning | EM-03 | survey | Unified taxonomy of LLM agent paradigms; covers tool use, planning, and feedback learning for general agent frameworks |
| 22 | 2407.21778 | 2024 | Tulip Agent — Enabling LLM-Based Agents to Solve Tasks Using Large Tool Libraries | EM-03 | mechanism | Recursive tool search architecture avoiding full prompt embedding; significant reduction in inference costs for large tool libraries |
| 23 | 2401.06201 | 2024 | EASYTOOL: Enhancing LLM-based Agents with Concise Tool Instruction | EM-03 | mechanism | Framework transforming diverse tool documentation into unified concise tool instructions for LLM-based agents |
| 24 | 2502.04644 | 2025 | Agentic Reasoning: A Streamlined Framework for Enhancing LLM Reasoning with Agentic Tools | EM-03 | mechanism | Mind-Map agent for structured knowledge graph of reasoning context; integrates web search, code execution, structured memory |
| 25 | 2503.23037 | 2025 | Agentic Large Language Models, a survey | EM-03 | survey | Comprehensive survey of agentic LLMs organised by reasoning, action, and interaction categories |

### EM-04: Seed selection & traversal strategies for citation graph mining

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 26 | 2403.09295 | 2024 | Seed-based information retrieval in networks of research publications: Evaluation of direct citations, bibliographic coupling, co-citations and PubMed related article score | EM-04 | mechanism | Direct comparison of seed-based retrieval strategies (direct citation, co-citation, bibliographic coupling) using systematic reviews as baseline |
| 27 | 1404.5322 | 2014 | CitNetExplorer: A new software tool for analyzing and visualizing citation networks | EM-04 | mechanism | Citation network analysis tool for studying field development, literature delineation, and literature reviewing support |
| 28 | 1507.01388 | 2015 | Time and Citation Networks | EM-04 | theory | Causally-aware citation network analysis; temporal constraints distinguishing citation networks from other graphs |
| 29 | 1511.07643 | 2015 | Homophily and missing links in citation networks | EM-04 | theory | Similarity-based citation prediction; missing link identification for barrier analysis in knowledge transfer |

*Overlaps with main pool excluded: PUREsuggest, Oignon, Dir-Aware Citation Analysis, SemScholar Lit Graph, Non-obvious Papers, CITEX, CiteSee, Refcat*

### EM-05: Citation intent & citation function classification methods

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 30 | 1904.01608 | 2019 | Structural Scaffolds for Citation Intent Classification in Scientific Publications (SciCite) | EM-05 | mechanism | Multitask model incorporating structural information of scientific papers for citation intent; SciCite dataset (5× larger than ACL-ARC) |
| 31 | 2304.12730 | 2023 | CitePrompt: Using Prompts to Identify Citation Intent in Scientific Papers | EM-05 | mechanism | Prompt-based learning for citation intent classification; SOTA on ACL-ARC; first zero-shot/few-shot exploration |
| 32 | 2104.08087 | 2021 | Citations are not opinions: a corpus linguistics approach to understanding how citations are made | EM-05 | theory | Corpus linguistics analysis of 2M citations from scite dataset; finds low correlation between citation type and sentiment |
| 33 | 1609.00435 | 2016 | Citation Classification for Behavioral Analysis of a Scientific Field | EM-05 | mechanism | Large-scale behavioral study of citations in NLP; 2K annotated citations for function and centrality; ACL Reference Corpus |
| 34 | 2107.00414 | 2021 | MultiCite: Modeling realistic citations requires moving beyond the single-sentence single-label setting | EM-05 | benchmark | Multi-sentence, multi-label citation context dataset; 12,653 citation contexts from 1,200+ papers; challenges single-sentence assumption |
| 35 | 1910.03498 | 2019 | SentiCite: An Approach for Publication Sentiment Analysis | EM-05 | mechanism | Citation sentiment analysis system; SentiCiteDB and IntentCiteDB datasets (~2,600 citations); detects nature of citations (dataset ref, reading ref) |
| 36 | 1710.03094 | 2017 | Characterizing in-text citations in scientific articles: A large-scale analysis | EM-05 | theory | Large-scale analysis of 5M+ full-text articles characterising in-text citation patterns by time, textual progression, and scientific field |
| 37 | 2501.18292 | 2025 | Citation Recommendation based on Argumentative Zoning of User Queries | EM-05 | mechanism | Multi-task learning model combining citation recommendation and argumentative zoning; new annotation schema on PubMed Central |
| 38 | 2009.08948 | 2020 | A New Citation Recommendation Strategy Based on Term Functions in Related Studies Section | EM-05 | mechanism | Nine term functions for citation context; BM25+Word2vec with term function enhancement outperforms baselines for citation recommendation |
| 39 | 1811.07351 | 2018 | Neural Multi-Task Learning for Citation Function and Provenance | EM-05 | mechanism | CNN-based joint training for citation function and provenance; demonstrates synergistic relationship between the two tasks |

### EM-06: Systematic review automation & screening tools

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 40 | 2202.10033 | 2022 | An open-source integrated framework for the automation of citation collection and screening in systematic reviews | EM-06 | mechanism | Bayesian active learning citation screening; 95.6% efficiency with 100% sensitivity; automated query generation from labelled datasets |
| 41 | 2510.26750 | 2025 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | EM-06 | method | Semi-automated SLR tool with iterative snowballing, human-in-the-loop filtering, LLM-based analysis of articles |
| 42 | 2510.06708 | 2025 | AISysRev — LLM-based Tool for Title-abstract Screening | EM-06 | mechanism | LLM-based title-abstract screening tool; four-category classification (Easy Include/Exclude, Boundary Include/Exclude) |
| 43 | 2006.12166 | 2020 | Open Source Software for Efficient and Transparent Reviews (ASReview) | EM-06 | method | Active learning pipeline for screening; widely-used open-source tool; simulation studies demonstrating far more efficient reviewing |
| 44 | 2110.12490 | 2021 | Paperfetcher: A tool to automate handsearch for systematic reviews | EM-06 | mechanism | Automated handsearch tool with bidirectional snowballing; free open-source Python package and web-app |
| 45 | 2309.01684 | 2023 | CRUISE-Screening: Living Literature Reviews Toolbox | EM-06 | method | Web-based living literature review tool with API-connected periodic search updates; text classification and QA models for screening |
| 46 | 2011.09752 | 2020 | From Protocol to Screening: A Hybrid Learning Approach for Technology-Assisted Systematic Literature Reviews | EM-06 | mechanism | Full pipeline from research protocol to screening; learning-to-rank + relevance feedback; CLEF 2019 eHealth dataset |
| 47 | 2402.08565 | 2024 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | EM-06 | survey | Survey comparing 21 SLR tools across 23 traditional + 11 AI features; analysis of 11 LLM-based tools for literature search and writing |
| 48 | 2502.03400 | 2025 | DenseReviewer: A Screening Prioritisation Tool for Systematic Review based on Dense Retrieval | EM-06 | mechanism | Dense retrieval-based screening prioritisation; outperforms active learning methods; web-based tool and Python library |
| 49 | 2402.05317 | 2024 | Emerging Results on Automated Support for Searching and Selecting Evidence for Systematic Literature Review Updates | EM-06 | mechanism | Automated snowballing + ML-based evidence selection for SLR updates; 74% recall with Linear SVM; 2.5× reduction in manual effort |

### EM-07: Contradiction & conflict detection across scientific papers

| # | arXiv ID | Year | Title | Source Query | Likely Role | Inclusion Reason |
|---|----------|------|-------|-------------|-------------|-----------------|
| 50 | 2310.18685 | 2023 | When Reviewers Lock Horn: Finding Disagreement in Scientific Peer Reviews (ContraSciView) | EM-07 | mechanism | First automated identification of contradictions among peer reviewers; ContraSciView dataset (8.5K papers, 28K review pairs) |
| 51 | 2605.10171 | 2026 | When Reviews Disagree: Fine-Grained Contradiction Analysis in Scientific Peer Reviews | EM-07 | mechanism | Fine-grained contradiction analysis with graded intensity; RevCI benchmark; IMPACT multi-agent framework; TIDE distilled model |
| 52 | 2212.09867 | 2022 | Detecting Contradictory COVID-19 Drug Efficacy Claims from Biomedical Literature | EM-07 | mechanism | NLI-based contradictory claim detection in biomedical literature; expert-created NLI dataset; case study on remdesivir and hydroxychloroquine |
| 53 | 2111.08543 | 2021 | WikiContradiction: Detecting Self-Contradiction Articles on Wikipedia | EM-07 | mechanism | Pairwise Contradiction Neural Network (PCNN) for self-contradiction detection; pre-trained on SNLI/MNLI |
| 54 | 1708.00850 | 2017 | Towards Semantic Modeling of Contradictions and Disagreements: A Case Study of Medical Guidelines | EM-07 | theory | Formal distinction between contradictions and disagreements; NLP system for finding contradictory medical guidelines |
| 55 | 2303.04219 | 2023 | Discovering substantive disagreement with review articles? | EM-07 | theory | Analysis of whether review articles are more time-efficient for identifying disagreements than non-review ones |
| 56 | 2311.09182 | 2023 | ContraDoc: Understanding Self-Contradictions in Documents with Large Language Models | EM-07 | benchmark | First human-annotated dataset for self-contradiction detection across long documents; evaluation of GPT3.5, GPT4, PaLM2, LLaMAv2 |
| 57 | 2506.23990 | 2025 | Machine Understanding of Scientific Language | EM-07 | survey | Thesis covering scientific fact checking, adversarial claim generation, zero-shot scientific fact-checking, and exaggerated claim detection |
| 58 | 1406.1143 | 2014 | Identifying Duplicate and Contradictory Information in Wikipedia | EM-07 | mechanism | Minhash-based near-duplicate detection for Wikipedia sentences; identifies copied content and contradictory facts |
| 59 | 2004.14974 | 2020 | Fact or Fiction: Verifying Scientific Claims (SciFact) | EM-07 | benchmark | Scientific claim verification task; SciFact dataset (1.4K claims); supports/refutes classification with rationale extraction |
| 60 | 2502.11705 | 2025 | LLM Agents Making Agent Tools (ToolMaker) | EM-03, EM-07 | mechanism | *Duplicate of #16 above* |

---

## Deduplication Summary

| Metric | Count |
|--------|-------|
| Total raw results (all 7 queries) | 76 |
| Overlap with main pool (02a_method_candidates.md) | 16 |
| Cross-query duplicates within extended pool | 1 |
| **Unique new candidates** | **59** |
| Unique candidates including duplicates within extended | 60 |

### Papers excluded due to main pool overlap

| arXiv ID | Short Name | Appears In | Main Pool Entry |
|----------|-----------|------------|-----------------|
| 2402.01788 | LitLLM | EM-01 | #5 |
| 2408.07884 | Instruct LLMs Step by Step | EM-01 | #7 |
| 2504.08762 | InteractiveSurvey | EM-01 | #9 |
| 2510.21900 | IterSurvey | EM-01 | #30 |
| 2504.18496 | DimInd | EM-01 | #31 |
| 2306.03535 | SciLit | EM-01 | #17 |
| 2503.04629 | SurveyForge | EM-01 | #35 |
| 2504.00824 | ScholarCopilot | EM-02 | #27 |
| 2507.15245 | SPAR | EM-02 | #11 |
| 2408.02508 | PUREsuggest | EM-04 | #19 |
| 2512.22159 | Oignon | EM-04 | #37 |
| 1205.1143 | Dir-Aware Citation | EM-04 | #14 |
| 1805.02262 | SemScholar LitGraph | EM-04 | #12 |
| 1812.11252 | Non-obvious Papers | EM-04 | #41 |
| 1501.04894 | CITEX | EM-04 | #40 |
| 2302.07302 | CiteSee | EM-04 | #18 |
| 2110.06595 | Refcat | EM-04 | #38 |

---

## Query Coverage Summary

| Source Query | Count (raw) | New Unique | Overlap |
|-------------|-------------|------------|---------|
| EM-01 (Interactive human-guided survey) | 12 | 5 | 7 |
| EM-02 (LLM scholarly writing agents) | 12 | 10 | 2 |
| EM-03 (LLM tool-use for research) | 10 | 10 | 0 |
| EM-04 (Seed selection & traversal) | 12 | 4 | 8 |
| EM-05 (Citation intent classification) | 10 | 10 | 0 |
| EM-06 (SLR automation & screening) | 10 | 10 | 0 |
| EM-07 (Contradiction detection) | 10 | 10 | 0 |
| **Total** | **76** | **59** | **17** |

---

## Role Distribution

| Role | Count |
|------|-------|
| method | 13 |
| mechanism | 32 |
| theory | 6 |
| survey | 4 |
| benchmark | 4 |

---

## Notes

1. **EM-03 (Tool-use agents)**: Lowest engineering overlap with survey-specific methods; these papers contribute general agent orchestration patterns (recursive tool search, mind-map agents, tool documentation unification) that could transfer to citation traversal.

2. **EM-05 (Citation intent)**: Highest density of specialised NLP methods directly applicable to survey quality — citation intent classifiers are essential for relevance-weighted citation synthesis. All 10 results are unique to the extended pool.

3. **EM-06 (SLR automation)**: Traditional SLR automation tools (ASReview, CRUISE-Screening) represent the pre-LLM state of the art in rigorous literature selection. These are candidates for hybrid LLM+traditional screening pipelines.

4. **EM-07 (Contradiction detection)**: Directly addresses a known capability gap in automated survey systems — no papers in the main pool handle conflicting evidence. Yields 10 new candidates with high potential impact.

5. **Cross-query duplicate**: 2502.11705 (ToolMaker) appeared in both EM-03 and is listed as an adjacent result in EM-07; counted once.

6. **Overlap pattern**: EM-04 (seed selection) had 8/12 overlaps with the main pool's M-04 query, confirming good coverage of citation graph expansion in the main plan. EM-01 had 7/12 overlaps, indicating partial coverage of interactive systems.
