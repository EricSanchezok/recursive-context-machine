# Section Summaries — Round 1

## Section 1: Introduction and Scope
Established the problem context (publication volume, researcher burden), defined scope around LLM-based survey agents with citation graph expansion, enumerated two anchor questions (primary: citation graph guidance → Sections 2–3; secondary: failure modes → Sections 6, 9), and listed four survey contributions. Cited prior surveys [arXiv:2402.08565, arXiv:2412.15249] to position the gap. ~450 words.

## Section 2: Architectural Taxonomy of Citation-Graph-Aware Survey Agents
Presented three architectural categories: single-agent (AutoSurvey, SurveyX, SurveyForge, LitLLM, SurveyGen-I), multi-agent (Agentic AutoSurvey, SurveyG, LiRA, ResearchPilot, MATC, CKMAs), and hybrid/interactive (STORM, IterSurvey, InteractiveSurvey, SciSage). Included a comparison table (Table 1) across axes: graph traversal strategy, construction, depth control, and relevance ranking. ~1,100 words.

## Section 3: Citation Graph Expansion Strategies for Paper Discovery
Covered four categories: classical traversal (direct citation, cocitation, bibliographic coupling, Interleaved Snowballing, Cascading Citation Expansion, Oignon), graph-based retrieval augmentation (LitFM 28.1% precision improvement, CG-RAG, CitationIE), hierarchical traversal (SurveyG 3-layer, PUREsuggest), and agent-driven RL-optimized traversal (PaSa +37.78% recall, PaperSearchQA, PaSaMaster, SPAR +56% F1). ~1,050 words.

## Section 4: Planning and Outline Generation Strategies
Covered hierarchical decomposition (AutoSurvey, SurveyForge, SurveyX AttributeTree, IterSurvey, Generating Related Work, SurveySum, HierCat), adaptive planning (SurveyGen-I, CogWriter, SuperWriter with MCTS), iterative refinement (Self-Refine, EIPE-text, SuperWriter), and planning quality (SurveyBench outline quality as predictor). ~750 words.

## Section 5: Retrieval-Augmented Pipelines for Evidence Collection
Covered query formulation (LitLLM, SurveyForge, SurveyX, SurveyGen-I coarse-to-fine), evidence extraction and re-ranking (LitFM vs LitLLM contrast, PUREsuggest), adaptive retrieval (Self-RAG, Self-Routing RAG 29% fewer retrievals, FoRAG, InstructRAG, RA-RAG), and multi-source synthesis (OpenScholar, DimInd, foundational RAG [2005.11401]). ~750 words.

## Section 6: Citation Attribution and Factuality Mechanisms
Covered sentence-level citation (ReClaim 90% accuracy, VeriCite, MIRAGE, Sub-Sentence Citations, ARC-JSD), citation capacity analysis [2410.11217], citation graph expansion for attribution (SurveyG, LitFM, PUREsuggest cross-references), and factuality evaluation (FActScore, SAFE, VERISCORE, WildHallucinations, D-FActScore, LongDocFACTScore). ~700 words.

## Section 7: Evaluation Methodologies and Benchmarks
Covered six survey benchmarks (SurveyBench, SurGE, SurveyEval, DeepSurvey-Bench, SGSimEval, SurveyLens) with comparison table, citation-specific benchmarks (CiteEval, ALCE, CiteME 4.2-18.5% vs humans 69.7%, REASONS -42% hallucination, Survey-Arena), datasets (SciReviewGen, HierCat, Gen-Review, SurveyGen), evaluation metrics (PROXYQA, Auto-survey Challenge, Outcome-based), hallucination benchmarks (HaluEval, HALoGEN, SelfCheckGPT, TRUE, Provenance, DAHL, ReFACT), and human evaluation protocols. ~1,100 words.

## Section 8: Emerging Frontiers — Interactive, Living, and Coordinated Survey Paradigms
Covered interactive systems (InteractiveSurvey, ChatCite, DimInd, Synergi), living surveys (vitaLITy 2, Evolving Literature Analysis, InsightAgent), advanced coordination (AgensFlow, KABB, Federation of Agents, AgentCoord), and graph-aware deep research (PaperQA2 superhuman, OpenScholar, Deep Search Agents Survey). ~600 words.

## Section 9: Open Challenges and Future Directions
Covered six challenges: hallucination (15-25% rates, error propagation through citation chains), evaluation standardization (GLUE precedent), knowledge freshness (temporal ordering failures), graph traversal trade-offs (exploration-exploitation, hybrid search strategies [2004.09741]), domain adaptation and cost ($10-50/survey), multi-modal content (figures, tables, equations, code). ~1,200 words.

## Section 10: Conclusion
Summarized architectural recommendations (single-agent for focused topics, multi-agent hierarchical for broad coverage, hybrid for balanced needs), provided an evaluation checklist (citation precision, coverage breadth, factual consistency), listed three research directions (optimal traversal policies, evaluation standardization, interactive/living surveys), and ended with forward-looking statement on convergence with scientific discovery. Under 500 words.

---

# Section Summaries — Round 2

## Section 1: Introduction and Scope
Fixed unsubstantiated arXiv growth statistic (replaced with citation hallucination audit [arXiv:2605.07723]). Removed citations [arXiv:2002.06961] and [arXiv:2306.14905] from exclusion sentence per C2. Added survey spec quality bar statement. Added arXiv:2503.21460 (LLM Agent Survey) as broader agent-context framing. ~480 words.

## Section 2: Architectural Taxonomy of Citation-Graph-Aware Survey Agents
**C1**: Deleted AutoSurvey2 sentence from §2.1 — now kept only in §2.3 (hybrid). **N3**: Added explicit "coverage breadth" definition at first use in the section introduction. **M2**: Added 2–3 sentence automation-vs-user-control spectrum analysis (InteractiveSurvey → IterSurvey → STORM) in §2.3. **Judge #4**: Added evaluation scores column to Table 1 (Agentic AutoSurvey 8.18/10, AutoSurvey 4.77/10, SciSage +1.73/+32%, SurveyX +1.76). ~1,200 words.

## Section 3: Citation Graph Expansion Strategies for Paper Discovery
No changes from Round 1 — no supervisor/judge issues identified. Retained existing coverage of classical traversal, graph-based retrieval, hierarchical traversal, and RL-optimized traversal. ~1,050 words.

## Section 4: Planning and Outline Generation Strategies
**N1**: Added 2 sentences on community detection algorithms (Louvain, spectral clustering) as unsupervised section headings from citation graph partitioning. Added SurveyBench quantitative finding about outline quality as predictor of overall survey quality (already present in Round 1 draft). ~800 words.

## Section 5: Retrieval-Augmented Pipelines for Evidence Collection
No changes from Round 1 — no supervisor/judge issues identified. Retained existing coverage across all four subsections. ~750 words.

## Section 6: Citation Attribution and Factuality Mechanisms
**Judge #13**: Strengthened §6.2 with additional Generate-then-Refine analysis details: two-stage pipeline description, 15–20% accuracy improvement from full-text access, model scale correlation findings. **N2**: Added concrete attribution example in §6.3 (method X SOTA claim with corroborating papers B, C through graph traversal). ~800 words.

## Section 7: Evaluation Methodologies and Benchmarks
**M4**: Added SurveyScope [arXiv:2506.12689] to benchmark list and comparison table in §7.1 (46 papers, 11 CS domains, coherence + citation F1). **Judge #8**: Changed HierCat in §7.3 from full description to cross-reference with §4.1 ("cross-referenced from Section 4.1 for its use in hierarchical outline decomposition"). ~1,150 words.

## Section 8: Emerging Frontiers — Interactive, Living, and Coordinated Survey Paradigms
**M1**: Expanded all four subsections to minimum 150 words. **8.1**: Added details on interaction spectrum, user guidance scalability trade-offs. **8.2**: Added incremental updating challenges, selective re-retrieval vs. full regeneration. **8.3 (M3)**: Added substantive paragraph on HOW coordination patterns affect citation graph partitioning — AgensFlow learned routing for subgraph assignment, KABB bandit selection for prioritized exploration, Federation semantics-aware communication for reduced redundancy. **8.4**: Added PaperQA2 convergence paragraph showing how modular retriever-reader mirrors coordination patterns from §8.3. ~850 words.

## Section 9: Open Challenges and Future Directions
**Judge #5**: Expanded to ~1,100+ words total (was ~620 words). **§9.1**: Added citation hallucination audit reference [arXiv:2605.07723] and error-propagation-aware traversal discussion. **§9.2**: Added explicit coverage breadth definition as standard evaluation dimension. **§9.3**: Added temporal attribution challenge. **§9.4 (Judge #15)**: Added PaSa cross-reference from §3.4 as RL-optimized traversal example addressing exploration-exploitation trade-off. **§9.6 (Judge #6)**: Added PaperArena [arXiv:2510.10909] and Deep Search Agents Survey [arXiv:2508.05668] citations; added multi-modal parsing capability discussion. ~1,200 words.

## Section 10: Conclusion
Minor update: added GLUE (General Language Understanding Evaluation) full name per outline refinement. No substantive content changes from Round 1. ~450 words.

## Section 1: Introduction and Scope
Established the problem context (publication volume, researcher burden), defined scope around LLM-based survey agents with citation graph expansion, enumerated two anchor questions (primary: citation graph guidance → Sections 2–3; secondary: failure modes → Sections 6, 9), and listed four survey contributions. Cited prior surveys [arXiv:2402.08565, arXiv:2412.15249] to position the gap. ~450 words.

## Section 2: Architectural Taxonomy of Citation-Graph-Aware Survey Agents
Presented three architectural categories: single-agent (AutoSurvey, SurveyX, SurveyForge, LitLLM, SurveyGen-I), multi-agent (Agentic AutoSurvey, SurveyG, LiRA, ResearchPilot, MATC, CKMAs), and hybrid/interactive (STORM, IterSurvey, InteractiveSurvey, SciSage). Included a comparison table (Table 1) across axes: graph traversal strategy, construction, depth control, and relevance ranking. ~1,100 words.

## Section 3: Citation Graph Expansion Strategies for Paper Discovery
Covered four categories: classical traversal (direct citation, cocitation, bibliographic coupling, Interleaved Snowballing, Cascading Citation Expansion, Oignon), graph-based retrieval augmentation (LitFM 28.1% precision improvement, CG-RAG, CitationIE), hierarchical traversal (SurveyG 3-layer, PUREsuggest), and agent-driven RL-optimized traversal (PaSa +37.78% recall, PaperSearchQA, PaSaMaster, SPAR +56% F1). ~1,050 words.

## Section 4: Planning and Outline Generation Strategies
Covered hierarchical decomposition (AutoSurvey, SurveyForge, SurveyX AttributeTree, IterSurvey, Generating Related Work, SurveySum, HierCat), adaptive planning (SurveyGen-I, CogWriter, SuperWriter with MCTS), iterative refinement (Self-Refine, EIPE-text, SuperWriter), and planning quality (SurveyBench outline quality as predictor). ~750 words.

## Section 5: Retrieval-Augmented Pipelines for Evidence Collection
Covered query formulation (LitLLM, SurveyForge, SurveyX, SurveyGen-I coarse-to-fine), evidence extraction and re-ranking (LitFM vs LitLLM contrast, PUREsuggest), adaptive retrieval (Self-RAG, Self-Routing RAG 29% fewer retrievals, FoRAG, InstructRAG, RA-RAG), and multi-source synthesis (OpenScholar, DimInd, foundational RAG [2005.11401]). ~750 words.

## Section 6: Citation Attribution and Factuality Mechanisms
Covered sentence-level citation (ReClaim 90% accuracy, VeriCite, MIRAGE, Sub-Sentence Citations, ARC-JSD), citation capacity analysis [2410.11217], citation graph expansion for attribution (SurveyG, LitFM, PUREsuggest cross-references), and factuality evaluation (FActScore, SAFE, VERISCORE, WildHallucinations, D-FActScore, LongDocFACTScore). ~700 words.

## Section 7: Evaluation Methodologies and Benchmarks
Covered six survey benchmarks (SurveyBench, SurGE, SurveyEval, DeepSurvey-Bench, SGSimEval, SurveyLens) with comparison table, citation-specific benchmarks (CiteEval, ALCE, CiteME 4.2-18.5% vs humans 69.7%, REASONS -42% hallucination, Survey-Arena), datasets (SciReviewGen, HierCat, Gen-Review, SurveyGen), evaluation metrics (PROXYQA, Auto-survey Challenge, Outcome-based), hallucination benchmarks (HaluEval, HALoGEN, SelfCheckGPT, TRUE, Provenance, DAHL, ReFACT), and human evaluation protocols. ~1,100 words.

## Section 8: Emerging Frontiers — Interactive, Living, and Coordinated Survey Paradigms
Covered interactive systems (InteractiveSurvey, ChatCite, DimInd, Synergi), living surveys (vitaLITy 2, Evolving Literature Analysis, InsightAgent), advanced coordination (AgensFlow, KABB, Federation of Agents, AgentCoord), and graph-aware deep research (PaperQA2 superhuman, OpenScholar, Deep Search Agents Survey). ~600 words.

## Section 9: Open Challenges and Future Directions
Covered six challenges: hallucination (15-25% rates, error propagation through citation chains), evaluation standardization (GLUE precedent), knowledge freshness (temporal ordering failures), graph traversal trade-offs (exploration-exploitation, hybrid search strategies [2004.09741]), domain adaptation and cost ($10-50/survey), multi-modal content (figures, tables, equations, code). ~1,200 words.

## Section 10: Conclusion
Summarized architectural recommendations (single-agent for focused topics, multi-agent hierarchical for broad coverage, hybrid for balanced needs), provided an evaluation checklist (citation precision, coverage breadth, factual consistency), listed three research directions (optimal traversal policies, evaluation standardization, interactive/living surveys), and ended with forward-looking statement on convergence with scientific discovery. Under 500 words.
