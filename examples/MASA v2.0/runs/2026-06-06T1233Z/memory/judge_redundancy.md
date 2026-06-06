## Redundancy Evaluation

### Score: 4 — Good

Most information appears in its most logical location. Cross-references are used effectively throughout to avoid re-describing content already established in other sections. The draft follows the outline's explicit cross-referencing guidance in most cases. Some mild redundancy exists — primarily OpenScholar described across two sections, and a few methods re-mentioned without cross-references in Section 8.1 — but overall the duplication does not significantly harm readability.

---

### Redundant Content

1. **OpenScholar (arXiv:2411.14199) — Described in Section 4.4 and Section 7.4**

   **Section 4.4 (line 136):** *"OpenScholar [arXiv:2411.14199] is a retrieval-augmented model specifically designed for scientific literature synthesis, incorporating a large-scale datastore of 45 million open-access papers and a structured retrieval pipeline that identifies the most relevant passages for each section."*

   **Section 7.4 (line 265):** *"OpenScholar [arXiv:2411.14199] represents a significant step beyond traditional survey generation toward comprehensive scientific synthesis. Rather than generating a survey from a set of retrieved papers, OpenScholar maintains a 45-million-paper datastore and supports interactive scientific reasoning — answering questions, comparing methods, and generating synthesis text — grounded in the full open-access literature."*

   **Why redundant:** Both sections characterize OpenScholar's purpose and both mention the distinctive 45-million-paper datastore. Section 7.4 does not cross-reference Section 4.4. The new content in Section 7.4 (interactive reasoning, method comparison, gap identification) could stand alone with a cross-reference.

   **Suggested fix:** Replace the OpenScholar description in Section 7.4 with a cross-reference: *"OpenScholar [arXiv:2411.14199] (described in Section 4.4) extends the retrieval-augmented paradigm toward comprehensive scientific synthesis, supporting interactive reasoning — answering questions, comparing methods, and generating synthesis text — grounded in the full open-access literature. ResearchAgent [arXiv:2404.07738]..."*

2. **FActScore (arXiv:2305.14251) — Mentioned in Sections 5.4, 6.4, and 8.1**

   **Section 5.4 (line 170):** Canonical description — *"FActScore [arXiv:2305.14251] provides the most widely adopted approach of this class: it decomposes text into minimal verifiable units (atomic facts), verifies each fact against a knowledge source, and reports factual precision as the fraction of supported atomic facts."*

   **Section 6.4 (line 227):** Listed without description — *"Decomposition-based: FActScore (2305.14251)."*

   **Section 8.1 (line 279):** *"The FActScore decomposition method [arXiv:2305.14251] (discussed in Section 5.4) can be applied to survey text, but verifying hundreds of atomic facts per survey remains labor-intensive without automation."*

   **Why redundant:** Low severity. Section 8.1 correctly includes "(discussed in Section 5.4)" and adds a new analytical point (labor-intensity). However, the arXiv ID is duplicated alongside the cross-reference, which is slightly redundant.

   **Suggested fix:** Remove the arXiv ID from the Section 8.1 mention: change *"The FActScore decomposition method [arXiv:2305.14251] (discussed in Section 5.4)"* to *"The FActScore decomposition method (Section 5.4)"*.

3. **SAFE (arXiv:2403.18802) — Described in Section 5.4, mentioned in Section 8.1**

   **Section 5.4 (line 172):** *"SAFE (Search-Augmented Factuality Evaluation) [arXiv:2403.18802] uses a search engine to verify factual claims against web sources, making it applicable to survey text where the LLM's parametric knowledge may be incomplete."*

   **Section 8.1 (line 280):** *"SAFE's search-augmented evaluation [arXiv:2403.18802] offers a more scalable alternative, though its reliance on web search may not capture the full evidence base available in restricted-access scientific databases."*

   **Why redundant:** Low severity. Different analytical points (capability vs. limitation). But Section 8.1 includes the arXiv ID without a cross-reference to Section 5.4.

   **Suggested fix:** Add cross-reference: *"SAFE's search-augmented evaluation (Section 5.4) offers a more scalable alternative, though its reliance on web search..."*

4. **SelfCheckGPT (arXiv:2303.08896) — Described in Section 6.4, mentioned in Section 8.1**

   **Section 6.4 (line 221):** *"SelfCheckGPT [arXiv:2303.08896] generates multiple samples from the LLM and checks for consistency between them — if a claim appears in only one of several samples, it is flagged as a potential hallucination."*

   **Section 8.1 (line 280):** *"SelfCheckGPT's sampling-based detection [arXiv:2303.08896] requires no external databases but has not been validated on scientific survey text specifically."*

   **Why redundant:** Low severity. Different analytical points. But arXiv ID included without cross-reference.

   **Suggested fix:** Add cross-reference: *"SelfCheckGPT's sampling-based detection (Section 6.4) requires no external databases but has not been validated on scientific survey text specifically."*

5. **InteractiveSurvey — Re-described despite cross-reference in Section 8.6**

   **Section 2.3 (line 47):** Canonical description — *"enables users to customize intermediate components — the outline, the paper pool, and section-level drafting parameters — through an iterative interaction loop."*

   **Section 7.1 (line 247):** *"InteractiveSurvey (described in Section 2.3) allows users to customize the outline, the paper pool, and section-level drafting parameters through a GUI-based interaction loop, with the system regenerating affected sections based on user input."* — cross-reference present, but re-lists all components.

   **Section 8.6 (line 311):** *"InteractiveSurvey (described in Section 2.3) offers a GUI-based approach where users can intervene at specific stages (outline, paper pool, section parameters) while the system handles the rest autonomously, providing more flexible control."*

   **Why redundant:** Low severity. Cross-references are present, but each re-describes aspects already established.

   **Suggested fix:** Tighten Section 8.6 to: *"InteractiveSurvey (Section 2.3) allows users to intervene at specific pipeline stages while the system handles the rest autonomously, providing more flexible control."*

6. **Within-paragraph repetition in Section 3.4**

   **Location (lines 100-103):** *"The finding has important implications for system design"* appears twice within the same paragraph — first after introducing the r>0.7 correlation, then again after describing the human-rating protocol.

   **Why redundant:** Same evaluative claim stated twice within one paragraph.

   **Suggested fix:** Remove the second occurrence of *"The finding has important implications for system design"* so the sentence flows directly from the protocol description to the next analytical point.

---

### Evidence

**Good cross-referencing practices observed:**
| Location | Target | Quality |
|----------|--------|---------|
| Section 2.4 → Section 2.3 (STORM) | "STORM [arXiv:2402.14207] (discussed in Section 2.3)" | ✅ Clean |
| Section 3.3 → Section 5.2 (Generate-then-Refine) | Explicit comparison of refinement granularities | ✅ Excellent — adds analytical value |
| Section 3.4 → Section 6.1 (SurveyBench) | "See Section 6.1 for a full description" | ✅ Clean |
| Section 3.4 → Section 8.2 (evaluation standardization) | "We revisit...in Section 8.2" | ✅ Good |
| Section 7.1 → Section 2.3 (InteractiveSurvey) | Cross-reference only, no arXiv ID | ✅ Good |
| Section 7.3 → Section 2.2 (LiRA) | "discussed in Section 2.2" | ✅ Clean |
| Section 8.1 → Section 5.4 (FActScore) | "(discussed in Section 5.4)" | ✅ Present |
| Section 8.6 → Section 2.3 (InteractiveSurvey) | "(described in Section 2.3)" | ✅ Present |

**Systems appearing in multiple sections (assessed for legitimate multi-faceted coverage):**

| System | Sections | Assessment |
|--------|----------|------------|
| AutoSurvey | 2.1 (foundation), 3.1 (outline), 9 (recommendation) | ✅ Different aspects |
| SurveyForge | 2.1 (agent), 3.1 (outline heuristics), 4.1 (scholar nav) | ✅ Different aspects |
| SurveyX | 2.1 (agent), 3.1 (AttributeTree), 4.1 (attr-aware queries) | ✅ Different aspects |
| SurveyGen-I | 2.1 (agent), 3.2 (evolving plans), 4.1 (coarse-to-fine) | ✅ Different aspects |
| LitLLM toolkit | 2.1 (agent), 4.1 (query formulation), 4.2 (re-ranking) | ✅ Different aspects |
| SurveyG | 2.2 (multi-agent), 5.3 (citation graph) | ✅ Different aspects |
| LitFM | 4.2 (graph retriever), 5.3 (citation graph) | ✅ Different aspects |
| ResearchPilot | 2.2 (multi-agent), 7.4 (synthesis), 8.4 (cost) | ✅ Different aspects |
| SurveySum | 3.1 (planning strategies), 6.2 (dataset) | ✅ Different aspects |

**Complementary rather than overlapping — Section 5.4 vs. Section 6.4:**
- **Section 5.4** covers factuality evaluation *methods* (FActScore, SAFE, VERISCORE, WildHallucinations, FINETUNE-RAG)
- **Section 6.4** covers hallucination detection *benchmarks and detectors* (HaluEval, HALoGEN, SelfCheckGPT, TRUE, Provenance, DAHL)
- **Zero overlap** in cited papers between the two subsections — good separation of concerns.

**LitLLMs study vs. LitLLM toolkit clearly distinguished:**
- Section 6.3 explicitly notes: *"The LitLLMs evaluation study [arXiv:2412.15249] — distinct from the LitLLM toolkit discussed in Sections 2.1, 4.1, and 4.2, which is a RAG-based pipeline system"* — good practice.

**Section 9 (Conclusion) avoids re-description:**
- Architectural recommendations are brief prescriptions without re-explaining systems
- Evaluation checklist is forward-looking (what to evaluate, not how each metric works)
- Research agenda items map to earlier sections without re-summarizing their content

---

### Suggestions

| # | Severity | Location | Issue | Suggested Fix |
|---|----------|----------|-------|---------------|
| S1 | 🟡 Moderate | §7.4 → §4.4 | OpenScholar: both sections describe purpose and 45M-paper datastore | Replace with cross-reference: *"(described in Section 4.4)"* + keep new interactive-reasoning content only |
| S2 | 🟢 Minor | §8.1 → §5.4 | FActScore: arXiv ID duplicated alongside cross-reference | Remove [arXiv:2305.14251]; "(Section 5.4)" is sufficient |
| S3 | 🟢 Minor | §8.1 → §5.4 | SAFE: no cross-reference to Section 5.4 | Add "(Section 5.4)" after first mention |
| S4 | 🟢 Minor | §8.1 → §6.4 | SelfCheckGPT: no cross-reference to Section 6.4 | Add "(Section 6.4)" after first mention |
| S5 | 🟢 Minor | §8.6 → §2.3 | InteractiveSurvey: re-lists components despite cross-reference | Trim to: *"InteractiveSurvey (Section 2.3) allows users to intervene at specific pipeline stages while the system handles the rest autonomously."* |
| S6 | 🟢 Minor | §3.4 | "The finding has important implications for system design" repeated in same paragraph | Delete the second occurrence |

---

### Weighted Contribution

Score 4 × 15% = **0.60**
