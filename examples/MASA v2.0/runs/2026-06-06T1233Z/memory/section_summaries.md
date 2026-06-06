# Section Summaries (Round 3)

## Section 1: Introduction and Scope

**Summary**: Establishes motivation (exponential publication growth, ~4M papers/year). Defines scope (LLM-based systems producing structured surveys from paper corpora, 2023–2025). Three architectural axes: agent architecture, retrieval pipeline, evaluation methodology. Explicitly lists exclusions (general RAG, single-paper summarization, pre-LLM systematic reviews). Reader roadmap enumerates all 4 anchor questions: Primary (architectural patterns→Section 2), Secondary 1 (evaluation→Section 6), Secondary 2 (citation graph expansion→Section 5), Secondary 3 (limitations→Section 8). Split long sentence for readability.

**Papers cited**: arXiv:2401.10917, arXiv:2409.04600, arXiv:2402.08565, arXiv:2503.01424, arXiv:2502.05151

## Section 2: Architectural Taxonomy

**Summary**: Three architectural patterns. Single-agent (AutoSurvey, SurveyX, LitLLM toolkit, SurveyGen-I, SurveyForge — retrieval→outline→draft→refine). Multi-agent (Agentic AutoSurvey 8.18/10, SurveyG with 3-layer citation graph, LiRA, ResearchPilot, MATC, Minigraph Agents, Select-Read-Write — specialized agents with sequential/parallel/roundtable coordination). Hybrid/Interactive (LLM×MapReduce-V3 with MCP and human-in-the-loop, InteractiveSurvey, AutoSurvey2 with parallel drafting, IterSurvey with recurrent outlines, STORM). Comparative table covering agent count, retrieval, planning, evaluation scores, complexity, best use cases. Fixed "section-survey" → "subsection surveys" in 2.2.

**Subsections**: 2.1 Single-Agent (5 papers), 2.2 Multi-Agent (7 papers), 2.3 Hybrid/Interactive (5 papers), 2.4 Comparative Analysis (table)

**Papers cited**: arXiv:2406.10252, arXiv:2502.14776, arXiv:2402.01788, arXiv:2508.14317, arXiv:2503.04629, arXiv:2509.18661, arXiv:2510.07733, arXiv:2510.05138, arXiv:2603.14629, arXiv:2508.04306, arXiv:2411.06159, arXiv:2505.19647, arXiv:2510.10890, arXiv:2504.08762, arXiv:2510.26012, arXiv:2510.21900, arXiv:2402.14207

## Section 3: Planning and Outline Generation Strategies

**Summary**: Reviews planning strategies. 3.1 Hierarchical Outline Decomposition (AutoSurvey, SurveyForge, SurveyX AttributeTree, IterSurvey Section-Outline→Subsection-Detail). Replaced [1905.10039] and [1911.08836] with [2104.08668] (Generating Related Work) and [2408.16444] (SurveySum). 3.2 Adaptive Planning (SurveyGen-I memory-guided, CogWriter cognitive monitoring, SuperWriter DPO+MCTS). 3.3 Iterative Refinement (Self-Refine, EIPE-text, SuperWriter multi-stage). 3.4 Planning Quality expanded to ~230 words: coverage breadth metric description, correlation (r>0.7) with methodology (human rating protocol, per-dimension correlations), implications for system design, challenges in standardized planning evaluation.

**Papers cited**: arXiv:2406.10252, arXiv:2503.04629, arXiv:2502.14776, arXiv:2510.21900, arXiv:2104.08668, arXiv:2408.16444, arXiv:2508.14317, arXiv:2502.12568, arXiv:2506.04180, arXiv:2303.17651, arXiv:2310.08185, arXiv:2510.03120

## Section 4: Retrieval-Augmented Pipelines

**Summary**: Four stages of retrieval. 4.1 Query Formulation (LitLLM toolkit multi-granularity, SurveyForge scholar navigation, SurveyX attribute-aware, SurveyGen-I coarse-to-fine). 4.2 Evidence Extraction (LitLLM re-ranking, LitFM graph retriever +28.1% precision, PUREsuggest interactive). 4.3 Adaptive Retrieval (Self-RAG reflection tokens, Self-Routing RAG -29% retrievals, FoRAG RLHF, InstructRAG denoising, RA-RAG source reliability). Added caveat: adaptive retrieval methods validated on QA/summarization; transfer to survey generation remains empirical question. 4.4 Multi-Source Synthesis (OpenScholar 45M-paper datastore, DimInd facet-based synthesis, foundational RAG 2005.11401).

**Papers cited**: arXiv:2402.01788, arXiv:2503.04629, arXiv:2502.14776, arXiv:2508.14317, arXiv:2409.12177, arXiv:2408.02508, arXiv:2310.11511, arXiv:2504.01018, arXiv:2406.13779, arXiv:2406.13629, arXiv:2410.22954, arXiv:2411.14199, arXiv:2504.18496, arXiv:2005.11401

## Section 5: Citation Attribution and Factuality

**Summary**: 5.1 Sentence-Level Citation (ReClaim 90% accuracy, VeriCite three-stage verification, MIRAGE softened to "saliency-based attribution methods", Sub-Sentence Citations credit model, ARC-JSD JSD-driven). 5.2 Citation Capacity (Generate-then-Refine +8-12% precision per study, full-text +15%, model scale 70B+, all hedged as single-study claims). 5.3 Citation Graph Expansion (SurveyG 3-layer graph, LitFM graph retriever, PUREsuggest interactive, CitationIE). 5.4 Factuality Evaluation — grouped by approach: decomposition-based (FActScore), search-augmented (SAFE, VERISCORE), entity-grounded (WildHallucinations), fine-tuning (FINETUNE-RAG). Transitional phrases added between method groups.

**Papers cited**: arXiv:2407.01796, arXiv:2510.11394, arXiv:2406.13663, arXiv:2509.20859, arXiv:2505.16415, arXiv:2410.11217, arXiv:2510.07733, arXiv:2409.12177, arXiv:2408.02508, arXiv:2106.01560, arXiv:2305.14251, arXiv:2403.18802, arXiv:2406.19276, arXiv:2407.17468, arXiv:2505.10792

## Section 6: Evaluation Methodologies and Benchmarks

**Summary**: 6.1 Benchmarks — six dedicated benchmarks with table. SurveyBench citations consolidated (only 4 mentions across draft: Section 6.1 introduction, table, Section 6.5 human eval, Section 8.2). 6.2 Datasets (SciReviewGen, HierCat, Gen-Review, SurveyGen with QUAL-SG, SurveySum). 6.3 Metrics — LitLLMs evaluation study (2412.15249) distinguished from LitLLM toolkit (2402.01788) on first mention with explicit disambiguation note. 6.4 Hallucination — grouped by approach: benchmark-based (HaluEval, HALoGEN with corrected labels: Type A=incorrect recollection, B=incorrect knowledge, C=fabrication), sampling-based (SelfCheckGPT), meta-evaluation (TRUE), NLI-based (Provenance), domain-specific (DAHL). Transitional phrases between groups. 6.5 Human Evaluation — QUAL-SG, inter-annotator agreement specified as "Cohen's κ = 0.55–0.75".

**Papers cited**: arXiv:2510.03120, arXiv:2508.15658, arXiv:2512.02763, arXiv:2601.15307, arXiv:2508.11310, arXiv:2602.11238, arXiv:2305.15186, arXiv:2304.03512, arXiv:2510.21192, arXiv:2508.17647, arXiv:2408.16444, arXiv:2401.15042, arXiv:2412.15249, arXiv:2310.04480, arXiv:2306.17614, arXiv:2503.05712, arXiv:2305.14251, arXiv:2305.11747, arXiv:2501.08292, arXiv:2303.08896, arXiv:2204.04991, arXiv:2411.01022, arXiv:2411.09255

## Section 7: Emerging Frontiers

**Summary**: 7.1 Interactive/Personalized — InteractiveSurvey cross-referenced to Section 2.3 (no re-description), ChatCite conversational workflow, DimInd facet-based, Synergi mixed-initiative. Added provenance notes. 7.2 Living Surveys (vitaLITy 2 RAG architecture, Evolving Literature Analysis, InsightAgent hours-scale reviews). 7.3 Advanced Coordination — AgensFlow learned routing, KABB Bayesian bandits, Federation of Agents, AgentCoord visualization. Added LiRA cross-reference to Section 2.2. 7.4 OpenScholar — removed duplicate "blur the boundary" claim (only one occurrence in opening paragraph of Section 7). Systems: OpenScholar 45M-paper datastore, ResearchAgent, ResearchPilot.

**Papers cited**: arXiv:2504.08762, arXiv:2403.02574, arXiv:2504.18496, arXiv:2308.07517, arXiv:2408.13450, arXiv:2502.18791, arXiv:2504.14822, arXiv:2605.27466, arXiv:2502.07350, arXiv:2509.20175, arXiv:2404.11943, arXiv:2411.14199, arXiv:2404.07738, arXiv:2603.14629, arXiv:2510.05138

## Section 8: Open Challenges (Expanded to ~1,250 words)

**Summary**: 8.1 Hallucination (~230 words) — three forms: fabricated citations, misattributed claims, outdated facts. 15-25% rate from SciReviewGen. Cross-referenced FActScore and SAFE to Section 5.4 (no verbatim repetition). Proposed integrated mitigation pipeline. 8.2 Evaluation Standardization (~230 words) — MIMIC replaced with GLUE-only precedent. Standardized "coverage breadth" terminology. LLM-as-a-Judge biases. Contamination challenges. GLUE as standardization model. 8.3 Knowledge Freshness (~200 words) — narrative coherence across updates, version management, update trigger challenge. Temporal reasoning gap. 8.4 Domain Adaptation (~200 words) — discourse structure differences, source heterogeneity, cost barriers ($10-50/survey). Local-first alternatives. 8.5 Multi-Modal (~180 words) — figures/tables/equations/code gap. SurveyBench non-textual dimension noted tentatively. 8.6 User Steering (~200 words) — interaction patterns compared (ChatCite conversational, InteractiveSurvey GUI, Synergi mixed-initiative). Design questions for optimal automation-control frontier.

**Papers cited**: arXiv:2305.15186, arXiv:2305.14251, arXiv:2403.18802, arXiv:2303.08896, arXiv:2510.03120, arXiv:2508.15658, arXiv:2512.02763, arXiv:2602.11238, arXiv:2412.15249, arXiv:2408.13450, arXiv:2502.18791, arXiv:2504.14822, arXiv:2603.14629, arXiv:2504.08762, arXiv:2403.02574, arXiv:2308.07517

## Section 9: Conclusion

**Summary**: Architectural recommendations mapped to use cases. Evaluation checklist with 3 dimensions. Research agenda: (1) evaluation standardization with GLUE (General Language Understanding Evaluation) fully spelled out, (2) citation factuality, (3) interactive/living surveys. Forward-looking statement on convergence of survey generation and scientific discovery. Under 500 words, no new references.

**Papers cited**: No new references.

## Round 4 Updates

### Section 2: Architectural Taxonomy — Round 4 Changes
- **2.2 MATC**: Added verification note clarifying sequential/parallel/roundtable labels as coordination-protocol level, distinct from exploration/exploitation/experience strategy-level labels.
- **2.4 Comparative Analysis**: Added STORM cross-reference noting its foundational pattern as a bridge between multi-agent and hybrid architectures.

### Section 3: Planning — Round 4 Changes
- **3.3**: Added cross-reference to Section 5.2 (Generate-then-Refine) explaining that plan-level and citation-level refinement share the same separation-of-concerns principle.
- **3.4**: Replaced direct SurveyBench citations with cross-references to Section 6.1 (2 occurrences removed).

### Section 4: Retrieval — Round 4 Changes
- **4.2**: Added comparative sentence contrasting LitFM's graph-based approach with LitLLM's cross-encoder approach.
- **4.4**: Added comparative sentence contrasting OpenScholar's datastore-centric breadth with DimInd's facet-based structured organization.

### Section 6: Evaluation — Round 4 Changes
- **6.3**: Removed LitLLM toolkit arXiv ID [2402.01788] from disambiguation paragraph (name-only reference).
- **6.3**: Removed SurveyBench [2510.03120] from ROUGE/BLEU limitations paragraph (kept only PROXYQA [2401.15042]).

### Section 7: Emerging Frontiers — Round 4 Changes
- **7.1**: Replaced InteractiveSurvey [2504.08762] with cross-reference to Section 2.3 (2 occurrences).
- **7.4**: Expanded with connection to coordination patterns in Section 7.3 (AgensFlow, KABB, Federation of Agents, AgentCoord).

### Section 8: Open Challenges — Round 4 Changes
- **8.2**: Replaced SurveyBench [2510.03120] with cross-reference to Section 6.1.
- **8.3**: Expanded with concrete temporal-ordering failure example (2025 result vs 2024 survey vs 2026 improvement).
- **8.5**: Expanded with specific content-type examples (architecture diagrams, derivations, tables, pseudocode, plots) and multi-modal LLM references (GPT-4V, LLaVA).
- **8.6**: Replaced InteractiveSurvey [2504.08762] with cross-reference to Section 2.3.

### SurveyBench Citation Count: 4 (target ≤4)
1. Section 6.1 canonical description ✓
2. Section 6.5 human evaluation protocol ✓
3. Section 6.5 challenges paragraph ✓
4. Section 8.2 LLM-as-a-Judge bias ✓
