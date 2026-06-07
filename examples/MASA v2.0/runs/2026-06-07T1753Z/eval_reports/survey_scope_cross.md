鈺攢 Response 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
# SurveyScope Evaluation Report

## S1. Citation Precision: 2/5

The survey is citation-dense and often attributes claims to specific papers, but citation precision is weakened by several clear or likely misattributions, unsupported claims, and internal inconsistencies.

Specific evidence:

- The introduction attributes PRISMA-related rigor to 鈥淸Waltman et al., 2020]鈥?in line 11, but the cited reference is 鈥淩eturn to Basics: Clustering using Structural Information,鈥?not the PRISMA reporting guideline. The canonical PRISMA 2020 statement is Page et al., 2021, not Waltman et al.
- 鈥淪TM Global Brief 2023鈥?and 鈥淯NESCO Science Report 2021鈥?are cited in line 5 but do not appear in the reference list.
- SummEval is discussed in line 11, but the canonical SummEval paper is not cited in the bibliography.
- The survey repeatedly cites very specific performance numbers 鈥?e.g., 鈥?28% MAP鈥?for Context-Aware Citation Recommendation, 鈥?28.1% precision鈥?for LitFM, 鈥?37.78% recall@20鈥?for PaSa, 鈥?32% citation F1鈥?for SciSage 鈥?but often does not provide enough benchmark context to verify what the metric means, whether it is relative or absolute, or whether the comparison baseline is appropriate.
- There are internal citation inconsistencies: Section 6.4 says 鈥淪urveyLens鈥檚 discipline-aware design [Li et al., 2026]鈥?while Section 6鈥檚 prioritization table cites 鈥淪urveyLens [Chen et al., 2025e]鈥? the bibliography lists SurveyLens under Li et al., 2026 and Chen et al., 2025e as SGSimEval.
- Several references appear highly generic or possibly synthetic, with placeholder-like author lists and future-dated arXiv identifiers. This is especially concerning for many 2025鈥?026 benchmark/system papers that are treated as established literature.

That said, many core citations are plausible and appropriately matched to their claims: SPECTER [Cohan et al., 2020], Semantic Scholar Literature Graph [Ammar et al., 2018], STORM [Shao et al., 2024], PaperQA [Lala et al., 2023], OpenScholar [Akter et al., 2024], ASReview [van de Schoot et al., 2021], and Science of Science [Fortunato et al., 2018] are cited in generally appropriate contexts.

Overall, the survey shows an attempt at precise attribution, but citation precision is compromised by demonstrable misattributions, missing bibliography entries, inconsistent labels, and a large number of difficult-to-verify recent/future citations.

## S2. Citation Recall: 3/5

The survey covers many relevant works for automated literature survey generation, citation-aware retrieval, multi-agent survey writing, and evaluation. It includes several important high-level clusters:

- Automated survey or long-form synthesis systems: AutoSurvey, STORM, PaperQA, PaperQA2, OpenScholar, SciSage, Agentic AutoSurvey, SurveyGen, SurveyX, IterSurvey.
- Citation-aware retrieval and graph representation: Semantic Scholar Literature Graph, SPECTER, BERT+GCN citation recommendation, LitFM, HiGTL, SurveyG.
- Human-in-the-loop and systematic review automation: ASReview, FAST虏, InsightAgent.
- Evaluation benchmarks: SurveyScope, SurveyBench, Survey-Arena, SurGE, DeepSurvey-Bench, SurveyLens, ReportBench, SGSimEval.
- Adjacent methodological literatures: scientometrics, citation bias, citation recommendation surveys, long-form evaluation.

However, recall is only moderate because several important reference areas are thinly covered or missing:

- The actual PRISMA 2020 statement and systematic review reporting standards are missing/misattributed.
- SummEval is mentioned but not cited.
- Major long-form summarization and factuality/attribution evaluation works are underrepresented, despite the survey鈥檚 emphasis on survey faithfulness and citation hallucination.
- Key retrieval-augmented generation evaluation and attribution works are largely absent.
- Practical literature-review tools and systems such as Elicit, Semantic Scholar recommendation workflows, Connected Papers, ResearchRabbit, Inciteful, and systematic-review automation tools beyond ASReview/FAST虏 are not discussed.
- Foundational citation-network work such as PageRank-style graph ranking, co-citation analysis, bibliographic coupling, and classic scientometrics are only indirectly referenced.
- The survey claims a 鈥渢axonomy of 135+ papers,鈥?but the bibliography contains roughly 50 entries, creating a mismatch between claimed breadth and documented coverage.

The recall is therefore reasonable for the narrow topic of LLM-based automated survey generation, but incomplete for the broader intersection of citation graph expansion, literature review automation, survey generation, and evaluation.

## S3. Coverage Impact: 3/5

The survey covers several impactful works and concepts:

- SPECTER is correctly treated as a foundational citation-informed embedding model.
- The Semantic Scholar Literature Graph is appropriately framed as important infrastructure.
- STORM, PaperQA, PaperQA2, AutoSurvey, and OpenScholar are central recent systems for LLM-based scientific synthesis or literature QA.
- ASReview is an influential system in active-learning-based systematic review screening.
- Fortunato et al.鈥檚 *Science of Science* is an important scientometric reference.
- The survey gives substantial attention to benchmark fragmentation, citation F1, citation hallucination, and cost reporting 鈥?all high-impact methodological issues for the field.

However, the impact prioritization is uneven:

- The survey devotes extensive space to many 2025鈥?026 systems that may be preliminary, niche, or difficult to verify, while some established, high-impact work in systematic review automation, long-form summarization evaluation, RAG faithfulness, citation recommendation, and scientometrics receives little or no discussion.
- PRISMA is central to systematic reviews but is cited incorrectly and not properly integrated.
- The survey鈥檚 focus on named automated survey systems is useful, but it underweights broader high-impact research on evidence synthesis, claim verification, retrieval evaluation, and citation networks.
- Some 鈥渇rontier鈥?papers are treated as highly consequential despite limited evidence or non-standard benchmarks.

Overall, the survey gives reasonable coverage of influential automated survey-generation systems and citation-aware retrieval papers, but its high-impact coverage is diluted by speculative or weakly verified recent works and by omissions of canonical adjacent literature.

## S4. Recency & Currency: 4/5

The survey is highly current. It includes numerous 2024, 2025, and 2026 works and explicitly frames the field as evolving through phases up to 鈥淐itation Graph Re-integration, 2025鈥?026.鈥?It discusses recent benchmarks such as SurveyScope, SurveyBench, DeepSurvey-Bench, SurveyLens, ReportBench, SGSimEval, and SurveyEval, and recent architectures such as SciSage, Agentic AutoSurvey, MATC, PaSa, SurveyG, and Graphs of Research.

Strengths in recency:

- Strong integration of 2024鈥?026 LLM-based survey-generation systems.
- Good attention to recent multi-agent, RL-guided, and graph-aware developments.
- Up-to-date discussion of evaluation fragmentation and citation verification.
- The survey identifies current open problems rather than only summarizing older work.

Limitations:

- The survey may over-rely on very recent preprints and future-dated benchmark papers without clearly distinguishing mature, validated work from speculative frontier work.
- Some cited works appear difficult to verify or possibly synthetic, which weakens confidence in the currency dimension.
- Recentness sometimes substitutes for impact: several frontier systems are treated as major phase-defining works without enough external validation.

Despite these caveats, the survey is clearly current and reflects many of the latest claimed developments in automated survey generation.

## Score Summary

| Dimension | Score | Notes |
|-----------|:-----:|-------|
| Citation Precision | 2 | Dense citation practice, but multiple misattributions, missing references, and internal inconsistencies |
| Citation Recall | 3 | Covers many relevant systems but misses canonical systematic-review, summarization-evaluation, and attribution literature |
| Coverage Impact | 3 | Includes several high-impact works but overweights speculative recent systems and omits some foundational adjacent literature |
| Recency & Currency | 4 | Very current, with extensive 2024鈥?026 coverage, though some recent citations are hard to verify |
| **Citation Quality (P+R avg)** | **2.50** | Average of Precision and Recall |
| **Overall Average** | **3.00** | Average of all 4 dimensions |

## Assessment Summary

This is an ambitious, well-structured, and highly current survey of automated literature survey generation with a useful organizing theme around citation graph expansion and the semantic鈥搒tructural tension. However, its citation quality is weakened by several clear attribution errors, missing references, internal inconsistencies, and possible overreliance on hard-to-verify recent works. The survey is strongest as a conceptual synthesis and weakest as a rigorously grounded bibliographic artifact.

## Citation Quality Assessment

The survey uses citations extensively and often links claims to named systems, which is a strength. Many major systems are cited in plausible contexts: AutoSurvey for outline-driven generation, STORM for perspective-guided synthesis, PaperQA/PaperQA2 for scientific QA and citation chaining, SPECTER for citation-informed embeddings, Semantic Scholar for literature graph infrastructure, ASReview for active-learning screening, and OpenScholar for large-scale retrieval-augmented scientific synthesis.

However, the citation pattern has several problems:

- **Unsupported or missing references:** STM Global Brief, UNESCO Science Report, and SummEval are invoked but not included in the bibliography.
- **Incorrect attribution:** PRISMA is associated with Waltman et al., 2020, which is not the PRISMA guideline.
- **Inconsistent citation mapping:** SurveyLens is attributed inconsistently to Li et al., 2026 and Chen et al., 2025e.
- **Over-specific numerical claims:** Many exact performance numbers are repeated without enough benchmark detail, baseline detail, or qualification.
- **Possible synthetic citation risk:** A large fraction of 2025鈥?026 citations have generic titles/authors and are treated as established despite appearing preliminary or difficult to verify.
- **Reference-list mismatch:** The text claims a taxonomy of 鈥?35+ papers,鈥?but the reference list contains far fewer cited items.

**Citation F1 Proxy**: Poor

## Strengths

1. **Strong conceptual organization:** The five-phase evolution arc is clear and analytically useful, especially the semantic鈥搒tructural tension between embedding-based retrieval and citation graph-aware retrieval.

2. **Good coverage of core LLM-era survey systems:** AutoSurvey, STORM, PaperQA, PaperQA2, OpenScholar, SciSage, PaSa, SurveyG, and related systems are discussed in a coherent architecture-focused taxonomy.

3. **Excellent attention to evaluation limitations:** The survey insightfully identifies benchmark fragmentation, weak ablations, unmeasured critical-analytic depth, citation hallucination, and cost-reporting gaps.

## Weaknesses/Gaps

1. **Citation precision problems:** The PRISMA misattribution, missing bibliography entries, and inconsistent SurveyLens citations are significant errors for a survey that emphasizes citation quality.

2. **Overreliance on recent or possibly unverified works:** The survey treats many 2025鈥?026 systems and benchmarks as established reference points without clearly distinguishing mature evidence from frontier claims.

3. **Incomplete adjacent-literature coverage:** Important works in systematic review methodology, long-form summarization evaluation, RAG faithfulness, attribution evaluation, and classic citation-network analysis are missing or underdeveloped.

## Recommendations

1. **Audit every citation and reference.** Add missing references for STM, UNESCO, SummEval, and PRISMA; correct the PRISMA citation; resolve SurveyLens/SGSimEval inconsistencies; and verify all arXiv identifiers and author lists.

2. **Separate verified findings from claimed findings.** For each system, distinguish 鈥渞eported by authors,鈥?鈥渧alidated on shared benchmark,鈥?鈥渋ndependently reproduced,鈥?and 鈥渟peculative/future direction.鈥?
3. **Expand foundational and adjacent coverage.** Add canonical works on PRISMA, systematic review automation, long-form summarization evaluation, RAG faithfulness/attribution, citation recommendation, bibliographic coupling, co-citation analysis, and graph ranking.

鈺扳攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
