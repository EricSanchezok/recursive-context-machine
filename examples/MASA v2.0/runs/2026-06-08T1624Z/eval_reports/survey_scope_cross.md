鈺攢 Response 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
# SurveyScope Evaluation Report

## S1. Citation Precision: 2/5

The survey is citation-dense and generally attaches citations to concrete system-level claims, but citation precision is substantially weakened by the reference section itself: every entry is marked 鈥淐itation not verified.鈥?For example, the bibliography repeatedly states 鈥淐itation not verified鈥?for core works such as AutoSurvey, Agentic AutoSurvey, SciSage, SurveyBench, SurveyG, and others. This makes it impossible to treat the survey鈥檚 highly specific claims as reliably attributed.

The text makes many precise empirical claims鈥攅.g., 鈥淎utoSurvey鈥檚 4.77/10,鈥?鈥淎gentic AutoSurvey鈥檚 8.18/10,鈥?鈥淪ciSage鈥?+32% Citation F1,鈥?鈥淚nsightAgent鈥?+27.2% quality improvement,鈥?鈥淪urveyBench鈥?11,343 arXiv topics and 4,947 human surveys鈥濃€攂ut most are supported only by short-name citations rather than full bibliographic metadata, authors, venues, or verified sources. The survey also states that no paper profiles contain author metadata and that author names are not inferred, which is transparent but reduces citation specificity.

There are also places where claims are broad and synthetic but cited only indirectly. For instance, the claim that 鈥渙nly 5 of the 35 core method papers surveyed use citation graph structure鈥?is important, but the survey does not provide a complete list of the 35 papers or a reproducible inclusion criterion. Similarly, claims about 鈥渘o Phase 1鈥? system鈥?being evaluated on later benchmarks are strong field-wide claims but are not backed by a systematic search protocol.

Overall, citations are frequent and usually placed near relevant claims, but the explicit lack of verification and short-anchor citation style substantially lowers precision.

## S2. Citation Recall: 3/5

The survey covers many central papers and systems in the automated survey generation space, especially recent ASG-specific systems: AutoSurvey, Agentic AutoSurvey, SurveyX, SurveyGen, SurveyForge, Meow, SciSage, MATC, KMCA, AutoSurvey2, InsightAgent, SurveyG, LitFM, SurveyGen-I, and IterSurvey. It also includes several evaluation benchmarks: SurveyBench, SurveyEval, SurGE, SurveyLens, DeepSurvey-Bench, and SGSimEval.

The survey also reaches into adjacent work on citation graphs and evaluation, including Semantic Scholar Literature Graph, citation chaining, PageRank, ReAct, LATS, FActScore, VERISCORE, CiteGuard, CiteME, and graph-based citation recommendation methods. This gives the survey reasonable breadth.

However, recall is limited in several ways. The survey focuses heavily on a narrow ASG-preprint ecosystem and does not sufficiently situate ASG within older work on automatic literature review generation, scientific document summarization, multi-document summarization, evidence synthesis, systematic review automation, PRISMA-style screening tools, scientific claim verification, retrieval-augmented generation evaluation, and citation recommendation. It also lacks discussion of influential practical systems and infrastructures beyond Semantic Scholar, such as Connected Papers, Elicit, Scite, Litmaps, ResearchRabbit, or systematic-review automation platforms, if the scope includes automated literature-review support rather than only LLM-generated surveys.

The survey claims to cover 鈥?5 core method papers,鈥?but the visible discussion centers on a smaller subset. Some papers are only listed in references or tables without meaningful discussion. Thus, recall is reasonable for recent ASG systems but incomplete for the broader high-impact literature surrounding survey generation, citation faithfulness, retrieval, and evidence synthesis.

## S3. Coverage Impact: 3/5

The survey covers several likely high-impact or central ASG papers, especially AutoSurvey and Agentic AutoSurvey, which are positioned as foundational and empirically central. It also covers recent benchmark papers that would be important if the field is indeed moving toward standardized evaluation: SurveyBench, SurveyEval, SurGE, SurveyLens, DeepSurvey-Bench, and SGSimEval. The paper also includes some high-impact foundations outside ASG, such as Semantic Scholar, PageRank, ReAct, FActScore, and VERISCORE.

The main strength is that the survey correctly prioritizes the field鈥檚 architectural and evaluative bottlenecks: the single-agent versus multi-agent transition, graph-aware retrieval, iterative refinement, benchmark fragmentation, citation hallucination, and cost opacity. These are consequential issues, and the survey鈥檚 critical framing is impactful.

However, impact coverage is uneven. Many cited works are recent arXiv preprints, and the survey does not clearly distinguish seminal, influential, peer-reviewed, or widely adopted papers from speculative or peripheral new systems. The impact hierarchy is asserted more than demonstrated. For example, the text repeatedly frames AutoSurvey and Agentic AutoSurvey as central, but other systems are described with similar detail even when their influence is unclear. The survey also undercovers older influential work on systematic review automation, scientific summarization, literature-based discovery, citation recommendation, and evidence retrieval, which are important historical and methodological foundations for ASG.

As a result, the survey has reasonable impact coverage within the narrow ASG system literature but does not fully cover the broader high-impact ecosystem.

## S4. Recency & Currency: 5/5

The survey is exceptionally current. It covers work from 2024, 2025, and even 2026, including SurveyLens and DeepSurvey-Bench. The survey鈥檚 central narrative explicitly tracks a 2024鈥?026 evolution and includes very recent systems and benchmarks.

The emphasis on current concerns鈥攃itation hallucination, benchmark fragmentation, multi-agent cost opacity, graph-aware retrieval, and convergence criteria鈥攊s well aligned with contemporary LLM and RAG evaluation debates. The survey also integrates recent factuality and citation-evaluation tools such as VERISCORE, CiteGuard, CiteME, and FActScore.

The main caveat is that currency appears to come at the expense of verification and impact filtering. Many references are marked unverified, and the survey may over-index on very recent arXiv papers whose influence is not yet established. Still, on recency alone, the survey is highly up to date.

## Score Summary

| Dimension | Score | Notes |
|-----------|:-----:|-------|
| Citation Precision | 2 | Citations are frequent but explicitly unverified; many specific empirical claims rely on short-name anchors only. |
| Citation Recall | 3 | Good coverage of recent ASG systems and benchmarks, but limited broader coverage of scientific summarization, systematic review automation, and citation/evidence synthesis literature. |
| Coverage Impact | 3 | Covers several central ASG papers, but impact prioritization is weak and many works are recent/unverified preprints. |
| Recency & Currency | 5 | Very current, including 2025鈥?026 systems, benchmarks, and evaluation concerns. |
| **Citation Quality (P+R avg)** | **2.50** | Average of Precision and Recall |
| **Overall Average** | **3.25** | Average of all 4 dimensions |

## Assessment Summary

This is a timely and well-structured critical survey of automated survey generation, with strong thematic organization around architecture, graph awareness, evaluation gaps, and blind spots. Its major weakness is citation reliability: the survey makes many specific empirical claims while explicitly noting that all references are unverified. Coverage is strong for recent ASG preprints but weaker for broader high-impact work in literature review automation, scientific summarization, citation recommendation, and evidence synthesis.

## Citation Quality Assessment

The survey uses citations frequently and generally places them near the claims they support. This is a strength: system descriptions, benchmark descriptions, and empirical results are usually tied to named sources. For example, AutoSurvey is cited when discussing the Outline 鈫?Retrieve 鈫?Draft 鈫?Refine pipeline, Agentic AutoSurvey is cited for the 8.18/10 versus 4.77/10 comparison, and SciSage is cited for the +32% Citation F1 claim.

However, citation quality is undermined by the reference list. Every entry is marked 鈥淐itation not verified,鈥?including the most important sources. The survey also uses short project-name anchors rather than conventional author-year citations, and it does not provide authors, venues, DOIs, or verified metadata. This makes it hard to assess whether claims are correctly attributed or whether some titles, arXiv IDs, or results have been hallucinated or misremembered.

The survey is transparent about this limitation, stating that references use short system/project names because profiles lacked author or metadata fields. That transparency is useful, but it does not solve the citation-precision problem. For SurveyScope purposes, where citation attribution and coverage impact are central, the lack of verified references is a serious flaw.

**Citation F1 Proxy**: Acceptable-to-Poor  
The survey has reasonable recall within its chosen ASG scope but poor-to-moderate precision because the cited evidence is not verified.

## Strengths

1. **Strong topical organization**: The phase-based structure and architectural taxonomy make the ASG landscape easy to follow.
2. **Excellent recency**: The survey captures very recent systems, benchmarks, and concerns through 2026.
3. **Good critical framing**: The discussion of the 鈥渃ontrolled comparison gap,鈥?benchmark fragmentation, citation hallucination, cost opacity, and missing ablations is insightful and valuable.

## Weaknesses/Gaps

1. **Unverified citations throughout**: The reference section explicitly marks every citation as unverified, which severely limits confidence in attribution.
2. **Insufficient broader literature grounding**: The survey undercovers older and adjacent high-impact work in systematic review automation, scientific summarization, evidence synthesis, and citation recommendation.
3. **Impact prioritization is unclear**: The survey treats many recent systems as comparable without clearly distinguishing seminal, highly cited, benchmark-setting, or widely adopted works from speculative preprints.

## Recommendations

1. **Verify and normalize the bibliography**: Add authors, venues, links/DOIs/arXiv metadata, and remove or qualify any claims that cannot be verified against the cited paper.
2. **Add a systematic inclusion protocol**: If the survey covers 鈥?5 core method papers,鈥?list them explicitly and explain search sources, inclusion criteria, exclusion criteria, and coverage boundaries.
3. **Broaden the foundation section**: Include more high-impact work on scientific multi-document summarization, systematic review automation, PRISMA-style evidence synthesis, citation recommendation, RAG evaluation, and factuality/citation faithfulness.

鈺扳攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
