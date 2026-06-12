"""
ReportBench Simple Mode — 单篇论文陈述真实性评测

从 PDF/Markdown/LaTeX 提取 citation claims 和 non-cited claims，
抓取引用 URL 并验证对齐，最后输出结构化评测报告。

用法:
    python evaluate_simple.py --markdown paper.md
    python evaluate_simple.py --pdf paper.pdf --output results/reportbench.json
    python evaluate_simple.py --latex paper.tex --skip-non-cited
"""
import argparse
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import pandas as pd

# ── ReportBench 内部模块 ──────────────────────────────────────
from statement.extract_citations import extract_citations_from_text
from statement.extract_no_citations import extract_no_citations_from_text
from statement.scrape_content import scrape_all
from statement.match_text import match_sentences
from statement.verify_alignment import verify
from statement.verify_no_citations_web import verify_no_citations_web
from utils import save_csv


def read_input(path: Path) -> str:
    """读取输入文件内容"""
    suffix = path.suffix.lower()
    if suffix == ".pdf":
        try:
            import pymupdf4llm
            text = pymupdf4llm.to_markdown(str(path))
            print(f"[✓] PDF 转 Markdown: {len(text)} 字符")
            return text
        except ImportError:
            print("[!] pymupdf4llm 未安装，尝试 PyMuPDF 备用方案")
            import fitz
            doc = fitz.open(str(path))
            text = "\n\n".join(page.get_text() for page in doc)
            print(f"[✓] PyMuPDF 提取文本: {len(text)} 字符")
            return text
    elif suffix in (".md", ".txt"):
        return path.read_text(encoding="utf-8")
    elif suffix in (".tex",):
        return path.read_text(encoding="utf-8")
    else:
        raise ValueError(f"不支持的文件格式: {suffix}")


def main():
    parser = argparse.ArgumentParser(description="ReportBench Simple Mode")
    parser.add_argument("--markdown", help="Markdown 文件路径")
    parser.add_argument("--pdf", help="PDF 文件路径")
    parser.add_argument("--latex", help="LaTeX 文件路径")
    parser.add_argument("--text", help="纯文本文件路径")
    parser.add_argument(
        "--output", "-o", default="reports/reportbench_result.json",
        help="输出 JSON 路径"
    )
    parser.add_argument(
        "--mode", choices=["simple", "full"], default="simple",
        help="simple=仅LLM知识验证（默认）, full=联网搜索+URL抓取验证"
    )
    parser.add_argument(
        "--work-dir", default="_reportbench_work",
        help="中间产物目录（提取的 CSV、抓取文本等）"
    )
    parser.add_argument(
        "--skip-non-cited", action="store_true",
        help="跳过无引用表述验证"
    )
    parser.add_argument(
        "--skip-url-verify", action="store_true",
        help="跳过 URL 抓取与对齐验证（仅提取统计）"
    )
    args = parser.parse_args()

    # ── 1. 读取输入 ──────────────────────────────────────────
    input_path = None
    for key in ("markdown", "pdf", "latex", "text"):
        val = getattr(args, key, None)
        if val:
            input_path = Path(val)
            break

    if not input_path or not input_path.exists():
        parser.print_help()
        print("\n[!] 请提供有效的输入文件路径")
        sys.exit(1)

    work_dir = Path(args.work_dir)
    work_dir.mkdir(parents=True, exist_ok=True)
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    print(f"输入文件: {input_path}")
    report_text = read_input(input_path)

    # ── 2. 提取引用表述 ──────────────────────────────────────
    print("\n" + "=" * 50)
    print("阶段 1/5: 提取引用表述")
    citations_csv = work_dir / "citations.csv"
    df_citations = extract_citations_from_text(report_text, citations_csv, work_dir / "url_cache.csv")
    n_cited = len(df_citations)
    print(f"  → 提取到 {n_cited} 条引用表述")

    # ── 3. 提取无引用表述（可选）─────────────────────────────
    df_no_citations = pd.DataFrame()
    no_cit_stats = {}
    if not args.skip_non_cited:
        print("\n" + "=" * 50)
        print("阶段 2/5: 提取无引用表述")
        no_cit_csv = work_dir / "no_citations.csv"
        df_no_citations = extract_no_citations_from_text(report_text, df_citations, no_cit_csv)
        n_non_cited = len(df_no_citations)
        print(f"  → 提取到 {n_non_cited} 条无引用表述")

        # ── 3b. 验证无引用表述 ────────────────────
        if not df_no_citations.empty:
            print("\n" + "=" * 50)
            print(f"阶段 3/5: 验证无引用表述 (模式: {args.mode})")
            web_verify_csv = work_dir / "no_citations_web_verification.csv"
            web_final_csv = work_dir / "no_citations_web_final.csv"
            try:
                df_verify, df_web_final = verify_no_citations_web(
                    df_no_citations, web_verify_csv, web_final_csv, mode=args.mode
                )
                if not df_web_final.empty:
                    correct = len(df_web_final[df_web_final["final_decision"] == "正确"])
                    incorrect = len(df_web_final[df_web_final["final_decision"] == "错误"])
                    no_cit_stats = {
                        "total": len(df_web_final),
                        "verified_correct": int(correct),
                        "verified_incorrect": int(incorrect),
                        "accuracy_rate": round(
                            correct / len(df_web_final), 4
                        ) if len(df_web_final) > 0 else 0.0,
                    }
                    print(f"  → 正确: {correct}, 错误: {incorrect}")
            except Exception as e:
                print(f"  [!] 无引用表述验证失败: {e}")

    # ── 4. 抓取 URL 内容（仅 full 模式）───────────────────
    cited_stats = {"total": n_cited}
    if args.mode == "full" and not args.skip_url_verify and not df_citations.empty:
        print("\n" + "=" * 50)
        print("阶段 4/5: 抓取引用 URL 内容")
        raw_dir = work_dir / "raw_texts"
        scrape_all(df_citations, raw_dir)

        # ── 5. 匹配句子 ──────────────────────────────────────
        print("\n" + "=" * 50)
        print("阶段 5/5: 匹配与对齐验证")
        matched_csv = work_dir / "matched.csv"
        df_match = match_sentences(df_citations, raw_dir, matched_csv)

        if not df_match.empty:
            # ── 6. 验证对齐 ────────────────────────────────
            final_csv = work_dir / "final.csv"
            match_rate = verify(df_match, final_csv)

            # 详细统计
            if Path(final_csv).exists():
                df_final = pd.read_csv(final_csv, encoding="utf-8-sig")
                if not df_final.empty:
                    match_count = int(df_final["match"].sum())
                    no_match_count = len(df_final) - match_count
                    cited_stats.update({
                        "total_matched": len(df_match),
                        "match_count": match_count,
                        "no_match_count": no_match_count,
                        "match_rate": round(match_count / len(df_final), 4)
                        if len(df_final) > 0 else 0.0,
                    })
                    print(f"  → Match Rate: {match_rate:.2%}")

    # ── 7. 汇总报告 ──────────────────────────────────────────
    result = {
        "benchmark": "ReportBench",
        "input_file": str(input_path),
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "summary": {
            "cited_statements": cited_stats,
            "non_cited_statements": no_cit_stats,
        },
        "overall_verification_score": _compute_overall(cited_stats, no_cit_stats),
        "work_dir": str(work_dir),
        "mode": args.mode,
        "notes": (
            "simple 模式: 基于 LLM 知识判断（无需联网）"
            if args.mode == "simple"
            else "full 模式: 联网搜索验证 + URL 抓取对齐"
        ),
    }

    output_path.write_text(
        json.dumps(result, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"\n{'=' * 50}")
    print(f"[✓] 评测完成 → {output_path}")

    # 打印摘要
    s = result["summary"]
    ov = result["overall_verification_score"]
    print(f"\n{'=' * 50}")
    print(f"ReportBench 评测摘要")
    print(f"{'=' * 50}")
    print(f"  引用陈述: {s['cited_statements']['total']} 条")
    if "match_rate" in s["cited_statements"]:
        print(f"    Match Rate: {s['cited_statements']['match_rate']:.1%}")
        print(f"    对齐匹配: {s['cited_statements']['match_count']}")
        print(f"    对齐不匹配: {s['cited_statements']['no_match_count']}")
    if s["non_cited_statements"].get("total", 0) > 0:
        nc = s["non_cited_statements"]
        print(f"  无引用陈述: {nc['total']} 条")
        print(f"    验证正确: {nc.get('verified_correct', 0)}")
        print(f"    验证错误: {nc.get('verified_incorrect', 0)}")
        print(f"    正确率: {nc.get('accuracy_rate', 0):.1%}")
    print(f"  综合验证得分: {ov:.2f}/5.0")
    print(f"{'=' * 50}\n")


def _compute_overall(cited: dict, non_cited: dict) -> float:
    """综合评分 0-5：引用对齐 (0-5) + 无引用准确率 (0-5) 的加权平均"""
    scores = []
    weights = []

    if "match_rate" in cited:
        scores.append(cited["match_rate"] * 5.0)
        weights.append(0.7)

    if non_cited and non_cited.get("accuracy_rate", 0) > 0:
        scores.append(non_cited["accuracy_rate"] * 5.0)
        weights.append(0.3)

    if not scores:
        return 0.0

    return round(sum(s * w for s, w in zip(scores, weights)) / sum(weights), 2)


if __name__ == "__main__":
    main()
