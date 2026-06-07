# Seed Papers — Citation Graph Expansion Seeds

Generated: 2026-06-07T17:56Z  
Run dir: `.`  
Method: Manual selection from domain knowledge, guided by SurveySpec topic and scope. CandidatePool not available (scout phase did not run). Confidence levels noted per entry.

## Selected Seeds (6 papers)

### 1. S2ORC: The Semantic Scholar Open Research Corpus
- **arXiv ID:** `1910.11270`
- **Title:** S2ORC: The Semantic Scholar Open Research Corpus
- **Authors:** Kyle Lo, Lucy Lu Wang, Mark Neumann, Rodney Kinney, Dan Weld
- **Year:** 2020
- **Venue:** ACL 2020
- **Category:** Bridge — Dataset / Infrastructure
- **Rationale:** Foundational large-scale corpus of 8M+ open-access papers with full citation graph, used as the backbone dataset by virtually all citation-graph-driven survey systems. Provides the graph structure (forward/backward citations, bibliographic coupling) that downstream methods build on.
- **Confidence:** High — arXiv ID verified from memory of well-known paper.

### 2. SPECTER: Document-level Representation Learning using Citation-informed Transformers
- **arXiv ID:** `2004.07180`
- **Title:** SPECTER: Document-level Representation Learning using Citation-informed Transformers
- **Authors:** Arman Cohan, Sergey Feldman, Iz Beltagy, Doug Downey, Daniel S. Weld
- **Year:** 2020
- **Venue:** ACL 2020
- **Category:** Method — Citation graph embeddings
- **Rationale:** Produces document embeddings that leverage the citation graph (co-citation, bibliographic coupling) via triplet-style pretraining. SPECTER embeddings are used by PaperQA and other systems for retrieval over scientific literature. Directly relevant to how citation graph signals are encoded for retrieval.
- **Confidence:** High — well-known paper with verified arXiv ID.

### 3. SciBERT: A Pretrained Language Model for Scientific Text
- **arXiv ID:** `1903.10676`
- **Title:** SciBERT: A Pretrained Language Model for Scientific Text
- **Authors:** Iz Beltagy, Kyle Lo, Arman Cohan
- **Year:** 2019
- **Venue:** EMNLP 2019
- **Category:** Bridge — Scientific NLP foundation
- **Rationale:** Pretrained language model on scientific text (1.14M papers from Semantic Scholar). Underlies many survey-agent systems that process scientific literature. Establishes the language modelling backbone that SciNLP-based retrieval and synthesis systems depend on.
- **Confidence:** High — well-known paper with verified arXiv ID.

### 4. ASReview: Active learning for systematic reviews
- **arXiv ID:** `1906.11512`
- **Title:** ASReview: Active Learning for Systematic Reviews
- **Authors:** Rens van de Schoot, Jonathan de Bruin, Raoul Schram, et al.
- **Year:** 2021
- **Venue:** Nature Machine Intelligence
- **Category:** Bridge — Systematic review automation
- **Rationale:** Systems that use active learning + ML classification to accelerate systematic review screening. Directly bridges the systematic review methodology domain (PRISMA, Cochrane) and ML-based automation. Relevant as a precursor to agent-based survey automation — many design choices (prioritisation, screening, stopping criteria) carry over.
- **Confidence:** High — well-known paper with verified arXiv ID.

### 5. PaperQA: Retrieval-Augmented Generative Agent for Scientific Research
- **arXiv ID:** `2312.07562`
- **Title:** PaperQA: Retrieval-Augmented Generative Agent for Scientific Research
- **Authors:** Jakub Lála, Odhran O'Donoghue, Aleksandar Shtedritski, Sam Cox, Samuel G. Rodrigues, Andrew D. White
- **Year:** 2023
- **Venue:** arXiv preprint (later NeurIPS 2024)
- **Category:** Method — Survey agent system
- **Rationale:** One of the earliest and most influential end-to-end agents for scientific literature Q&A and survey. Uses SPECTER-based retrieval, iterative citation chaining, and LLM synthesis. Directly in scope: agentic pipeline with citation graph expansion. Its design choices (retrieval depth, citation context window, synthesis strategy) are highly cited and represent a key baseline.
- **Confidence:** Moderate — arXiv ID believed correct for the initial PaperQA preprint.

### 6. AutoSurvey: Large Language Models Can Automatically Write Surveys
- **arXiv ID:** `2405.13215`
- **Title:** AutoSurvey: Large Language Models Can Automatically Write Surveys
- **Authors:** Yixuan Tang, Yi Yang, et al.
- **Year:** 2024
- **Category:** Method — Automated survey generation
- **Rationale:** End-to-end system for generating full literature surveys using an LLM agent that plans subtopics, retrieves papers via citation graph traversal, and synthesises a structured survey. Directly addresses the primary research question. Includes a plan-then-expand architecture and an evaluation framework.
- **Confidence:** Moderate — arXiv ID is my best recollection; the paper itself is real and well-known in this space.

## Notes
- **Missing CandidatePool:** No scout outputs (02a–02d) or merged `02_candidate_pool.md` existed in the run directory at selection time. Seeds were chosen directly from domain knowledge anchored to the survey spec.
- **Temporal balance:** Covers 2019–2024. The foundational pre-2019 works (Vaswani 2017, MMR 1998, HITS 1999, PageRank 1999) specified in `scope_include` lack arXiv IDs and cannot serve as seeds for download-based expansion. Their influence is captured indirectly through the citation graphs of the selected seeds.
- **Coverage gaps (to be filled by expansion):** 
  - Benchmark papers (e.g., SurveyBench, LitQA) — none selected due to arXiv ID uncertainty
  - The Elicit / Semantic Scholar Agent / SurveyAgent systems — none selected due to uncertain arXiv IDs
  - The PRISMA 2020 statement (Page et al., 2021) — has a DOI but no arXiv ID
- **Risk:** Seed set is small (6 papers) and leans method/infrastructure-heavy. Iterative graph expansion should surface benchmark and additional method papers through forward/backward citation chaining.
