鈺攢 Response 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
# SurveyScope Evaluation Report

## S1. Citation Precision: 3/5

The survey is generally citation-dense and often attributes concrete claims to specific papers or arXiv IDs, especially in the comparison tables and the 鈥淐laim vs. Evidence Gap鈥?section. Examples of good precision include the discussion of RAG as foundational to retrieval-augmented generation with citation to Lewis et al. `[arXiv:2005.11401]`, the dataset table citing SciFact, Multi-XScience, SciReviewGen, FActScore, SciTLDR, and MS虏, and Table 9 explicitly tying claims to sources such as AutoSurvey `[2406.10252]`, PaperQA2 `[2409.13740]`, and SciSage `[2506.12689]`.

However, precision is weakened by several issues:

- Many citations are bare arXiv IDs without authors, titles, venues, or bibliography entries, making attribution hard to verify.
- Some broad technical claims are under-supported, e.g. 鈥渧irtually every automated survey generation system鈥?inherits passage-level RAG limitations, or 鈥渁ll graph traversal is single-hop BFS.鈥?- Several quantitative or empirical statements lack clear citation support, such as ReClaim requiring 鈥渁pproximately 10 API calls per sentence,鈥?local quantized models achieving 鈥?0鈥?5%鈥?of cloud-model benchmark performance, and claims about mean/median evaluation topic counts across 12 systems.
- Some foundational attributions are slightly oversimplified: RAG is correctly cited, but dense passage retrieval as a 鈥渢ypical鈥?retriever should also cite DPR/Karpukhin et al.; the RAG marginalization formula is simplified and not clearly tied to the exact original formulation.

Overall, most claims are at least plausibly attached to specific papers, but the survey mixes well-cited source claims with uncited analytic extrapolations and system-wide generalizations.

## S2. Citation Recall: 3/5

The survey covers a substantial set of directly relevant automated survey generation systems and benchmarks, including AutoSurvey, SurveyX, SciSage, SurveyForge, SurveyG, IterSurvey, ARISE, Agentic AutoSurvey, AutoSurvey2, DOVA, OrchMAS, ResearchPilot, SurveyBench, SurveyScope, Survey-Arena, SurveyLens, DeepSurvey-Bench, and CiteRAG. It also includes important enabling datasets and evaluation resources such as SciFact, Multi-XScience, SciReviewGen, FActScore, SciTLDR, and MS虏.

That said, recall is incomplete in important adjacent areas:

- Foundational retrieval literature beyond RAG is under-cited: DPR, REALM, FiD, RETRO, Atlas, ColBERT, Contriever, and modern scientific retrieval systems are mostly absent.
- Long-form grounded generation systems such as STORM and related outline/question-driven long-form generation work are missing despite clear relevance.
- Citation recommendation, citation context analysis, scientific claim verification, and scholarly graph retrieval literatures are only lightly represented.
- Multi-document summarization foundations prior to 2020 are mostly omitted, even though related-work generation and survey generation build directly on that tradition.
- Practical/industrial literature-review tools and human-AI literature review systems such as Elicit-like workflows, Semantic Scholar tools, and systematic review automation literature are only partially reflected through PROMPTHEUS and CRUISE-Screening.
- The survey鈥檚 scope is focused on arXiv-era ASG systems, which is reasonable, but it sometimes presents field-wide conclusions without sufficiently covering the broader literature that would support those conclusions.

The recall is therefore reasonable for recent automated survey generation preprints, but not comprehensive for the broader research ecosystem.

## S3. Coverage Impact: 3/5

The survey covers several high-impact or likely high-impact works for the specific topic of automated survey generation: RAG `[2005.11401]`, SciFact, Multi-XScience, FActScore, AutoSurvey, PaperQA2, SurveyBench, SciSage/SurveyScope, SurveyForge, SurveyG, and IterSurvey. It also correctly emphasizes major field-level issues: evaluation fragmentation, citation faithfulness, retrieval breadth, iterative refinement, and multi-agent orchestration.

However, the impact prioritization is uneven:

- Many covered systems appear to be very recent arXiv preprints with uncertain influence, while older and broader influential works in retrieval, multi-document summarization, scientific QA, citation graph mining, and long-form generation receive little or no attention.
- The survey gives extensive space to speculative or frontier 2026 systems such as DOVA, OrchMAS, ResearchPilot, SurveyLens, DeepSurvey-Bench, and CiteRAG, but gives less attention to established retrieval and summarization foundations that shape the field.
- The citation graph discussion would be stronger with high-impact graph/citation resources such as Semantic Scholar/OpenAlex papers, citation intent classification, citation recommendation, and scientific knowledge graph literature.
- The evaluation section is strong in recent ASG-specific benchmarks but does not deeply connect to summarization evaluation standards such as ROUGE鈥檚 limitations, BERTScore, QAGS, SummaC, SummEval, or factual consistency evaluation beyond FActScore.

The survey captures many impactful recent ASG papers, but its impact coverage is skewed toward a narrow, recent preprint cluster rather than the full set of influential foundations.

## S4. Recency & Currency: 5/5

The survey is exceptionally current. It includes 2025 and 2026 systems, benchmarks, and frontier directions, including ARISE `[2511.17689]`, AutoSurvey2 `[2510.26012]`, SurveyBench `[2510.03120]`, SurveyLens `[2602.11238]`, DeepSurvey-Bench `[2601.15307]`, CiteRAG `[2601.14949]`, DOVA `[2603.13327]`, OrchMAS `[2603.03005]`, ResearchPilot `[2603.14629]`, and SciAtlas `[2605.22878]`.

The survey also does more than merely list recent work: it integrates recent systems into a coherent timeline from 2020鈥?026 and identifies emerging trends such as deliberation-first architectures, domain-specialized agents, local-first deployment, and benchmark fragmentation. This is one of the strongest dimensions of the survey.

The only caveat is that extreme recency increases the risk of relying on unvalidated preprints, but for the purpose of currency the survey is highly up to date.

## Score Summary

| Dimension | Score | Notes |
|-----------|:-----:|-------|
| Citation Precision | 3 | Many claims are specifically cited, but there are uncited extrapolations, bare arXiv IDs, and some overbroad claims. |
| Citation Recall | 3 | Good coverage of recent ASG papers, but important adjacent retrieval, summarization, scientific QA, citation graph, and long-form generation literature is missing. |
| Coverage Impact | 3 | Covers several high-impact ASG and RAG works, but overweights very recent preprints and underweights foundational/high-impact adjacent work. |
| Recency & Currency | 5 | Very current, including many 2025鈥?026 systems and benchmarks. |
| **Citation Quality (P+R avg)** | **3.00** | Average of Precision and Recall |
| **Overall Average** | **3.50** | Average of all 4 dimensions |

## Assessment Summary

This is a strong, timely, and ambitious survey of automated survey generation, with especially good coverage of recent multi-agent, graph-enhanced, and evaluation-benchmark developments. Its main weaknesses are citation-verification discipline and broader literature coverage: it relies heavily on recent arXiv systems, uses bare arXiv IDs, and sometimes makes sweeping field-level claims without enough support from foundational or adjacent literature.

## Citation Quality Assessment

The survey has a high density of citations and frequently links named systems to specific arXiv identifiers. Tables are particularly useful for attribution, and Table 9鈥檚 鈥淐laim vs. Evidence Gap鈥?is a strong citation-quality feature because it explicitly separates paper claims, evidence, and critical assessment.

However, citation quality is limited by three recurring patterns. First, the survey often cites systems by arXiv ID only, without a bibliography, author names, or full paper titles. Second, many analytical claims are not directly sourced, especially cost estimates, quantitative summaries across papers, and assertions about field-wide limitations. Third, some claims about 鈥渁ll systems,鈥?鈥渘o system,鈥?or 鈥渢he field鈥?require stronger evidence than the cited examples provide.

**Citation F1 Proxy**: Acceptable

## Strengths

1. **Excellent recency**: The survey incorporates a large number of 2025鈥?026 papers and benchmarks, making it highly current.
2. **Clear system-level organization**: The phase-based structure from foundational RAG to multi-agent and frontier systems is coherent and easy to follow.
3. **Strong critical framing**: The four narrative threads 鈥?evaluation comparability, automation-control tension, citation graph shallowness, and depth-breadth trade-off 鈥?provide a useful analytical lens.

## Weaknesses/Gaps

1. **Insufficient foundational recall**: Important retrieval, multi-document summarization, long-form generation, citation recommendation, and scientific QA works are missing or underdeveloped.
2. **Overreliance on recent preprints**: Many central claims depend on very recent arXiv papers whose influence and validity are not yet established.
3. **Unsupported quantitative extrapolations**: Cost estimates, benchmark-performance claims, evaluation-scope statistics, and quantized-model performance claims need citations or methodological explanation.

## Recommendations

1. **Add a full bibliography and richer citation metadata**: Include authors, titles, venues, and years rather than relying only on arXiv IDs.
2. **Strengthen adjacent-literature coverage**: Add key work on DPR, REALM, FiD, ColBERT, Contriever, Atlas, STORM, SummEval, QAGS/SummaC-style factuality evaluation, citation recommendation, citation intent, and scientific knowledge graphs.
3. **Separate sourced facts from author analysis**: Clearly mark when a claim is directly reported by a paper versus when it is the survey author鈥檚 synthesis or estimate, especially for cost, scale, and 鈥渘o system does X鈥?claims.

鈺扳攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
