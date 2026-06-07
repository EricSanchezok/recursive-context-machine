# Candidate Pool — Coverage Auditor Additions

**Generated:** 2026-06-07T18:53Z  
**Source:** coverage_auditor (canonical paper verification)  
**Note:** This file contains canonical papers added via arxiv_search after verifying the existing pools (02b_candidate_pool_extended.md, 03_expansion.md). None of the canonical papers were already present.

---

## Added Canonical Papers (18 entries)

### Foundation Code Models

- **arXiv:2107.03374** — Evaluating Large Language Models Trained on Code — Mark Chen, Jerry Tworek, Heewoo Jun et al. — 2021 — Introduces Codex, GPT language model fine-tuned on GitHub code. On HumanEval, solves 28.8% of problems (0% for GPT-3). Powering GitHub Copilot. — Source: coverage_auditor [canonical]

- **arXiv:2204.05999** — InCoder: A Generative Model for Code Infilling and Synthesis — Daniel Fried, Armen Aghajanyan, Jessy Lin et al. — 2022 — Unified generative model for program synthesis and editing via infilling. First generative model capable of zero-shot code infilling for type inference, comment generation, variable re-naming. — Source: coverage_auditor [canonical] *(Canonical list ID was 2201.11903; actual paper found at 2204.05999)*

- **arXiv:2109.00859** — CodeT5: Identifier-aware Unified Pre-trained Encoder-Decoder Models for Code Understanding and Generation — Yue Wang, Weishi Wang, Shafiq Joty et al. — 2021 — Unified pre-trained encoder-decoder Transformer for code. Novel identifier-aware pre-training task. Outperforms prior methods on defect detection, clone detection, and generation. — Source: coverage_auditor [canonical]

- **arXiv:2305.07922** — CodeT5+: Open Code Large Language Models for Code Understanding and Generation — Yue Wang, Hung Le, Akhilesh Deepak Gotmare et al. — 2023 — Family of encoder-decoder LLMs with flexible module combinations. Mixture of pretraining objectives (span denoising, contrastive learning, text-code matching, causal LM). SoTA on code generation and completion. — Source: coverage_auditor [canonical]

- **arXiv:2103.06333** — Unified Pre-training for Program Understanding and Generation (PLBART) — Wasi Uddin Ahmad, Saikat Chakraborty, Baishakhi Ray et al. — 2021 — Introduces PLBART, a sequence-to-sequence model pre-trained on Java/Python functions via denoising autoencoding. Outperforms on code summarization, generation, translation, and discriminative tasks. — Source: coverage_auditor [canonical] *(Canonical list ID was 2103.06144; actual paper found at 2103.06333)*

- **arXiv:2305.06161** — StarCoder: may the source be with you! — Raymond Li, Loubna Ben Allal, Yangtian Zi et al. — 2023 — 15.5B parameter Code LLM with 8K context, infilling capabilities. Trained on 1T tokens from The Stack. Achieves 40% pass@1 on HumanEval. Outperforms every open Code LLM. — Source: coverage_auditor [canonical] *(Canonical list ID was 2308.04675; actual paper found at 2305.06161)*

- **arXiv:2308.12950** — Code Llama: Open Foundation Models for Code — Baptiste Rozière, Jonas Gehring, Fabian Gloeckle et al. — 2023 — Family of LLMs for code based on Llama 2. 7B-70B parameters. State-of-the-art among open models on HumanEval (67%) and MBPP (65%). Supports infilling and large input contexts. — Source: coverage_auditor [canonical]

- **arXiv:2401.14196** — DeepSeek-Coder: When the Large Language Model Meets Programming -- The Rise of Code Intelligence — Daya Guo, Qihao Zhu, Dejian Yang et al. — 2024 — Range of open-source code models (1.3B-33B) trained on 2T tokens. Fill-in-the-blank task with 16K window. Surpasses Codex and GPT-3.5 on multiple benchmarks. — Source: coverage_auditor [canonical]

- **arXiv:2303.08774** — GPT-4 Technical Report — OpenAI, Josh Achiam, Steven Adler et al. — 2023 — Reports development of GPT-4, a large-scale multimodal model. Exhibits human-level performance on professional and academic benchmarks, including passing simulated bar exam at top 10%. — Source: coverage_auditor [canonical]

- **arXiv:2401.04088** — Mixtral of Experts — Albert Q. Jiang, Alexandre Sablayrolles, Antoine Roux et al. — 2024 — Introduces Mixtral 8x7B, a Sparse Mixture of Experts (SMoE) language model. 47B parameters total but only 13B active per token. Outperforms Llama 2 70B and GPT-3.5 across all benchmarks. — Source: coverage_auditor [canonical] *(Canonical list ID was 2310.06825; actual paper found at 2401.04088)*

### Editing & APRs

- **arXiv:2208.05446** — CoditT5: Pretraining for Source Code and Natural Language Editing — Jiyang Zhang, Sheena Panthaplackel, Pengyu Nie et al. — 2022 — Pretraining objective explicitly modeling edits. Fine-tuned on comment updating, bug fixing, automated code review. Outperforms standard generation-based models on editing tasks. — Source: coverage_auditor [canonical] *(Canonical list ID was 2203.09054; actual paper found at 2208.05446)*

- **arXiv:2305.18584** — Coeditor: Leveraging Contextual Changes for Multi-round Code Auto-editing — Jiayi Wei, Greg Durrett, Isil Dillig — 2023 — Fine-tuned language model for multi-round code editing. Uses line diff format and static analysis. Outperforms GPT-3.5 (exact-match from 34.7 to 60.4). VSCode extension released. — Source: coverage_auditor [canonical] *(Canonical list ID was 2304.09650; actual paper found at 2305.18584)*

### Agentic & Benchmark

- **arXiv:2405.15793** — SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering — John Yang, Carlos E. Jimenez, Alexander Wettig et al. — 2024 — Custom agent-computer interface (ACI) for LM agents to autonomously use computers. SoTA on SWE-bench (12.5%) and HumanEvalFix (87.7%). — Source: coverage_auditor [canonical] *(Canonical list ID was 2310.06770; actual paper found at 2405.15793)*

- **arXiv:2404.05427** — AutoCodeRover: Autonomous Program Improvement — Yuntong Zhang, Haifeng Ruan, Zhiyu Fan et al. — 2024 — Automated approach for solving GitHub issues combining LLMs with code search. Spectrum-based fault localization. 19% on SWE-bench-lite at $0.43 per issue. — Source: coverage_auditor [canonical] *(Canonical list ID was 2401.12914; actual paper found at 2404.05427)*

- **arXiv:2407.01489** — Agentless: Demystifying LLM-based Software Engineering Agents — Chunqiu Steven Xia, Yinlin Deng, Soren Dunn et al. — 2024 — Simplistic three-phase approach (localization, repair, validation) without complex agent tools. Achieves 32.00% on SWE-bench Lite at $0.70 per issue. — Source: coverage_auditor [canonical] *(Canonical list ID was 2402.09746; actual paper found at 2407.01489)*

- **arXiv:2407.16741** — OpenHands: An Open Platform for AI Software Developers as Generalist Agents — Xingyao Wang, Boxuan Li, Yufan Song et al. — 2024 — Platform for AI agents that write code, interact with command line, and browse web. MIT licensed. Community project with 2.1K+ contributions from 188+ contributors. — Source: coverage_auditor [canonical] *(Canonical list ID was 2402.15780; actual paper found at 2407.16741)*

- **arXiv:2310.06770** — SWE-bench: Can Language Models Resolve Real-World GitHub Issues? — Carlos E. Jimenez, John Yang, Alexander Wettig et al. — 2023 — Evaluation framework with 2,294 real GitHub issues across 12 Python repositories. Best model (Claude 2) solves only 1.96%. Requires understanding across multiple files. — Source: coverage_auditor [canonical] *(Canonical list ID was 2312.08693; actual paper found at 2310.06770)*

- **arXiv:2605.29522** — DeepSurvey: Enhancing Analytical Depth and Citation Reliability in Automated Survey Generation — Ziyue Yang, Da Ma, Hanqi Li et al. — 2026 — Agentic system combining citation-graph expansion with multi-granularity agentic refinement; achieves 8.644/10 content score; domain experts prefer over human-written surveys (83.3%). — Source: supervisor_retrieval_R2

- **arXiv:2512.20854** — How important is Recall for Measuring Retrieval Quality? — Shelly Schwartz, Oleg Vasilyev, Randy Sawaya — 2025 — Investigates correlation between retrieval quality metrics and LLM-judged response quality; proposes recall-free retrieval quality metric. — Source: supervisor_retrieval_R2

### Survey & Meta

- **arXiv:2007.12626** — SummEval: Re-evaluating Summarization Evaluation — Alexander R. Fabbri, Wojciech Kryściński, Bryan McCann et al. — 2020 — Re-evaluates 14 automatic evaluation metrics. Benchmarks 23 summarization models. Largest collection of human judgments on CNN/DailyMail summaries. — Source: coverage_auditor [canonical] *(Canonical list ID was 2003.00744; actual paper found at 2007.12626)*

---

## Summary

| Metric | Value |
|--------|-------|
| Canonical papers checked | 23 (unique) |
| Already in pool | 0 |
| Newly added | 18 |
| Still missing (no arXiv paper found) | 5 |
