#!/usr/bin/env python3
"""Import MoEH related papers into .research/literature/ (manual workaround).

research_wiki ingest_paper's metadata chain (DBLP -> Semantic Scholar -> OpenAlex
fallback) produced garbage records for several arXiv IDs (it matched unrelated
papers, e.g. RADseq genomics for HARBOR) and ignored explicitly-passed metadata.
This script fetches authoritative metadata from the arXiv API for 20 papers and
writes yaml + md + bib entries in the tool's expected format.
"""

import os
import re
import urllib.request
import xml.etree.ElementTree as ET
from datetime import datetime, timezone

ROOT = "/Users/yzxoi/RCM/.research/literature"
PAPERS_DIR = os.path.join(ROOT, "papers")
BIB_PATH = os.path.join(ROOT, "references.bib")
API_URL = "https://export.arxiv.org/api/query?id_list={}&max_results=25"
NS = {"atom": "http://www.w3.org/2005/Atom"}

# (arxiv_id, table_title, tags, relevance, thesis)
PAPERS = [
    (
        "2604.20938",
        "HARBOR: Automated Harness Optimization",
        ["harness-automation", "closest-work"],
        "core",
        "Harness 设计是一等机器学习问题;自动配置搜索在 flag 空间足够大时优于手工堆叠。",
    ),
    (
        "2602.11574",
        "Learning to Configure Agentic AI Systems",
        ["harness-automation", "closest-work"],
        "core",
        "ARC 用轻量层级策略 + RL 按 query 动态定制 agent 配置(workflows/tools/token budgets/prompts)。",
    ),
    (
        "2605.09998",
        "Continual Harness: Online Adaptation for Self-Improving Foundation Agents",
        ["harness-automation", "evolving", "closest-work"],
        "core",
        "Reset-free 自改进 harness:交替改进自身 prompt、子 agent、技能、记忆,完全去人类。",
    ),
    (
        "2512.09108",
        "Evolving Excellence: Automated Optimization of LLM-based Agents",
        ["harness-automation", "closest-work"],
        "core",
        "无代码进化优化平台,语义感知遗传算子联合优化 agent 配置(prompt、工具描述、参数)。",
    ),
    (
        "2512.24615",
        "Youtu-Agent: Scaling Agent Productivity with Automated Generation and Hybrid Policy Optimization",
        ["harness-automation", "closest-work"],
        "core",
        "自动化 agent 生成与持续进化:工具代码生成、prompt 生成、配置合成 + 端到端 RL 训练。",
    ),
    (
        "2605.22166",
        "Adapting the Interface, Not the Model: Runtime Harness Adaptation for Deterministic LLM Agents",
        ["harness-automation", "runtime-adaptation", "closest-work"],
        "core",
        "生命周期感知运行时 harness,不改权重改进冻结 LLM agent,把重复交互失败转成可复用干预。",
    ),
    (
        "2602.03786",
        "AOrchestra: Automating Sub-Agent Creation for Agentic Orchestration",
        ["harness-automation", "closest-work"],
        "core",
        "中央编排器逐步具体化 Instruction/Context/Tools/Model,按需自动创建子 agent,Terminal-Bench +16.28%。",
    ),
    (
        "2510.11967",
        "Scaling Long-Horizon LLM Agent via Context-Folding",
        ["context-compression", "baseline"],
        "core",
        "FoldGRPO:程序化上下文折叠,10x 更小 active context 保持性能(Deep Research/SWE)。",
    ),
    (
        "2506.15841",
        "MEM1: Learning to Synergize Memory and Reasoning for Efficient Long-Horizon Agents",
        ["context-compression", "baseline"],
        "core",
        "端到端 RL 恒定记忆;MEM1-7B 性能 3.5x 提升、记忆 3.7x 减少 vs Qwen2.5-14B-Instruct。",
    ),
    (
        "2604.15877",
        "Experience Compression Spectrum: Unifying Memory, Skills, and Rules in LLM Agents",
        ["context-compression", "closest-work"],
        "core",
        "统一 memory/skills/rules 为压缩轴(5-20x / 50-500x / 1000x+),全是非参数化。",
    ),
    (
        "2510.02453",
        "How to Train Your Advisor: Steering Black-Box LLMs with Advisor Models",
        ["advisor-models", "baseline"],
        "core",
        "RL 训练的小模型生成动态 NL 建议提升黑盒模型(GPT-5.2 RuleArena +27.4%, Gemini 3 Pro SWE 步数 -24.6%)。",
    ),
    (
        "2604.09741",
        "ExecTune: Effective Steering of Black-Box LLMs with Guide Models",
        ["advisor-models", "closest-work"],
        "core",
        "teacher-guided acceptance sampling + SFT + structure-aware RL 优化句法有效性/执行成功率/成本(+9.2% 精度, -22.4% 推理成本)。",
    ),
    (
        "2605.11436",
        "Agent-BRACE: Decoupling Beliefs from Actions in Long-Horizon Tasks via Verbalized State Uncertainty",
        ["advisor-models", "closest-work"],
        "core",
        "把 agent 解耦为 belief state 模型 + policy 模型,belief 输出带确定性标签的原子 NL 声明(+14.5% Qwen2.5-3B)。",
    ),
    (
        "2602.17038",
        "Phase-Aware Mixture of Experts for Agentic Reinforcement Learning",
        ["moe-agents", "closest-work"],
        "core",
        "轻量 phase router 从 RL 目标学习潜阶段边界,时间一致分配专家;phase-level vs MoEH step-level。",
    ),
    (
        "2603.24984",
        "MoE-GRPO: Optimizing Mixture-of-Experts via Reinforcement Learning in Vision-Language Models",
        ["moe-agents"],
        "related",
        "GRPO 优化 MoE VLM 路由,专家选择为序列决策,modality-aware 路由。",
    ),
    (
        "2510.23027",
        "Towards Stable and Effective Reinforcement Learning for Mixture-of-Experts",
        ["moe-agents"],
        "related",
        "router-aware 重要性采样权重优化 off-policy RL 中 MoE,降低梯度方差、缓解发散。",
    ),
    (
        "2604.14419",
        "Equifinality in Mixture of Experts: Routing Topology Does Not Determine Language Modeling Quality",
        ["moe-agents"],
        "related",
        "路由拓扑不决定 MoE 渐近困惑度,五种 cosine routing 统计等价——对专家特化主张的威胁。",
    ),
    (
        "2504.08703",
        "TerminalBench: Benchmarking Autonomous Agents on Real-World Coding Tasks",
        ["benchmark"],
        "core",
        "2110 实例、21 仓库、多语言仓库级编码 agent 评测(bug 修复/功能添加/重构)。",
    ),
    (
        "2310.06770",
        "SWE-bench: Can Language Models Resolve Real-World GitHub Issues?",
        ["benchmark"],
        "core",
        "真实 GitHub issue 软件工程评测,要求模型编辑代码库解决问题。",
    ),
    (
        "2407.18901",
        "AppWorld: A Controllable World of Apps and People for Benchmarking Interactive Coding Agents",
        ["benchmark"],
        "related",
        "750 任务、9 应用、457 API 的交互式编码 agent 评测。",
    ),
]


def slugify_title(title: str) -> str:
    words = []
    for w in title.split():
        c = re.sub(r"[^A-Za-z0-9]", "", w)
        if c:
            words.append(c.lower())
    return "_".join(words)


def fetch_metadata(ids):
    url = API_URL.format(",".join(ids))
    req = urllib.request.Request(url, headers={"User-Agent": "rcm-moeh-import/0.1"})
    with urllib.request.urlopen(req, timeout=60) as resp:
        data = resp.read()
    root = ET.fromstring(data)
    out = {}
    for entry in root.findall("atom:entry", NS):
        eid_el = entry.find("atom:id", NS)
        title_el = entry.find("atom:title", NS)
        published_el = entry.find("atom:published", NS)
        if eid_el is None or title_el is None or published_el is None:
            continue
        eid = eid_el.text or ""
        arxiv_id = eid.split("/abs/")[-1].split("v")[0]
        title = re.sub(r"\s+", " ", (title_el.text or "").strip())
        authors = []
        for a in entry.findall("atom:author", NS):
            name_el = a.find("atom:name", NS)
            if name_el is not None and name_el.text:
                authors.append(name_el.text.strip())
        published = (published_el.text or "")[:4]
        out[arxiv_id] = {
            "title": title,
            "authors": authors,
            "year": int(published) if published else 0,
        }
    return out


def main():
    ids = [p[0] for p in PAPERS]
    meta = fetch_metadata(ids)
    now = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    bib_lines = []
    missing = []
    for arxiv_id, _table_title, tags, relevance, thesis in PAPERS:
        m = meta.get(arxiv_id)
        if not m:
            missing.append(arxiv_id)
            continue
        title, authors, year = m["title"], m["authors"], m["year"]
        parts = authors[0].split()
        first_lower = parts[0].lower() if parts else "unknown"
        last_lower = parts[-1].lower() if parts else "unknown"
        slug = f"{first_lower}{last_lower}_{year}_{slugify_title(title)}"
        cite_key = slug.replace("_", "")
        doi = f"10.48550/arxiv.{arxiv_id}"

        yaml_content = f"""slug: {slug}
title: "{title}"
authors:
"""
        for a in authors:
            yaml_content += f"  - {a}\n"
        yaml_content += f"""year: {year}
venue: arXiv (Cornell University)
arxiv: "{arxiv_id}"
doi: {doi}
tags:
"""
        for t in tags:
            yaml_content += f"  - {t}\n"
        yaml_content += f"""relevance: {relevance}
cite_key: {cite_key}
created: {now}
updated: {now}
"""
        md_content = f"""# {title}

## One-line Thesis

{thesis}

## Problem / Gap



## Method



## Key Results



## Assumptions



## Limitations / Failure Modes



## Reusable Ingredients

(techniques, datasets, or insights that could be repurposed)

## Open Questions



## Connections

(auto-generated from edges.jsonl — do not edit manually)

## Relevance to This Project

"""
        with open(os.path.join(PAPERS_DIR, f"{slug}.yaml"), "w") as f:
            f.write(yaml_content)
        with open(os.path.join(PAPERS_DIR, f"{slug}.md"), "w") as f:
            f.write(md_content)
        bib_lines.append(f"""@article{{{cite_key},
  title={{{title}}},
  author={{{" and ".join(authors)}}},
  year={{{year}}},
  journal={{arXiv (Cornell University)}},
  doi={{{doi}}},
}}""")
        print(f"OK {arxiv_id} -> {slug}")

    with open(BIB_PATH, "a") as f:
        f.write("\n" + "\n".join(bib_lines) + "\n")
    print(f"appended {len(bib_lines)} bib entries")
    if missing:
        print(f"MISSING (no arXiv metadata): {missing}")


if __name__ == "__main__":
    main()
