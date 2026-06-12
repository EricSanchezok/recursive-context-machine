<#
.SYNOPSIS
    ReportBench — 单篇论文陈述真实性评测的一键运行脚本

.DESCRIPTION
    从 PDF / Markdown / LaTeX 中提取引用表述和无引用表述，
    并用 LLM 验证无引用的事实性 claim，最终输出结构化评测报告（JSON）。

    两种模式:
      simple（默认） — 基于 LLM 知识判断，只需 OPENAI_API_KEY
      full          — 联网搜索验证 + URL 抓取对齐，还需 WEB_LLM_KEY

    环境变量:
      OPENAI_API_KEY   — 必需。LLM API 密钥
      OPENAI_BASE_URL  — 可选。API 地址
      WEB_LLM_KEY      — full 模式必需。联网模型 API Key（如 Gemini）
      WEB_LLM_BASE_URL — full 模式可选。联网模型 base URL

.PARAMETER Mode
    评测模式: simple（默认）| full

.PARAMETER Pdf
    PDF 论文路径

.PARAMETER Markdown
    Markdown 论文路径

.PARAMETER Latex
    LaTeX 论文路径

.PARAMETER Output
    输出 JSON 路径（默认 reports/reportbench_result.json）

.PARAMETER SkipNonCited
    跳过无引用表述验证

.PARAMETER SkipUrlVerify
    跳过 URL 抓取与对齐验证（仅提取统计）

.PARAMETER Help
    显示此帮助信息

.EXAMPLE
    .\run_reportbench.ps1 -Mode simple -Pdf paper.pdf
    .\run_reportbench.ps1 -Mode full -Markdown paper.md
    .\run_reportbench.ps1 -Latex paper.tex -SkipNonCited
#>

param(
    [ValidateSet("simple", "full")]
    [string]$Mode = "simple",
    [string]$Pdf,
    [string]$Markdown,
    [string]$Latex,
    [string]$Output = "",
    [switch]$SkipNonCited,
    [switch]$SkipUrlVerify,
    [switch]$Help
)

if ($Help) {
    Get-Help $PSCommandPath -Detailed
    exit 0
}

# ── 入口目录 ──────────────────────────────────────────────
$ScriptDir = Split-Path -Parent $PSCommandPath
Set-Location $ScriptDir

# ── 构建参数 ──────────────────────────────────────────────
$ArgsList = @()

$ArgsList += "--mode"; $ArgsList += $Mode

if ($Pdf)       { $ArgsList += "--pdf";       $ArgsList += $Pdf }
if ($Markdown)  { $ArgsList += "--markdown";  $ArgsList += $Markdown }
if ($Latex)     { $ArgsList += "--latex";     $ArgsList += $Latex }

if ($Output)    { $ArgsList += "--output";    $ArgsList += $Output }
if ($SkipNonCited)  { $ArgsList += "--skip-non-cited" }
if ($SkipUrlVerify) { $ArgsList += "--skip-url-verify" }

# 如果没有指定输入，显示帮助
if (-not $Pdf -and -not $Markdown -and -not $Latex) {
    Get-Help $PSCommandPath -Detailed
    Write-Host "`n[!] 请提供输入文件 ( -Pdf / -Markdown / -Latex )" -ForegroundColor Red
    exit 1
}

# ── 环境检查 ──────────────────────────────────────────────
if (-not $env:OPENAI_API_KEY) {
    Write-Host "[!] 需要设置 OPENAI_API_KEY 环境变量" -ForegroundColor Red
    exit 1
}

if ($Mode -eq "full" -and -not $env:WEB_LLM_KEY) {
    Write-Host "[WARN] full 模式推荐设置 WEB_LLM_KEY（联网模型 API Key）" -ForegroundColor Yellow
    Write-Host "       未设置时将降级使用 OPENAI_API_KEY 的 LLM 进行知识判断" -ForegroundColor Yellow
}

# ── 运行评测 ──────────────────────────────────────────────
Write-Host "`n╔══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         ReportBench ($Mode 模式)          ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"

try {
    python evaluate_simple.py @ArgsList
} catch {
    Write-Host "[!] 运行失败: $_" -ForegroundColor Red
    exit 1
}

Write-Host "`n[✓] 评测完成" -ForegroundColor Green
