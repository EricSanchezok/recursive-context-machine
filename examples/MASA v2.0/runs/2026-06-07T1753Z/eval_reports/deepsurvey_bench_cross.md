鈺攢 Response 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
# DeepSurvey-Bench Evaluation Report

## Layer 1: Surface Quality

### SQ1. Outline Quality: 5/5

The survey has an expert-level organization with a clear macro-argument and recurring analytical threads. It begins with a precise scope statement in **鈥? Introduction and Scope鈥?*, then builds a chronological framework in **鈥? The Evolution Arc 鈥?Five Phases of Automated Survey Generation鈥?*, followed by mechanism-level analysis in **鈥? Architectural Deep Dive鈥?*, cross-cutting methodological strategies in **鈥? Cross-Cutting Strategies鈥?*, a critical field-level diagnosis in **鈥? Critical Assessment鈥?*, and actionable proposals in **鈥? Future Directions.鈥?*

The outline is not merely descriptive; it creates a coherent taxonomy around citation graph expansion and the 鈥渟emantic鈥搒tructural tension.鈥?The survey explicitly states its organizing logic:

> 鈥渋t traces a five-phase evolution narrative organized around the central tension between semantic content and structural context.鈥?
The cross-phase comparison table in 搂2 and the recurring 鈥淭hread鈥?framing in 搂5鈥撀? help maintain continuity across a large amount of material. The structure supports both historical narration and analytical comparison, which is a major strength.

### SQ2. Content Quality: 4/5

The content is detailed, analytical, and generally well-developed. The survey goes beyond listing systems by explaining mechanisms, limitations, and field-level consequences. For example, 搂3.4 introduces the **鈥渂ottleneck transfer problem鈥?*:

> 鈥渢he untested assumption that improving retrieval components linearly improves survey outcomes.鈥?
This is a strong conceptual contribution. The survey also provides useful mechanism-level descriptions of systems such as SPECTER, LitFM, SurveyG, SciSage, PaSa, IterSurvey, and PaperQA2.

The critical sections are especially strong. 搂5.1鈥檚 **鈥淐laim vs. Evidence Gap Analysis鈥?* distinguishes between reported performance and what the evidence actually supports, e.g.:

> 鈥淏enchmark tests factual recall and summarization, not survey-quality synthesis.鈥?
However, the content has some weaknesses. Several claims appear overconfident or under-verified. The introduction claims a **鈥渃ritical taxonomy of 135+ papers鈥?*, but the reference list contains far fewer cited works, and the phase table lists much smaller paper pools. Some referenced benchmarks and future-dated works may be plausible in the survey鈥檚 imagined context, but the survey does not always provide enough grounding to verify them. There are also some questionable or incorrect contextual references, such as attributing PRISMA-like systematic review rigor to **Waltman et al., 2020**, which is not the standard PRISMA reference.

Overall, the content is strong and analytical, but not flawless in factual precision or evidentiary grounding.

### SQ3. Reference Quality: 3/5

The reference coverage is broad and includes many relevant systems: **AutoSurvey, STORM, PaperQA, PaperQA2, OpenScholar, SciSage, Agentic AutoSurvey, PaSa, IterSurvey, SurveyG, LitFM, SPECTER, HiGTL, SurveyBench, DeepSurvey-Bench, SurveyLens**, and others. This gives the survey good apparent breadth.

However, reference quality is weakened by several issues:

1. Some important claims in the introduction cite sources that are not included in the reference list, e.g.:
   > 鈥淪TM Global Brief 2023; UNESCO Science Report 2021鈥?
2. SummEval is mentioned as an evaluation tradition, but no SummEval reference appears in the bibliography.

3. The PRISMA discussion appears misreferenced:
   > 鈥淭he PRISMA framework for systematic reviews [Waltman et al., 2020]鈥?   
   Waltman et al. is not the canonical PRISMA statement.

4. There are inconsistencies in benchmark references. For example, 搂6.4 says:
   > 鈥淪urveyLens鈥檚 discipline-aware design [Li et al., 2026]鈥?   
   but 搂6.1鈥檚 prioritization table lists:
   > 鈥淪urveyLens [Chen et al., 2025e]鈥?
5. The claim of **鈥?35+ papers鈥?* is not reflected in the actual reference list.

The references are numerous and mostly relevant, but the citation discipline is not expert-curated enough for a top score.

**Surface Quality Average: 4.00/5**

---

## Layer 2: Academic Value

### AV1. Informational Value: 4/5

The survey is highly informative. It provides a broad map of automated literature survey agents, especially around citation graph expansion, LLM-based retrieval-augmented generation, multi-agent architectures, iterative refinement, RL-guided search, and evaluation benchmarks.

Strong informational passages include 搂2鈥檚 five-phase evolution, 搂3鈥檚 architectural comparisons, and 搂5.3鈥檚 benchmark landscape. The benchmark table in 搂5.3 is particularly valuable because it summarizes the field鈥檚 fragmentation across:

> 鈥淟itQA, LitQA2, ScholarQABench, SurveyScope, Survey-Arena, SurveyBench, SurGE, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval, and SurveyEval.鈥?
The survey also explains why graph structure matters:

> 鈥淐itation graphs encode at least four complementary signals invisible to text-only representations: intellectual lineage, role differentiation, community boundaries, temporal evolution.鈥?
This is informative and conceptually useful.

The main limitation is that some informational claims are not sufficiently verifiable from the references provided. The survey sometimes reports precise performance numbers, system details, or benchmark properties without enough citation specificity or with potentially inconsistent references. Therefore, it is highly informative but not definitive.

### AV2. Scholarly Communication Value: 5/5

The survey excels at scholarly framing. It does not merely summarize papers; it positions them relative to one another and identifies tensions, confounds, and unresolved debates.

A particularly strong example is the discussion of multi-agent systems:

> 鈥渢he comparison is confounded: Agentic AutoSurvey uses a more capable base LLM, better retrieval, and a different evaluation rubric than the original AutoSurvey.鈥?
This is exactly the kind of nuanced contextualization expected in a high-value survey. Similarly, 搂5.1鈥檚 claim-evidence table provides balanced treatment of major claims:

> 鈥淕enuine improvement, but baselines differ across papers鈥?No controlled ablation studies isolating the architectural contribution.鈥?
The survey also repeatedly identifies field-level tensions:

- semantic retrieval vs. citation graph structure;
- retrieval gains vs. survey-quality gains;
- surface quality vs. critical-analytic depth;
- automation scalability vs. human oversight;
- benchmark proliferation vs. meaningful comparison;
- cost vs. quality.

The conclusion synthesizes these threads clearly:

> 鈥淭he next breakthrough will come not from a better pipeline鈥?but from the evaluation infrastructure that makes pipeline comparison meaningful.鈥?
This is strong scholarly communication with clear positioning and critical comparison.

### AV3. Research Guidance Value: 5/5

The survey provides concrete, well-motivated, and actionable research directions. 搂6 is especially strong. It does not offer generic 鈥渇uture work鈥? instead, it proposes specific architectural and evaluation interventions.

Examples include the proposed unified architecture in 搂6.1, combining:

> 鈥淟itFM鈥檚 graph transformer,鈥? 
> 鈥淪urveyG鈥檚 three-layer hierarchical graph,鈥? 
> 鈥淧aSa鈥檚 RL policy,鈥? 
> 鈥淪ciSage鈥檚 reflect-while-writing framework,鈥? 
> and 鈥淐iteGuard鈥?citation validation.

The survey also proposes specific extensions to PaSa-style traversal policies:

> 鈥渉orizontal expand,鈥?鈥渧ertical ascend,鈥?鈥渧ertical descend,鈥?鈥渓ayer-switch,鈥?and 鈥渟top.鈥?
Most importantly, 搂6.3 offers a concrete evaluation framework with five dimensions:

1. critical-analytic depth;
2. bias awareness;
3. field-situatedness;
4. citation hallucination audit;
5. standardized cost reporting.

This is actionable and directly follows from the critical assessment. The prioritization table in 搂6 also distinguishes impact and feasibility, which adds practical research guidance:

> 鈥淩eimagined evaluation has the highest impact鈥?but the lowest feasibility.鈥?
This is exemplary future-work guidance.

**Academic Value Average: 4.67/5**

---

## Score Summary

| Dimension | Score | Weight | Weighted |
|-----------|:-----:|:------:|:--------:|
| Surface Quality Average | 4.00 | 40% | 1.600 |
| Academic Value Average | 4.67 | 60% | 2.800 |
| **Overall** | 鈥?| **100%** | **4.40** |

## Assessment Summary

This is a strong, academically valuable survey with an unusually coherent analytical frame around citation graph expansion, graph-blind LLM pipelines, benchmark fragmentation, and the missing evaluation of scholarly value. Its main weakness is not organization or insight, but reference discipline: several citations are missing, inconsistent, or insufficiently grounded, and some quantitative claims would need verification before the survey could serve as a definitive reference.

## Strengths

1. **Excellent conceptual framing.** The recurring threads 鈥?semantic鈥搒tructural tension, evaluation comparability crisis, bottleneck transfer problem, and critical-analytic blind spot 鈥?give the survey a strong intellectual identity.

2. **Strong critical analysis of claims.** The survey repeatedly distinguishes reported performance from supported conclusions, especially in 搂5.1鈥檚 claim-evidence table.

3. **Actionable research agenda.** 搂6 offers concrete architectural and evaluation proposals rather than vague future-work statements.

## Weaknesses/Gaps

1. **Citation and reference inconsistencies.** Several sources mentioned in the text are absent from the bibliography, and some references appear misattributed or inconsistent.

2. **Overstated coverage claim.** The survey claims a taxonomy of 鈥?35+ papers,鈥?but the bibliography and internal phase counts do not substantiate that number.

3. **Some quantitative claims need stronger grounding.** Performance improvements such as 鈥?32% citation F1,鈥?鈥?37.78% recall@20,鈥?and 鈥?28.1% precision鈥?are useful, but the survey often does not provide enough benchmark detail to assess their validity or comparability.

## Recommendations for Improving Academic Value

1. **Audit and normalize all references.** Add missing references for STM, UNESCO, SummEval, PRISMA, and any benchmark or system mentioned in the text; correct misattributions and inconsistent author-year labels.

2. **Add an evidence table with verification metadata.** For each system, include dataset, metric definition, baseline, absolute score, relative gain, evaluation type, and whether the result is peer-reviewed, arXiv-only, or unpublished.

3. **Align the claimed corpus size with the bibliography.** If the survey covers 135+ papers, provide either a full extended bibliography or a supplementary taxonomy table listing all papers and their categories.

鈺扳攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
