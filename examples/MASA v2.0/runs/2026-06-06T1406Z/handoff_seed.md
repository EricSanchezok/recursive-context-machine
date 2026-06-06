# Handoff: CitationSeedSelector → CitationExpander

**run_dir**: `.`
**artifact**: `03a_seed_papers.md`
**status**: ok

**Seed count**: 8

**Seed arXiv IDs**:
- 2406.10252v2 (AutoSurvey)
- 2312.07559v2 (PaperQA)
- 2411.14199v1 (OpenScholar)
- 2510.07733 (SurveyG)
- 2503.04629v1 (SurveyForge)
- 2509.18661v1 (Agentic AutoSurvey)
- 2510.03120 (SurveyBench)
- 2409.12177v1 (LitFM)

**Coverage rationale**:
- **Timeline**: 2023 (PaperQA) → 2024 (AutoSurvey, OpenScholar, LitFM) → 2025 (SurveyG, SurveyForge, Agentic AutoSurvey, SurveyBench)
- **Sub-areas**: survey generation pipelines (4 seeds), retrieval/infrastructure (3 seeds), evaluation (1 seed)
- **Citation graph centricity**: SurveyG (hierarchical citation graphs), LitFM (graph retriever), AutoSurvey (most-cited hub)
- **Benchmark forward expansion**: SurveyBench captures all systems evaluated against it

**Risks**:
1. **run_dir recovered from cwd** (`"."`) — no upstream run_dir was passed; all artifacts read from and written to the current working directory.
2. **All seeds are 2023–2025** — no pre-2020 seeds included. If the CitationExpander needs older foundational papers (e.g., cocitation theory, early citation recommendation), it should rely on backward expansion from AutoSurvey and PaperQA to reach pre-2020 literature.
3. **Seeds are all arXiv-available** — no downloads needed; all 8 IDs are arXiv papers with accessible PDFs.
