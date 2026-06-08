鈺攢 Response 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
# SurveyEval Evaluation Report

## D1. Overall Quality: 4/5

The survey is clear, readable, and analytically strong. It presents a focused thesis 鈥?that automated survey generation has advanced architecturally faster than it has advanced evaluatively 鈥?and sustains that thesis throughout the paper. The introduction clearly frames the scope through four research questions:

> 鈥淭his survey is organized around four research questions: (1) How have ASG architectures evolved鈥? (2) What evidence supports the claimed advances鈥? (3) What is the current state of ASG evaluation鈥? (4) What blind spots鈥?remain unaddressed?鈥?
The paper is especially strong in its critical synthesis. Sections such as **鈥?.4 The Controlled Comparison Gap鈥?*, **鈥?.1 Claim vs. Evidence 鈥?Systematic Audit鈥?*, and **鈥?.4 Blind Spots 鈥?What the Field Is Not Looking At鈥?* go beyond summary and evaluate the evidentiary status of the field. For example:

> 鈥淭he field's central empirical question 鈥?do multi-agent architectures outperform single-agent pipelines? 鈥?rests on a single data point.鈥?
This is a useful and academically meaningful organizing claim. The survey also provides helpful comparative tables, including architecture comparisons, benchmark comparisons, and claim-evidence audits.

However, the quality is weakened by several issues. First, some strong claims are not sufficiently substantiated, such as:

> 鈥淥nly 5 of the 35 core method papers surveyed use citation graph structure鈥︹€?
The survey does not provide a transparent inclusion/exclusion protocol for the 鈥?5 core method papers,鈥?so the systematicity of the review is unclear. Second, the paper鈥檚 own references are explicitly marked as unverified, which undermines its academic rigor. Third, some language is rhetorically strong 鈥?鈥渆valuation comparability crisis,鈥?鈥渢he field has collectively avoided,鈥?鈥渁rchitectural fashion鈥?鈥?and while persuasive, it occasionally outpaces the documented evidence.

Overall, this is a high-quality, well-written, analytically useful survey, but not publication-quality because of reference verification problems and insufficient methodological transparency.

## D2. Outline Coherence: 5/5

The outline is highly coherent and has a strong narrative arc. The survey begins with scope and research questions, then moves chronologically through the evolution of the field, then structurally through architectural paradigms, then critically through evidence gaps, and finally into future directions. The paper explicitly previews this structure:

> 鈥淪ection 2 traces the five-phase evolution of ASG. Section 3 dissects the three core architectural paradigms鈥?Section 4 analyzes the spectrum of citation graph awareness鈥?Section 5 provides a critical assessment鈥?Section 6 outlines future directions鈥︹€?
Section 2 provides a useful historical progression:

> 鈥淎SG has passed through five overlapping phases in under three years鈥︹€?
The phase structure 鈥?**Phase 0: Pre-LLM Foundations**, **Phase 1: The Single-Agent Pipeline**, **Phase 2: The Multi-Agent Explosion**, **Phase 3: Graph Awareness and Iterative Refinement**, and **Phase 4: Evaluation Maturation** 鈥?creates a clear temporal framework.

Sections 3 and 4 then shift from chronology to architectural analysis, which is logical because the reader has already been introduced to the systems. The transition into Section 5 is also effective: after describing architectures, the survey evaluates whether the claims about those architectures are supported. The structure of Section 5 is especially strong:

> 鈥淲e examine six major claims against their supporting evidence鈥?identify five methodological weaknesses鈥?quantify the evaluation comparability crisis鈥?and catalog five blind spots鈥︹€?
The conclusion successfully returns to the four research questions and synthesizes the central argument:

> 鈥淎SG architectures have evolved rapidly, but the transitions were driven as much by architectural fashion as by measured deficiencies鈥︹€?
The outline is therefore not merely organized; it is argumentative and cumulative. Each section builds on the previous one, making the survey easy to follow and intellectually coherent.

## D3. Reference Accuracy: 2/5

Reference accuracy is the weakest dimension. The survey includes many citations and uses them consistently in the body, but the reference list explicitly states that the citations are not verified:

> 鈥淩eferences use short system/project names as citation anchors鈥?because no paper profile鈥?contains `authors` or `metadata_source` fields. Per instructions, author names are not inferred from model knowledge.鈥?
Nearly every reference is then labeled:

> 鈥淐itation not verified.鈥?
Examples include:

> 鈥淸Agentic AutoSurvey, 2025] Citation not verified. 鈥楢gentic AutoSurvey: Let LLMs Survey LLMs.鈥?arXiv:2509.18661, 2025.鈥?
and:

> 鈥淸SurveyEval, 2025] Citation not verified. 鈥楽urveyEval: Multi-Subject Evaluation of Survey Generation.鈥?arXiv:2512.02763, 2025.鈥?
This substantially reduces confidence in the survey鈥檚 factual grounding. The paper relies heavily on these citations for specific numerical claims, such as:

> 鈥?.18/10 vs. AutoSurvey's 4.77/10鈥?
> 鈥?32% Citation F1 improvement鈥?
> 鈥?27.2% quality improvement and reducing timeline from months to 1.5 hours鈥?
But because the references are not verified and no author names, venues, or reliable metadata are provided, these claims cannot be confidently assessed.

The reference coverage appears broad across ASG systems, evaluation benchmarks, citation graph methods, factuality metrics, and multi-agent evaluation literature. However, breadth does not compensate for the lack of verification. The use of project-name citation anchors is understandable given the stated constraints, but it falls below normal academic standards. The reference section itself essentially admits that citation correctness is uncertain.

Thus, the survey has reasonable topical coverage but poor formal citation reliability.

## Score Summary

| Dimension | Score | Weight | Weighted |
|-----------|:-----:|:------:|:--------:|
| Overall Quality | 4 | 40% | 1.60 |
| Outline Coherence | 5 | 30% | 1.50 |
| Reference Accuracy | 2 | 30% | 0.60 |
| **Total** | 鈥?| **100%** | **3.70** |

## Assessment Summary

This is a well-structured and analytically strong survey that provides a compelling critical account of automated survey generation, especially around evaluation gaps and architectural claims. Its major weakness is reference reliability: the paper openly marks nearly all citations as unverified, which substantially limits its academic rigor despite its strong organization and synthesis.

## Strengths

1. Strong central thesis around the 鈥渆vidence gap鈥?and 鈥渆valuation comparability crisis.鈥?2. Excellent structure, with a clear progression from historical evolution to architecture, evidence audit, blind spots, and future directions.
3. Useful comparative tables that clarify architectural paradigms, evaluation metrics, graph-awareness levels, and claim-evidence relationships.

## Weaknesses/Gaps

1. References are explicitly unverified, and most citations lack authors, venues, and reliable metadata.
2. Several strong claims, such as 鈥渙nly 5 of the 35 core method papers,鈥?lack a transparent survey methodology or inclusion criteria.
3. The prose sometimes uses rhetorically forceful language that would benefit from more cautious academic framing.

## Recommendations

1. Verify all references, add author names, venues, publication status, URLs/DOIs/arXiv links, and ensure that all numerical claims are traceable to cited sources.
2. Add a methodology section explaining corpus construction, inclusion/exclusion criteria, and how the 鈥?5 core method papers鈥?were selected.
3. Temper broad claims or support them with explicit evidence, especially claims about field-wide avoidance, benchmark adoption, and absence of ablation studies.

鈺扳攢鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈺?
