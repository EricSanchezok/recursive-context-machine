"""
DeepResearch-Bench Simple Mode — 单篇研究报告 RACE + FACT 联合评测

实现官方 DeepResearch Bench 的两大评估框架:

  1. RACE (Reference-based Adaptive Criteria-driven Evaluation)
     动态生成评估准则 + 参考对比评分，4维度: Comprehensiveness, Insight,
     Instruction-Following, Readability. 最终输出归一化相对分数。

  2. FACT (Framework for Factual Abundance and Citation Trustworthiness)
     自动提取陈述-URL对 → 去重 → 抓取源内容 → 验证支持性 →
     计算 Citation Accuracy 和 Effective Citations.

用法:
  python evaluate_simple.py --pdf report.pdf
  python evaluate_simple.py --markdown report.md --reference ref.md
  python evaluate_simple.py --race-only --model gpt-4o
  python evaluate_simple.py --fact-only --jina-api-key sk-xxx
"""

import argparse
import json
import os
import re
import sys
import time
import threading
import concurrent.futures
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import requests
from config import (
    BENCH_DIR,
    API_KEY, API_BASE_URL,
    RACE_API_KEY, RACE_API_BASE, RACE_MODEL,
    FACT_API_KEY, FACT_API_BASE, FACT_MODEL,
    JINA_API_KEY, TEMPERATURE, MAX_TOKENS, MAX_WORKERS,
    DEFAULT_DIM_WEIGHTS,
)

# ═══════════════════════════════════════════════════════════════
#  LLM Client  (OpenAI-compatible)
# ═══════════════════════════════════════════════════════════════

class LLMClient:
    """Lightweight OpenAI-compatible LLM client with retry logic.

    Supports any provider with an OpenAI-compatible API:
    OpenAI, DeepSeek, OpenRouter, Groq, Together AI, etc.
    """

    def __init__(self, model: str = "", api_key: str = "", base_url: str = ""):
        self.model = model
        self.api_key = api_key
        self.base_url = base_url.rstrip("/") if base_url else ""

    def generate(self, prompt: str, system_prompt: str = "",
                 temperature: float = None, max_tokens: int = None) -> str:
        """Call the LLM with retries."""
        if not self.api_key:
            raise ValueError(
                "No API key configured. Set one of:\n"
                "  - API_KEY environment variable\n"
                "  - RACE_API_KEY / FACT_API_KEY for per-judge overrides\n"
                "  - --api-key / --race-api-key / --fact-api-key CLI arguments"
            )
        if not self.model:
            raise ValueError(
                "No model configured. Set one of:\n"
                "  - RACE_MODEL / FACT_MODEL environment variable\n"
                "  - --model / --race-model / --fact-model CLI arguments"
            )

        payload = {
            "model": self.model,
            "messages": [],
            "temperature": TEMPERATURE if temperature is None else temperature,
            "max_tokens": MAX_TOKENS if max_tokens is None else max_tokens,
        }
        if system_prompt:
            payload["messages"].append({"role": "system", "content": system_prompt})
        payload["messages"].append({"role": "user", "content": prompt})

        last_error = None
        for attempt in range(5):
            try:
                resp = requests.post(
                    f"{self.base_url}/chat/completions",
                    json=payload,
                    headers={
                        "Authorization": f"Bearer {self.api_key}",
                        "Content-Type": "application/json",
                    },
                    timeout=300,
                )
                resp.raise_for_status()
                data = resp.json()
                content = data["choices"][0]["message"]["content"]
                return content or ""
            except Exception as e:
                last_error = e
                if attempt < 4:
                    time.sleep(1.5 ** attempt)
        raise RuntimeError(f"LLM call failed after 5 retries: {last_error}")


# ═══════════════════════════════════════════════════════════════
#  RACE Evaluation Framework
# ═══════════════════════════════════════════════════════════════

# ── Prompts ───────────────────────────────────────────────────

RACE_DIMENSION_WEIGHT_PROMPT_EN = """You are an expert evaluator of deep research reports. Given a research task, assign importance weights (summing to 1.0) to four evaluation dimensions.

Task: {task_prompt}

Dimensions to weight:
- comprehensiveness: Coverage of key areas, no important omissions
- insight: Depth of analysis, valuable insights and critical thinking
- instruction_following: Relevance to the task, directly addresses the query
- readability: Clear structure, fluent language, easy to follow

Return ONLY a JSON object with dimension names as keys and weights as values, e.g.:
{{"comprehensiveness": 0.30, "insight": 0.30, "instruction_following": 0.25, "readability": 0.15}}
Weights must sum to 1.0."""

RACE_DIMENSION_WEIGHT_PROMPT_ZH = """你是一名深度研究报告的专家评估员。给定一个研究任务，请为以下四个评估维度分配重要性权重（总和为1.0）。

任务：{task_prompt}

需要权重的维度：
- comprehensiveness：全面性——覆盖关键领域，无重要遗漏
- insight：深度洞察——分析深入，提供有价值的见解和批判性思考
- instruction_following：指令遵循——紧扣研究主题，直接回答任务问题
- readability：可读性——结构清晰，语言流畅，易于理解

仅返回一个 JSON 对象，以维度名称为键、权重为值，例如：
{{"comprehensiveness": 0.30, "insight": 0.30, "instruction_following": 0.25, "readability": 0.15}}
权重必须总和为1.0。"""

RACE_CRITERIA_PROMPT_EN_TMPL = """You are an expert evaluator of deep research reports. Given the research task below, generate {num_criteria} specific, detailed evaluation criteria for the "{dimension}" dimension.

Task: {task_prompt}

Each criterion should have:
- criterion: A concise name (e.g., "Coverage of recent developments")
- explanation: A brief explanation of what to look for
- weight: Importance weight for this criterion within this dimension (must sum to 1.0 across all criteria in this dimension)

Return a JSON list like:
[
  {{"criterion": "Criterion name", "explanation": "What to check", "weight": 0.5}},
  {{"criterion": "Criterion name 2", "explanation": "What to check", "weight": 0.5}}
]"""

RACE_CRITERIA_PROMPT_ZH_TMPL = """你是一名深度研究报告的专家评估员。给定以下研究任务，为"{dimension}"维度生成{num_criteria}条具体、详细的评估准则。

任务：{task_prompt}

每条准则应包含：
- criterion：简洁的名称
- explanation：解释需要检查的内容
- weight：该准则在此维度内的重要性权重（同维度下所有准则的权重总和必须为1.0）

返回 JSON 列表格式：
[
  {{"criterion": "准则名称", "explanation": "检查内容说明", "weight": 0.5}},
  {{"criterion": "准则名称2", "explanation": "检查内容说明", "weight": 0.5}}
]"""

RACE_SCORE_PROMPT_EN = """You are an expert evaluator of deep research reports. Compare the following two reports for the given research task.

TASK:
{task_prompt}

DIMENSION WEIGHTS:
{dimension_weights_json}

EVALUATION CRITERIA PER DIMENSION:
{criteria_json}

---
REPORT A (the target report to evaluate):
{report_a}

---
REPORT B (a high-quality reference report):
{report_b}

---
For EACH dimension, score BOTH reports on each criterion (1-10 scale, 10 = best).
Then return a JSON object with the following structure:

{{
  "comprehensiveness": [
    {{"criterion": "Criterion name", "article_1_score": 7, "article_2_score": 8}},
    ...
  ],
  "insight": [...],
  "instruction_following": [...],
  "readability": [...]
}}

IMPORTANT:
- article_1_score refers to REPORT A (the target report)
- article_2_score refers to REPORT B (the reference report)
- Be objective and discriminating — use the full 1-10 range."""

RACE_SCORE_PROMPT_ZH = """你是一名深度研究报告的专家评估员。请对以下两份报告针对给定研究任务进行对比评分。

研究任务：
{task_prompt}

维度权重：
{dimension_weights_json}

各维度评估准则：
{criteria_json}

---
报告 A（待评估的目标报告）：
{report_a}

---
报告 B（高质量参考报告）：
{report_b}

---
请针对每个维度，对两份报告在每个准则上分别评分（1-10分，10分为最佳）。
返回以下结构的 JSON 对象：

{{
  "comprehensiveness": [
    {{"criterion": "准则名称", "article_1_score": 7, "article_2_score": 8}},
    ...
  ],
  "insight": [...],
  "instruction_following": [...],
  "readability": [...]
}}

重要说明：
- article_1_score 指报告 A（待评估的目标报告）
- article_2_score 指报告 B（参考报告）
- 请客观区分，充分利用 1-10 分的完整范围。"""

DIMENSION_NAMES = ["comprehensiveness", "insight", "instruction_following", "readability"]
DIMENSION_NAMES_ZH = {
    "comprehensiveness": "全面性",
    "insight": "深度洞察",
    "instruction_following": "指令遵循",
    "readability": "可读性",
}


def _is_chinese(text: str) -> bool:
    """Heuristic: if >10% of chars are in CJK range, treat as Chinese."""
    if not text:
        return False
    cjk = sum(1 for c in text if '\u4e00' <= c <= '\u9fff')
    return cjk / max(len(text), 1) > 0.10


def _extract_json(text: str):
    """Extract JSON object or list from LLM response text."""
    # Try direct parse
    text = text.strip()
    if text.startswith("```"):
        # Remove markdown fences
        text = re.sub(r"^```(?:json)?\s*", "", text)
        text = re.sub(r"\s*```$", "", text)
        text = text.strip()

    # Find first { or [
    for start_char, end_char in [("{", "}"), ("[", "]")]:
        start = text.find(start_char)
        if start == -1:
            continue
        depth = 0
        for i in range(start, len(text)):
            if text[i] == start_char:
                depth += 1
            elif text[i] == end_char:
                depth -= 1
                if depth == 0:
                    candidate = text[start:i + 1]
                    try:
                        return json.loads(candidate)
                    except json.JSONDecodeError:
                        continue
    return None


def _validate_weights(data, expected_sum=1.0, tol=1e-4):
    """Validate that weights sum to expected_sum."""
    if not isinstance(data, dict):
        return False
    total = sum(float(v) for v in data.values() if isinstance(v, (int, float)))
    return abs(total - expected_sum) < tol


def _round_weights(weights):
    """Round weights to 2 decimal places and adjust readability to sum to 1.0."""
    rounded = {k: round(float(v), 2) for k, v in weights.items()}
    total = sum(rounded.values())
    diff = round(1.0 - total, 2)
    if abs(diff) > 1e-6 and "readability" in rounded:
        rounded["readability"] = round(rounded["readability"] + diff, 2)
    return rounded


class RACEEvaluator:
    """Reference-based Adaptive Criteria-driven Evaluation with Dynamic Weighting."""

    def __init__(self, llm_client: LLMClient = None, language: str = "auto"):
        self.client = llm_client or LLMClient(model=RACE_JUDGE_MODEL)
        self.language = language

    def _detect_language(self, text: str) -> str:
        if self.language != "auto":
            return self.language
        return "zh" if _is_chinese(text) else "en"

    def _get_weight_prompt(self, lang: str) -> str:
        return RACE_DIMENSION_WEIGHT_PROMPT_ZH if lang == "zh" else RACE_DIMENSION_WEIGHT_PROMPT_EN

    def _get_criteria_prompt(self, lang: str) -> str:
        return RACE_CRITERIA_PROMPT_ZH_TMPL if lang == "zh" else RACE_CRITERIA_PROMPT_EN_TMPL

    def _get_score_prompt(self, lang: str) -> str:
        return RACE_SCORE_PROMPT_ZH if lang == "zh" else RACE_SCORE_PROMPT_EN

    def generate_dimension_weights(self, task_prompt: str, samples: int = 1) -> dict:
        """Generate dimension weights, optionally averaging multiple samples."""
        lang = self._detect_language(task_prompt)
        prompt_tmpl = self._get_weight_prompt(lang)
        prompt = prompt_tmpl.format(task_prompt=task_prompt)

        all_weights = []
        for _ in range(samples):
            for attempt in range(5):
                try:
                    resp = self.client.generate(prompt)
                    parsed = _extract_json(resp)
                    if parsed and _validate_weights(parsed):
                        all_weights.append(parsed)
                        break
                except Exception:
                    if attempt == 4:
                        raise
                    time.sleep(2)

        if not all_weights:
            return dict(DEFAULT_DIM_WEIGHTS)

        # Average weights
        dims = set()
        for w in all_weights:
            dims.update(w.keys())
        avg = {}
        for d in dims:
            vals = [w.get(d, 0) for w in all_weights]
            avg[d] = sum(vals) / len(vals)
        total = sum(avg.values())
        if total > 0:
            for d in avg:
                avg[d] /= total
        return _round_weights(avg)

    def generate_criteria(self, task_prompt: str, dimension: str,
                          num_criteria: int = 3) -> list:
        """Generate evaluation criteria for a specific dimension."""
        lang = self._detect_language(task_prompt)
        prompt_tmpl = self._get_criteria_prompt(lang)
        dim_label = DIMENSION_NAMES_ZH.get(dimension, dimension) if lang == "zh" else dimension
        prompt = prompt_tmpl.format(
            task_prompt=task_prompt,
            dimension=dim_label,
            num_criteria=num_criteria,
        )
        for attempt in range(5):
            try:
                resp = self.client.generate(prompt)
                parsed = _extract_json(resp)
                if isinstance(parsed, list) and len(parsed) > 0:
                    # Validate weights sum
                    total_w = sum(float(c.get("weight", 0)) for c in parsed)
                    if abs(total_w - 1.0) > 0.01:
                        # Normalize
                        for c in parsed:
                            c["weight"] = float(c.get("weight", 0)) / total_w
                    return parsed
            except Exception:
                if attempt == 4:
                    raise
                time.sleep(2)
        # Fallback: return generic criteria
        return [
            {"criterion": f"{dimension}_criterion_1", "explanation": "", "weight": 1.0 / num_criteria},
            {"criterion": f"{dimension}_criterion_2", "explanation": "", "weight": 1.0 / num_criteria},
            {"criterion": f"{dimension}_criterion_3", "explanation": "", "weight": 1.0 / num_criteria},
        ]

    def generate_all_criteria(self, task_prompt: str, num_criteria: int = 3) -> dict:
        """Generate criteria for all 4 dimensions."""
        criteria = {}
        # Generate in parallel
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
            fut_to_dim = {
                ex.submit(self.generate_criteria, task_prompt, d, num_criteria): d
                for d in DIMENSION_NAMES
            }
            for fut in concurrent.futures.as_completed(fut_to_dim):
                d = fut_to_dim[fut]
                criteria[d] = fut.result()
        return criteria

    def score(self, task_prompt: str, target_report: str, reference_report: str,
              dimension_weights: dict = None, criteria: dict = None) -> dict:
        """Score the target report against the reference using RACE."""
        lang = self._detect_language(task_prompt)

        # Auto-generate weights if not provided
        if dimension_weights is None:
            dimension_weights = self.generate_dimension_weights(task_prompt, samples=3)
        if criteria is None:
            criteria = self.generate_all_criteria(task_prompt)

        score_prompt_tmpl = self._get_score_prompt(lang)
        prompt = score_prompt_tmpl.format(
            task_prompt=task_prompt,
            dimension_weights_json=json.dumps(dimension_weights, ensure_ascii=False, indent=2),
            criteria_json=json.dumps(criteria, ensure_ascii=False, indent=2),
            report_a=target_report,
            report_b=reference_report,
        )

        resp = self.client.generate(prompt)
        scores = _extract_json(resp)
        if not scores or not isinstance(scores, dict):
            raise ValueError(f"Failed to parse RACE scores from LLM response: {resp[:200]}")

        # Calculate weighted scores
        return self._calculate_final_scores(scores, dimension_weights, criteria)

    def _calculate_final_scores(self, llm_scores: dict, dim_weights: dict,
                                 criteria: dict) -> dict:
        """Calculate normalized weighted scores from LLM output."""
        target_dims = {}
        reference_dims = {}
        target_total = 0.0
        reference_total = 0.0

        for dim in DIMENSION_NAMES:
            scores_list = llm_scores.get(dim, [])
            if not isinstance(scores_list, list):
                continue

            dim_weight = dim_weights.get(dim, 0.25)
            dim_criteria = criteria.get(dim, [])

            # Build weight lookup
            weight_map = {}
            for c in dim_criteria:
                weight_map[c.get("criterion", "")] = float(c.get("weight", 1.0 / max(len(dim_criteria), 1)))

            target_wsum = 0.0
            reference_wsum = 0.0
            total_w = 0.0

            for item in scores_list:
                if not isinstance(item, dict):
                    continue
                cname = item.get("criterion", "")
                s1 = item.get("article_1_score")
                s2 = item.get("article_2_score")
                if s1 is None:
                    s1 = item.get("target_score")
                try:
                    s1 = float(s1) if s1 is not None else None
                    s2 = float(s2) if s2 is not None else None
                except (ValueError, TypeError):
                    continue

                if cname and s1 is not None:
                    w = weight_map.get(cname, 1.0 / max(len(dim_criteria), 1))
                    target_wsum += s1 * w
                    if s2 is not None:
                        reference_wsum += s2 * w
                    total_w += w

            target_avg = target_wsum / total_w if total_w > 0 else 0
            reference_avg = reference_wsum / total_w if total_w > 0 else 0

            target_dims[dim] = target_avg
            reference_dims[dim] = reference_avg
            target_total += target_avg * dim_weight
            reference_total += reference_avg * dim_weight

        # Normalize: compute relative scores (target / (target + reference))
        overall = 0.0
        if target_total + reference_total > 0:
            overall = target_total / (target_total + reference_total)

        normalized_dims = {}
        for dim in DIMENSION_NAMES:
            t = target_dims.get(dim, 0)
            r = reference_dims.get(dim, 0)
            normalized_dims[dim] = t / (t + r) if t + r > 0 else 0

        return {
            "overall_score": round(overall, 4),
            "comprehensiveness": round(normalized_dims.get("comprehensiveness", 0), 4),
            "insight": round(normalized_dims.get("insight", 0), 4),
            "instruction_following": round(normalized_dims.get("instruction_following", 0), 4),
            "readability": round(normalized_dims.get("readability", 0), 4),
            "raw_target_scores": target_dims,
            "raw_reference_scores": reference_dims,
            "dimension_weights": dim_weights,
        }


# ═══════════════════════════════════════════════════════════════
#  FACT Evaluation Framework
# ═══════════════════════════════════════════════════════════════

FACT_EXTRACT_PROMPT_EN = """You will be provided with a research report. The body of the report contains citations to references.
Citations may appear as:
1. Text + space + number, e.g., "some text 15"
2. Text + [number], e.g., "some text[15]"
3. Text + [number†details], e.g., "some text[15†L10][5L23]"
4. [Title](URL), e.g., "[Title](https://example.com)"

Please extract ALL cited statement instances as (fact, url) pairs.

Rules:
- For each citation, extract enough context before/after to make the fact self-contained
- If a fact cites multiple sources, create a pair for each
- For format 4 (markdown link), use the URL directly
- For formats 1-3, try to find the URL from the reference list at the end
- If no URL can be found, set url to ""

Return a JSON list:
[
  {{"fact": "The complete factual statement...", "url": "https://..."}},
  {{"fact": "Another statement...", "url": "https://..."}}
]

Report text:
{report_text}"""

FACT_EXTRACT_PROMPT_ZH = """你将会看到一份研究报告，正文中包含对参考文献的引用。
引用形式可能为：
1. 文字 + 空格 + 数字，如："社会经济地位指数（SES）将社会划分为7个等级 15"
2. 文字 + [数字]，如："将社会划分为7个等级[15]"
3. 文字 + [数字†行号等]，如："将社会划分为7个等级[15†L10]"
4. [标题](链接)，如："[ChinaFile分类](https://example.com)"

请提取正文中所有引用了参考文献的陈述，提取 (fact, url) 对。

规则：
- 每条引用提取足够上下文，使陈述完整可理解
- 如果一条陈述引用多个来源，为每个来源创建一条
- 对于格式4（Markdown链接），直接使用URL
- 对于格式1-3，尝试从文末参考文献列表中提取URL
- 如果找不到URL，设为""

返回 JSON 列表：
[
  {{"fact": "完整的陈述文本...", "url": "https://..."}},
  {{"fact": "另一条陈述...", "url": "https://..."}}
]

报告正文：
{report_text}"""

FACT_DEDUP_PROMPT_EN = """You will be given a list of statements. De-duplicate them: two statements are duplicates ONLY if they express exactly the same thing.
Return a list of indices (1-based) of the unique statements to keep.

Statements:
{statements}

Return ONLY a JSON integer list, e.g.: [1, 3, 5]"""

FACT_DEDUP_PROMPT_ZH = """你将会看到一个陈述列表，请对其去重。只有表达完全一致的内容时，两个陈述才被认为是重复的。
返回去重后保留的陈述序号（从1开始）列表。

陈述列表：
{statements}

仅返回 JSON 整数列表，例如：[1, 3, 5]"""

FACT_VALIDATE_PROMPT_EN = """You will be provided with a reference and some statements. Determine whether each statement is "supported", "unsupported", or "unknown".

Rules:
- If the reference has no valid content (e.g., "page not found"), mark all as "unknown"
- If facts/data in the statement can be fully or partially found in the reference, it is "supported"
- If none of the facts/data can be found, it is "unsupported"

Return a JSON list:
[
  {{"idx": 1, "result": "supported"}},
  {{"idx": 2, "result": "unsupported"}}
]

<reference>
{reference}
</reference>
<statements>
{statements}
</statements>"""

FACT_VALIDATE_PROMPT_ZH = """你将会看到一个参考资料和一些陈述。请判断每条陈述对于参考资料来说是 "supported"（支持）、"unsupported"（不支持）还是 "unknown"（未知）。

规则：
- 如果参考资料无效（如"页面未找到"），所有陈述标记为 "unknown"
- 如果陈述中的事实/数据在参考资料中全部或部分能找到，则为 "supported"
- 如果完全找不到，则为 "unsupported"

返回 JSON 列表：
[
  {{"idx": 1, "result": "supported"}},
  {{"idx": 2, "result": "unsupported"}}
]

<reference>
{reference}
</reference>
<statements>
{statements}
</statements>"""


class FACTEvaluator:
    """Framework for Factual Abundance and Citation Trustworthiness."""

    def __init__(self, llm_client: LLMClient = None,
                 jina_api_key: str = None):
        self.client = llm_client or LLMClient(model=FACT_JUDGE_MODEL)
        self.jina_api_key = jina_api_key or JINA_API_KEY

    def extract_citations(self, report_text: str) -> list:
        """Extract (fact, url) pairs from report text using LLM."""
        lang = "zh" if _is_chinese(report_text) else "en"
        prompt_tmpl = FACT_EXTRACT_PROMPT_ZH if lang == "zh" else FACT_EXTRACT_PROMPT_EN
        prompt = prompt_tmpl.format(report_text=report_text)

        for attempt in range(5):
            try:
                resp = self.client.generate(prompt)
                parsed = _extract_json(resp)
                if isinstance(parsed, list):
                    return parsed
            except Exception:
                if attempt == 4:
                    raise
                time.sleep(2)
        return []

    def deduplicate(self, citations: list, report_text: str = "") -> list:
        """Deduplicate citations grouped by URL, keeping unique facts per URL."""
        if not citations:
            return []

        # Group by URL
        url_groups = {}
        for c in citations:
            url = c.get("url", "")
            fact = c.get("fact", "")
            if url not in url_groups:
                url_groups[url] = []
            url_groups[url].append(fact)

        result = []
        for url, facts in url_groups.items():
            if len(facts) == 1:
                result.append({"url": url, "facts": facts, "count": 1})
                continue

            # Use LLM to deduplicate
            lang = "zh" if _is_chinese(facts[0]) else "en"
            prompt_tmpl = FACT_DEDUP_PROMPT_ZH if lang == "zh" else FACT_DEDUP_PROMPT_EN
            statements_text = "\n".join(f"{i+1}. {f}" for i, f in enumerate(facts))
            prompt = prompt_tmpl.format(statements=statements_text)

            try:
                resp = self.client.generate(prompt)
                indices = _extract_json(resp)
                if isinstance(indices, list) and len(indices) > 0:
                    # Convert 1-based to 0-based
                    keep_idx = [i - 1 for i in indices if isinstance(i, (int, float)) and 1 <= i <= len(facts)]
                    unique_facts = [facts[i] for i in keep_idx] if keep_idx else facts
                else:
                    unique_facts = facts
            except Exception:
                unique_facts = facts

            result.append({"url": url, "facts": unique_facts, "count": len(unique_facts)})

        return result

    def scrape_url(self, url: str) -> str:
        """Scrape URL content using Jina Reader API."""
        if not url:
            return ""

        # Try direct fetch first
        try:
            resp = requests.get(url, timeout=15,
                                headers={"User-Agent": "Mozilla/5.0"})
            if resp.status_code == 200:
                text = resp.text
                # Simple text extraction (strip HTML tags)
                text = re.sub(r'<[^>]+>', ' ', text)
                text = re.sub(r'\s+', ' ', text).strip()
                if len(text) > 200:
                    return text[:10000]
        except Exception:
            pass

        # Try Jina API
        if self.jina_api_key:
            try:
                resp = requests.get(
                    f"https://r.jina.ai/{url}",
                    headers={
                        "Accept": "application/json",
                        "Authorization": f"Bearer {self.jina_api_key}",
                        "X-Timeout": "30000",
                    },
                    timeout=30,
                )
                if resp.status_code == 200:
                    data = resp.json()
                    content = data.get("data", {}).get("content", "")
                    if content:
                        return content[:10000]
            except Exception:
                pass

        return ""

    def validate(self, url: str, facts: list, report_text: str = "") -> dict:
        """Validate whether URL content supports the given facts."""
        if not url or not facts:
            return {"url": url, "validated": [], "supported": 0, "unsupported": 0, "unknown": 0}

        # Scrape URL content
        content = self.scrape_url(url)
        if not content:
            return {"url": url, "validated": [], "supported": 0, "unsupported": 0, "unknown": len(facts)}

        # Use LLM to validate
        lang = "zh" if _is_chinese(facts[0]) else "en"
        prompt_tmpl = FACT_VALIDATE_PROMPT_ZH if lang == "zh" else FACT_VALIDATE_PROMPT_EN
        statements_text = "\n".join(f"{i+1}. {f}" for i, f in enumerate(facts))
        prompt = prompt_tmpl.format(reference=content, statements=statements_text)

        try:
            resp = self.client.generate(prompt)
            results = _extract_json(resp)
            if isinstance(results, list):
                supported = sum(1 for r in results if r.get("result") == "supported")
                unsupported = sum(1 for r in results if r.get("result") == "unsupported")
                unknown = sum(1 for r in results if r.get("result") == "unknown")
                return {
                    "url": url,
                    "validated": results,
                    "supported": supported,
                    "unsupported": unsupported,
                    "unknown": unknown,
                }
        except Exception:
            pass

        return {"url": url, "validated": [], "supported": 0, "unsupported": 0, "unknown": len(facts)}

    def evaluate(self, report_text: str, max_workers: int = None) -> dict:
        """Run full FACT pipeline: extract → dedup → scrape → validate → metrics."""
        if max_workers is None:
            max_workers = MAX_WORKERS

        # Step 1: Extract
        print("  [FACT] Step 1/4: Extracting citations...")
        citations = self.extract_citations(report_text)
        print(f"         -> {len(citations)} citation(s) extracted")

        # Step 2: Deduplicate
        print("  [FACT] Step 2/4: Deduplicating...")
        deduped = self.deduplicate(citations, report_text)
        print(f"         -> {len(deduped)} unique URL(s)")

        if not deduped:
            return {
                "total_citations": 0,
                "unique_urls": 0,
                "citation_accuracy": 0.0,
                "effective_citations": 0.0,
                "details": [],
            }

        # Step 3: Validate in parallel
        print("  [FACT] Step 3/4: Validating citations...")
        validated = []
        lock = threading.Lock()

        def _validate(item):
            result = self.validate(item["url"], item["facts"], report_text)
            with lock:
                validated.append(result)

        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as ex:
            futures = [ex.submit(_validate, item) for item in deduped]
            for fut in concurrent.futures.as_completed(futures):
                pass  # results collected via _validate

        # Step 4: Calculate metrics
        print("  [FACT] Step 4/4: Computing metrics...")
        total_supported = sum(v.get("supported", 0) for v in validated)
        total_unsupported = sum(v.get("unsupported", 0) for v in validated)
        total_validated = total_supported + total_unsupported

        citation_accuracy = total_supported / total_validated if total_validated > 0 else 0.0
        effective_citations = total_supported  # avg effective citations = total supported

        result = {
            "total_citations": len(citations),
            "unique_urls": len(deduped),
            "total_validated": total_validated,
            "total_supported": total_supported,
            "total_unsupported": total_unsupported,
            "citation_accuracy": round(citation_accuracy, 4),
            "effective_citations": round(effective_citations, 2),
            "details": validated,
        }
        return result


# ═══════════════════════════════════════════════════════════════
#  Input helpers
# ═══════════════════════════════════════════════════════════════

def read_input(path: Path) -> str:
    """Read input file content (PDF → markdown, or plain text)."""
    suffix = path.suffix.lower()
    if suffix == ".pdf":
        try:
            import pymupdf4llm
            text = pymupdf4llm.to_markdown(str(path))
            print(f"[OK] PDF -> Markdown: {len(text)} chars")
            return text
        except ImportError:
            import fitz
            doc = fitz.open(str(path))
            text = "\n\n".join(page.get_text() for page in doc)
            print(f"[OK] PyMuPDF extracted: {len(text)} chars")
            return text
    elif suffix in (".md", ".txt"):
        return path.read_text(encoding="utf-8")
    elif suffix in (".tex",):
        return path.read_text(encoding="utf-8")
    else:
        raise ValueError(f"Unsupported file format: {suffix}")


def _compute_overall_verification_score(race_result: dict, fact_result: dict) -> float:
    """Combined score: RACE (0-1) → 0-5 scale, FACT accuracy → 0-5, weighted avg."""
    scores = []
    weights = []

    if race_result and "overall_score" in race_result:
        scores.append(race_result["overall_score"] * 5.0)
        weights.append(0.6)
    if fact_result and fact_result.get("total_validated", 0) > 0:
        scores.append(fact_result["citation_accuracy"] * 5.0)
        weights.append(0.4)

    if not scores:
        return 0.0
    return round(sum(s * w for s, w in zip(scores, weights)) / sum(weights), 2)


# ═══════════════════════════════════════════════════════════════
#  Main CLI
# ═══════════════════════════════════════════════════════════════

def main():
    parser = argparse.ArgumentParser(
        description="DeepResearch-Bench Simple Mode — RACE + FACT evaluation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--pdf", help="Path to PDF report")
    parser.add_argument("--markdown", "--md", help="Path to Markdown report")
    parser.add_argument("--latex", "--tex", help="Path to LaTeX report")
    parser.add_argument("--text", "--txt", help="Path to plain text report")

    parser.add_argument("--reference", "-r", help="Path to reference report (optional for RACE)")
    parser.add_argument("--task-prompt", "-t", help="Research task prompt (will auto-generate if not provided)")

    parser.add_argument("--race-only", action="store_true", help="Run only RACE evaluation")
    parser.add_argument("--fact-only", action="store_true", help="Run only FACT evaluation")
    parser.add_argument("--skip-race", action="store_true", help="Skip RACE evaluation")
    parser.add_argument("--skip-fact", action="store_true", help="Skip FACT evaluation")

    # ── Model / API selection (provider-agnostic) ────────────────
    parser.add_argument("--model", "--judge-model",
                        help="Model name for ALL judges (overridden by --race-model / --fact-model)")
    parser.add_argument("--api-base", "--api-base-url",
                        help="API base URL for ALL judges (overridden by --race-api-base / --fact-api-base)")
    parser.add_argument("--api-key",
                        help="API key for ALL judges (overridden by --race-api-key / --fact-api-key)")

    parser.add_argument("--race-model",
                        help="Model name for RACE judge (overrides --model, env RACE_MODEL)")
    parser.add_argument("--race-api-base",
                        help="API base URL for RACE judge (overrides --api-base, env RACE_API_BASE)")
    parser.add_argument("--race-api-key",
                        help="API key for RACE judge (overrides --api-key, env RACE_API_KEY)")

    parser.add_argument("--fact-model",
                        help="Model name for FACT judge (overrides --model, env FACT_MODEL)")
    parser.add_argument("--fact-api-base",
                        help="API base URL for FACT judge (overrides --api-base, env FACT_API_BASE)")
    parser.add_argument("--fact-api-key",
                        help="API key for FACT judge (overrides --api-key, env FACT_API_KEY)")

    parser.add_argument("--jina-api-key", help="Jina AI API key for web scraping (optional)")
    parser.add_argument("--output", "-o", default="", help="Output JSON path")
    parser.add_argument("--report", action="store_true", help="Generate human-readable report")

    args = parser.parse_args()

    # ── Input ───────────────────────────────────────────────────
    input_path = None
    for key in ("pdf", "markdown", "latex", "text"):
        val = getattr(args, key, None)
        if val:
            input_path = Path(val)
            break
    if not input_path or not input_path.exists():
        parser.print_help()
        print("\n[!] Please provide a valid input file (--pdf, --markdown, --latex, or --text)")
        sys.exit(1)

    report_text = read_input(input_path)

    # Reference
    reference_text = None
    if args.reference:
        ref_path = Path(args.reference)
        if ref_path.exists():
            reference_text = read_input(ref_path)

    # Task prompt — auto-generate from report if not provided
    task_prompt = args.task_prompt
    if not task_prompt:
        # Use first 500 chars as implicit prompt
        task_prompt = f"Research report evaluation. Report begins with: {report_text[:300]}..."

    # ── Models (provider-agnostic resolution) ──────────────────
    # Priority: CLI arg > env var (per-judge > global)
    race_model = args.race_model or args.model or RACE_MODEL
    fact_model = args.fact_model or args.model or FACT_MODEL
    race_api_key = args.race_api_key or args.api_key or RACE_API_KEY
    fact_api_key = args.fact_api_key or args.api_key or FACT_API_KEY
    race_api_base = args.race_api_base or args.api_base or RACE_API_BASE
    fact_api_base = args.fact_api_base or args.api_base or FACT_API_BASE

    # ── Determine what to run ───────────────────────────────────
    run_race = not args.skip_race and not args.fact_only
    run_fact = not args.skip_fact and not args.race_only
    if args.race_only:
        run_race, run_fact = True, False
    if args.fact_only:
        run_race, run_fact = False, True

    race_result = None
    fact_result = None

    # ── RACE Evaluation ─────────────────────────────────────────
    if run_race:
        print("\n" + "=" * 55)
        print("  RACE: Reference-based Adaptive Criteria-driven Evaluation")
        print("=" * 55)

        if not reference_text:
            print("  [WARN] No reference provided. RACE needs a reference report.")
            print("         Provide --reference for meaningful scores.")
            print("         Using the target report as itself (scores will be 0.5).\n")
            reference_text = report_text

        race_llm = LLMClient(model=race_model, api_key=race_api_key, base_url=race_api_base)
        evaluator = RACEEvaluator(llm_client=race_llm)

        print("  [RACE] Step 1/3: Generating dimension weights...")
        dim_weights = evaluator.generate_dimension_weights(task_prompt, samples=3)
        print(f"         Weights: {dim_weights}")

        print("  [RACE] Step 2/3: Generating per-dimension criteria...")
        criteria = evaluator.generate_all_criteria(task_prompt, num_criteria=3)
        for dim, crits in criteria.items():
            print(f"         {dim}: {len(crits)} criteria")

        print("  [RACE] Step 3/3: Scoring against reference...")
        race_result = evaluator.score(task_prompt, report_text, reference_text,
                                       dim_weights, criteria)
        print(f"\n  +-----------------------------------------------+")
        print(f"  |  RACE Overall Score:  {race_result['overall_score']:.4f}                   |")
        print(f"  |  Comprehensiveness:   {race_result['comprehensiveness']:.4f}                   |")
        print(f"  |  Insight/Depth:       {race_result['insight']:.4f}                   |")
        print(f"  |  Instruction-Follow:  {race_result['instruction_following']:.4f}                   |")
        print(f"  |  Readability:         {race_result['readability']:.4f}                   |")
        print(f"  +-----------------------------------------------+")

    # ── FACT Evaluation ─────────────────────────────────────────
    if run_fact:
        print("\n" + "=" * 55)
        print("  FACT: Factual Abundance & Citation Trustworthiness")
        print("=" * 55)

        fact_llm = LLMClient(model=fact_model, api_key=fact_api_key, base_url=fact_api_base)
        evaluator = FACTEvaluator(llm_client=fact_llm, jina_api_key=args.jina_api_key)
        fact_result = evaluator.evaluate(report_text)

        print(f"\n  +-----------------------------------------------+")
        print(f"  |  Citation Accuracy:   {fact_result['citation_accuracy']:.2%}                   |")
        print(f"  |  Effective Citations: {fact_result['effective_citations']:.1f}                   |")
        print(f"  |  Total Validated:     {fact_result['total_validated']}                          |")
        print(f"  |  +- Supported:        {fact_result['total_supported']}                          |")
        print(f"  |  +- Unsupported:      {fact_result['total_unsupported']}                          |")
        print(f"  +-----------------------------------------------+")

    # ── Combined Score ──────────────────────────────────────────
    overall_vs = _compute_overall_verification_score(
        race_result or {},
        fact_result or {"total_validated": 0, "citation_accuracy": 0.0},
    )

    result = {
        "benchmark": "DeepResearch-Bench",
        "input_file": str(input_path),
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "task_prompt": task_prompt,
        "race": race_result,
        "fact": fact_result,
        "overall_verification_score": overall_vs,
        "notes": (
            f"RACE model: {race_model} @ {race_api_base}, "
            f"FACT model: {fact_model} @ {fact_api_base}. "
            f"Reference: {args.reference or 'self (no reference provided)'}"
        ),
    }

    # Output
    output_path = args.output
    if not output_path:
        reports_dir = BENCH_DIR / "reports"
        reports_dir.mkdir(parents=True, exist_ok=True)
        ts = time.strftime("%Y%m%d_%H%M%S")
        output_path = str(reports_dir / f"deepresearch_simple_{ts}.json")

    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    Path(output_path).write_text(
        json.dumps(result, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"\n{'=' * 55}")
    print(f"  [OK] Results saved -> {output_path}")

    # Generate report
    if args.report:
        try:
            from generate_report import generate_report
            report_path = output_path.replace(".json", ".md")
            report_md = generate_report(result)
            Path(report_path).write_text(report_md, encoding="utf-8")
            print(f"  [OK] Report saved -> {report_path}")
        except ImportError:
            print("  [i] generate_report.py not found, skipping markdown report")

    print(f"\n  Overall Verification Score: {overall_vs:.2f}/5.0")
    print(f"{'=' * 55}\n")


if __name__ == "__main__":
    main()
