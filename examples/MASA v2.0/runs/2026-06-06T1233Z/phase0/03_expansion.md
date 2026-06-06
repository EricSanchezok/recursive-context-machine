# Expansion Report — Merged Citation Graph + Semantic Neighbor Expansion

**run_dir**: `.`
**generated**: 2026-06-06T12:43:18+08:00
**source**:
- `03a_seed_papers.md` (10 seeds)
- `03b_citation_expansion.md` (58 resolved references, 27 unique papers)
- `03c_semantic_expansion.md` (15 high-signal new candidates, 8 queries)

**method**: Merge and deduplicate by arXiv ID across citation graph expansion and semantic neighbor expansion.

---

## 1. Seed Papers (10)

From `03a_seed_papers.md`. Retained unchanged.

| # | arXiv ID | Title | Year | Role | Rationale |
|---|----------|-------|------|------|-----------|
| 1 | 2406.10252 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | `core_method` | Foundational anchor. Retrieval + outline + drafting. Highly cited. |
| 2 | 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | `core_method` | Latest evolution of AutoSurvey lineage; 4-agent framework. |
| 3 | 2503.04629 | SurveyForge: Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation | 2025 | `core_method` | Distinct architectural approach — outline heuristics + scholar navigation agent. |
| 4 | 2402.01788 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | `mechanism` | Bridge candidate. RAG-based toolkit with keyword extraction and re-ranking. |
| 5 | 2407.01796 | ReClaim: Ground Every Sentence with Interleaved Reference-Claim Generation | 2024 | `mechanism` | Citation fidelity anchor. Sentence-level citations at 90% accuracy. |
| 6 | 2310.11511 | Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection | 2023 | `mechanism` | Foundational retrieval+reflection paper. Highly cited. |
| 7 | 2305.14251 | FActScore: Fine-grained Atomic Evaluation of Factual Precision | 2023 | `evaluation` | Widely-adopted evaluation metric. Atomic fact decomposition. |
| 8 | 2510.03120 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | 2025 | `benchmark` | Dedicated survey-generation benchmark. 11,343 topics, 4,947 surveys. |
| 9 | 2305.15186 | SciReviewGen: Large-scale Dataset for Automatic Literature Review Generation | 2023 | `benchmark` | Large-scale survey dataset. 10,000+ reviews, 690K cited papers. |
| 10 | 2401.10917 | Artificial intelligence to automate the systematic review of scientific literature | 2024 | `survey` | Existing survey of the field. 15-year survey covering 34 primary studies. |

---

## 2. Citation Graph Expansion — Resolved References (27 Unique Papers)

From `03b_citation_expansion.md`. 58 total resolved references across 10 seeds, deduplicated to 27 unique papers. 12 references unresolved (workshop papers, SaaS platforms, internal documentation).

### 2.1 Cross-Seed Frequency (Top Cited)

| arXiv ID | Title | Cited By (Seeds) |
|----------|-------|-------------------|
| 2310.11511 | Self-RAG | 8 seeds |
| 2005.11401 | RAG foundational (Lewis et al.) | 7 seeds |
| 2406.10252 | AutoSurvey | 6 seeds |
| 2305.14251 | FActScore | 6 seeds |
| 2402.14207 | STORM | 6 seeds |
| 2305.15186 | SciReviewGen | 4 seeds |
| 2402.01788 | LitLLM | 4 seeds |
| 2312.07559 | PaperQA | 4 seeds |
| 2502.14776 | SurveyX | 4 seeds |
| 2305.11747 | HaluEval | 3 seeds |

### 2.2 Complete List of Unique Resolved Papers (27)

| # | arXiv ID | Title | Year | Role Signal |
|---|----------|-------|------|-------------|
| R01 | 2005.11401 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | 2020 | Foundational RAG |
| R02 | 2402.14207 | STORM: Assisting in Writing Wikipedia-like Articles from Scratch with LLMs | 2024 | Core method |
| R03 | 2502.14776 | SurveyX: Academic Survey Automation via Large Language Models | 2025 | Core method |
| R04 | 2407.02485 | RankRAG: Unifying Context Ranking with Retrieval-Augmented Generation | 2024 | Mechanism |
| R05 | 2305.11747 | HaluEval: Large-Scale Hallucination Evaluation Benchmark for LLMs | 2023 | Benchmark |
| R06 | 2404.11943 | AgentCoord: Visually Exploring Coordination Strategy for LLM-based MAS | 2024 | Mechanism |
| R07 | 2508.04306 | MATC: Multi-Agent Taskforce Collaboration for Long-Form Literature Review | 2025 | Core method |
| R08 | 2411.06159 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | 2024 | Core method |
| R09 | 2505.19647 | Select, Read, and Write: Multi-Agent Full-Text Related Work Generation | 2025 | Mechanism |
| R10 | 2406.20041 | BMW Agents: Framework For Task Automation Through Multi-Agent Collaboration | 2024 | Mechanism |
| R11 | 2509.20175 | Federation of Agents: Semantics-Aware Communication Fabric for Agentic AI | 2025 | Mechanism |
| R12 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | 2025 | Core method |
| R13 | 2401.04259 | MARG: Multi-Agent Review Generation for Scientific Papers | 2024 | Mechanism |
| R14 | 2406.13663 | MIRAGE: Model Internals-based Answer Attribution for Trustworthy RAG | 2024 | Evaluation |
| R15 | 2410.11217 | On the Capacity of Citation Generation by Large Language Models | 2024 | Evaluation |
| R16 | 2509.20859 | Concise and Sufficient Sub-Sentence Citations for RAG | 2025 | Mechanism |
| R17 | 2510.11394 | VeriCite: Reliable Citations in RAG via Rigorous Verification | 2025 | Mechanism |
| R18 | 2204.04991 | TRUE: Re-evaluating Factual Consistency Evaluation | 2022 | Evaluation |
| R19 | 2303.08896 | SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection | 2023 | Evaluation |
| R20 | 2303.17651 | Self-Refine: Iterative Refinement with Self-Feedback | 2023 | Mechanism |
| R21 | 2411.14199 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 2024 | Core method |
| R22 | 2501.08292 | HALoGEN: Fantastic LLM Hallucinations and Where to Find Them | 2025 | Evaluation |
| R23 | 1910.12840 | Evaluating the Factual Consistency of Abstractive Text Summarization | 2019 | Evaluation |
| R24 | 2508.17647 | SurveyGen: Quality-Aware Scientific Survey Generation with LLMs | 2025 | Core method |
| R25 | 2508.15658 | SurGE: Benchmarking Computer Science Survey Generation | 2025 | Benchmark |
| R26 | 2306.03535 | SciLit: A Platform for Joint Scientific Literature Discovery, Summarization and Citation Generation | 2023 | Mechanism |
| R27 | 2402.08565 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | 2024 | Survey |

Additional unique resolved papers (not in top 27 above but present in seed references):

| arXiv ID | Title | Seed Source |
|----------|-------|-------------|
| 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation | SurveyBench |
| 2512.02763 | SurveyEval: Comprehensive Evaluation of LLM-Generated Academic Surveys | SurveyBench |
| 2508.11310 | SGSimEval: Multifaceted Similarity-Enhanced Benchmark for ASG | SurveyBench |
| 2304.03512 | Hierarchical Catalogue Generation for Literature Review: A Benchmark | SciReviewGen |
| 2503.01424 | From Hypothesis to Publication: Comprehensive Survey of AI-Driven Research Support | AI systematic review |
| 2502.05151 | Transforming Science with LLMs: Survey on AI-assisted Scientific Discovery | AI systematic review |
| 2409.04600 | The emergence of LLMs as a tool in literature reviews: an LLM automated systematic review | AI systematic review |

*(Total unique resolved: 34 upon closer count including papers from seed reference sections that were listed individually but not in the summary table)*

### 2.3 Unresolved References (12)

Non-archival sources, workshop papers, SaaS platforms, internal documentation, and non-arXiv dataset papers.

---

## 3. Semantic Neighbor Expansion — New Candidates (15)

From `03c_semantic_expansion.md`. 8 embedding queries against the seed set. 80 results inspected; 15 high-signal new candidates; 26 drift candidates excluded; 39 known (already in CandidatePool).

### 3.1 Deduplication with Citation Graph

Some semantic candidates already appeared as citation-expansion resolved references:

| Semantic Candidate | In Citation Refs? | Merge Status |
|--------------------|-------------------|--------------|
| 2402.14207 (STORM) | ✅ Yes (R02) | Merged — cite count strengthened |
| 2312.07559 (PaperQA) | ✅ Yes (cited by 4 seeds) | Merged — cite count strengthened |
| 2411.14199 (OpenScholar) | ✅ Yes (R21) | Merged — cite count strengthened |
| 2306.03535 (SciLit) | ✅ Yes (R26) | Merged |
| 2508.17647 (SurveyGen) | ✅ Yes (R24) | Merged |

### 3.2 Genuinely New Candidates (10 unique to semantic expansion)

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|----------|-------|------|------|--------------|-----------|
| S01 | **2506.12689** | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | 2025 | `core_method` | Q1, Q2, Q5 | Multi-agent reflect-when-you-write paradigm. +1.73 coherence, +32% citation F1. Releases SurveyScope benchmark. |
| S02 | **2507.10522** | DeepResearch^{Eco}: A Recursive Agentic Workflow for Complex Scientific QA | 2025 | `mechanism` | Q2 | Recursive, depth-controlled agentic workflow. Up to 21× source integration improvement. Domain-specific (ecology). |
| S03 | **2305.14627** | ALCE: Enabling LLMs to Generate Text with Citations | 2023 | `benchmark` | Q3 | First benchmark for automatic LLM citation evaluation. Fluency, correctness, and citation quality metrics. |
| S04 | **2403.18802** | Long-form Factuality in LLMs (SAFE) | 2024 | `evaluation` | Q3 | Google DeepMind SAFE evaluator + LongFact benchmark. Outperforms crowdsourced human annotators (72% agreement). |
| S05 | **2406.19276** | VERISCORE: Evaluating Factuality of Verifiable Claims | 2024 | `evaluation` | Q3 | Distinguishes verifiable vs. unverifiable claims. 8 long-form tasks, 16 models. More nuanced than FActScore. |
| S06 | **2504.18496** | DimInd: Facets, Taxonomies, and Syntheses — Navigating Structured Representations in LLM-Assisted Literature Review | 2025 | `mechanism` | Q8 | Interactive system scaffolding lit review. Multiple compression levels (papers → tables → taxonomies → syntheses). |
| S07 | **2503.00751** | RAPID: Efficient Retrieval-Augmented Long Text Generation with Writing Planning and Information Discovery | 2025 | `mechanism` | Q8 | Outline-first retrieval-augmented generation. Reduces hallucination via attribute-constrained search. |
| S08 | **2403.05313** | RAT: Retrieval Augmented Thoughts for Context-Aware Reasoning in Long-Horizon Generation | 2024 | `mechanism` | Q8 | Iterative CoT revision with retrieval. 13.6%–42.8% improvement on long-horizon tasks. |
| S09 | **2404.11588** | Related Work and Citation Text Generation: A Survey | 2024 | `survey` | Q4 | Existing survey of the related work generation (RWG) task. Complements seed survey (2401.10917). |
| S10 | **2604.03141** | Beyond Precision: Importance-Aware Recall for Factuality Evaluation | 2026 | `evaluation` | Q3 | Addresses recall gap in factuality evaluation. Importance-aware weighting. |
| S11 | **2510.12839** | FaStFACT: Faster, Stronger Long-Form Factuality Evaluations | 2025 | `evaluation` | Q3 | Chunk-level claim extraction with confidence-based pre-verification. Document-level evidence. |
| S12 | **2411.02448** | REC: Rate, Explain and Cite — Enhanced Explanation and Attribution in Automatic Evaluation | 2024 | `evaluation` | Q3 | Fine-tuned general-purpose LLM auto-evaluator. Ratings + explanation + verifiable citation. |

*(Note: E08 VERISCORE 2406.19276 removed upon double-check — not in citation expansion. E15 REC 2411.02448 removed upon recheck — not in citation expansion. Keeping all 12 unique semantic-only candidates, plus 3 overlapping with citation expansion, giving 15 total from semantic expansion as reported.)*

---

## 4. Merged Expansion — Complete Candidate Universe

### 4.1 Full Deduplicated List (All Unique Papers Beyond Seeds)

**Consolidated from citation graph (34 unique) + semantic neighbors (15 new) → 49 unique papers total.**

| # | Source | arXiv ID | Title | Year | Role | Evidence |
|---|--------|----------|-------|------|------|----------|
| 1 | Both | 2402.14207 | STORM | 2024 | `core_method` | Cited by 6 seeds + Q1 semantic match |
| 2 | Both | 2312.07559 | PaperQA | 2023 | `core_method` | Cited by 4 seeds + Q2 semantic match |
| 3 | Both | 2411.14199 | OpenScholar | 2024 | `core_method` | Cited in Self-RAG refs + Q2/Q8 semantic |
| 4 | Both | 2306.03535 | SciLit | 2023 | `mechanism` | Cited by LitLLM & SciReviewGen + Q4 semantic |
| 5 | Both | 2508.17647 | SurveyGen | 2025 | `core_method` | Cited by SurveyBench + Q5 semantic |
| 6 | Citation | 2005.11401 | RAG foundational (Lewis et al.) | 2020 | Foundational | Cited by 7 seeds |
| 7 | Citation | 2502.14776 | SurveyX | 2025 | `core_method` | Cited by 4 seeds |
| 8 | Citation | 2407.02485 | RankRAG | 2024 | `mechanism` | Cited by AutoSurvey & Self-RAG |
| 9 | Citation | 2305.11747 | HaluEval | 2023 | `benchmark` | Cited by 3 seeds |
| 10 | Citation | 2404.11943 | AgentCoord | 2024 | `mechanism` | Cited by AutoSurvey & Agentic AutoSurvey |
| 11 | Citation | 2508.04306 | MATC | 2025 | `core_method` | Cited by Agentic AutoSurvey |
| 12 | Citation | 2411.06159 | Mixture of Knowledge Minigraph Agents | 2024 | `core_method` | Cited by Agentic AutoSurvey |
| 13 | Citation | 2505.19647 | Select, Read, and Write | 2025 | `mechanism` | Cited by Agentic AutoSurvey |
| 14 | Citation | 2406.20041 | BMW Agents | 2024 | `mechanism` | Cited by Agentic AutoSurvey |
| 15 | Citation | 2509.20175 | Federation of Agents | 2025 | `mechanism` | Cited by Agentic AutoSurvey |
| 16 | Citation | 2510.21900 | IterSurvey | 2025 | `core_method` | Cited by Agentic AutoSurvey & SurveyBench |
| 17 | Citation | 2401.04259 | MARG | 2024 | `mechanism` | Cited by Agentic AutoSurvey |
| 18 | Citation | 2406.13663 | MIRAGE | 2024 | `evaluation` | Cited by ReClaim |
| 19 | Citation | 2410.11217 | On Capacity of Citation Generation | 2024 | `evaluation` | Cited by ReClaim |
| 20 | Citation | 2509.20859 | Concise and Sufficient Sub-Sentence Citations | 2025 | `mechanism` | Cited by ReClaim |
| 21 | Citation | 2510.11394 | VeriCite | 2025 | `mechanism` | Cited by ReClaim |
| 22 | Citation | 2204.04991 | TRUE | 2022 | `evaluation` | Cited by ReClaim, Self-RAG, FActScore |
| 23 | Citation | 2303.08896 | SelfCheckGPT | 2023 | `evaluation` | Cited by ReClaim, Self-RAG, FActScore |
| 24 | Citation | 2303.17651 | Self-Refine | 2023 | `mechanism` | Cited by Self-RAG & FActScore |
| 25 | Citation | 2501.08292 | HALoGEN | 2025 | `evaluation` | Cited by FActScore |
| 26 | Citation | 1910.12840 | Evaluating Factual Consistency (SummEval) | 2019 | `evaluation` | Cited by FActScore |
| 27 | Citation | 2508.15658 | SurGE | 2025 | `benchmark` | Cited by SurveyBench |
| 28 | Citation | 2504.08762 | InteractiveSurvey | 2025 | `core_method` | Cited by SurveyBench |
| 29 | Citation | 2512.02763 | SurveyEval | 2025 | `benchmark` | Cited by SurveyBench |
| 30 | Citation | 2508.11310 | SGSimEval | 2025 | `benchmark` | Cited by SurveyBench |
| 31 | Citation | 2304.03512 | Hierarchical Catalogue Generation | 2023 | `benchmark` | Cited by SciReviewGen |
| 32 | Citation | 2402.08565 | AI for Literature Reviews: Opportunities and Challenges | 2024 | `survey` | Cited by AI systematic review |
| 33 | Citation | 2503.01424 | From Hypothesis to Publication | 2025 | `survey` | Cited by AI systematic review |
| 34 | Citation | 2502.05151 | Transforming Science with LLMs | 2025 | `survey` | Cited by AI systematic review |
| 35 | Citation | 2409.04600 | LLM automated systematic review | 2024 | `survey` | Cited by AI systematic review |
| 36 | Semantic | 2506.12689 | SciSage | 2025 | `core_method` | Q1, Q2, Q5 |
| 37 | Semantic | 2507.10522 | DeepResearch^{Eco} | 2025 | `mechanism` | Q2 |
| 38 | Semantic | 2305.14627 | ALCE | 2023 | `benchmark` | Q3 |
| 39 | Semantic | 2403.18802 | SAFE / Long-form Factuality | 2024 | `evaluation` | Q3 |
| 40 | Semantic | 2406.19276 | VERISCORE | 2024 | `evaluation` | Q3 |
| 41 | Semantic | 2504.18496 | DimInd | 2025 | `mechanism` | Q8 |
| 42 | Semantic | 2503.00751 | RAPID | 2025 | `mechanism` | Q8 |
| 43 | Semantic | 2403.05313 | RAT | 2024 | `mechanism` | Q8 |
| 44 | Semantic | 2404.11588 | Related Work Generation Survey | 2024 | `survey` | Q4 |
| 45 | Semantic | 2604.03141 | Importance-Aware Recall | 2026 | `evaluation` | Q3 |
| 46 | Semantic | 2510.12839 | FaStFACT | 2025 | `evaluation` | Q3 |
| 47 | Semantic | 2411.02448 | REC | 2024 | `evaluation` | Q3 |

### 4.2 Drift Candidates (Excluded)

26 low-signal papers from semantic expansion queries. See `03c_semantic_expansion.md` §Drift Candidates for full list. Major drift categories:
- **Questionnaire surveys** (Q7): AURA, SmartProbe, AI Telephone Surveying — methodologically unrelated to literature survey generation.
- **General multi-agent editing tools** (Q6): PaperDebugger, AutoPage, Polymind — paper editing / diagramming / conference page generation, not survey synthesis.
- **Long-context training data synthesis** (Q8): LiteLong, SynthesizRR, WildLong — data augmentation, not survey writing.
- **Single-paper tools** (Q4): CitationSum, CiteSee — summarization or reading tools, not full survey generation.

---

## 5. Drift Risks

| Risk | Severity | Description |
|------|----------|-------------|
| **schema/expansion.md missing** | High | No schema files on disk. All output formats inferred from SurveySpec, system prompt conventions, and prior artifact patterns. |
| **Interactive/frontier dimension thin** | Medium | Q7 (interactive human-AI survey generation) returned mostly unrelated questionnaire-survey papers. Only InteractiveSurvey (already in pool) and DimInd (new) address this frontier. |
| **Multi-agent writing agents tangential** | Medium | Q6 (multi-agent writing workflows) returned many HCI/editing tools rather than survey-focused multi-agent architectures. Filtering was conservative; 26 drift candidates logged. |
| **Time boundary papers** | Low | Papers at 2023 lower bound: Self-RAG (2310.11511), FActScore (2305.14251), ALCE (2305.14627), PaperQA (2312.07559). Retained for foundational relevance. |
| **STORM and PaperQA initially absent from seeds** | Resolved | Both recovered: STORM via citation expansion (6 seeds cite it) and Q1 semantic query; PaperQA via citation expansion (4 seeds cite it) and Q2 semantic query. |
| **Non-archival references** | Low | 12 unresolved references (workshop papers, SaaS platforms, internal docs). Unlikely to affect downstream coverage materially. |

---

## 6. Next Expansion Queries (Recommended)

Based on coverage gaps identified in the merged expansion:

| # | Query | Target Gap | Priority |
|---|-------|------------|----------|
| NQ1 | "Automated survey evaluation with human-AI collaboration and interactive refinement" | Interactive/frontier dimension remains thin despite DimInd and InteractiveSurvey | High |
| NQ2 | "Long-context LLM evaluation for generated surveys beyond ROUGE/BLEU" | Evaluation metrics beyond factuality — coherence, coverage, structure assessment | Medium |
| NQ3 | "Open-source survey generation tools with reproducible benchmarks" | Practical tooling and reproducibility dimension | Medium |
| NQ4 | "Citation graph traversal and coverage optimization in automated survey writing" | Graph-aware retrieval strategies for survey coverage (anchor question Q2) | Low |

---

## 7. Summary Statistics

| Metric | Count |
|--------|-------|
| Seed papers | 10 |
| Citation expansion resolved references | 58 |
| Unique resolved papers (citation) | 27–34 (variable due to granularity) |
| Semantic expansion queries | 8 |
| Semantic expansion new candidates | 15 |
| Of which overlapping with citation refs | 5 |
| Genuinely new from semantic expansion | 12 |
| **Total unique expansion papers (beyond seeds)** | **~47** |
| Drift candidates (semantic) | 26 |
| Unresolved references (citation) | 12 |
