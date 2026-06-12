<#
.SYNOPSIS
    DeepResearch-Bench — 一键评测运行脚本
.DESCRIPTION
    两种运行模式:
      Simple Mode (默认): 单篇研究报告的 RACE + FACT 联合评测
      Full Mode:          官方 100 任务基准评测（需 clone 官方仓库）

    Simple Mode 参数:
      -Pdf / -Markdown / -Latex : 输入研究报告
      -Reference                  : 参考报告（RACE 需要）
      -TaskPrompt                 : 研究任务描述
      -RaceOnly / -FactOnly       : 仅运行特定评估
      -Model                      : 所有 judge 的模型名（可被 -RaceModel / -FactModel 覆盖）
      -ApiBase                    : 所有 judge 的 API 地址（可被 -RaceApiBase / -FactApiBase 覆盖）
      -ApiKey                     : 所有 judge 的 API 密钥（可被 -RaceApiKey / -FactApiKey 覆盖）
      -RaceModel / -FactModel     : 分别指定 RACE/FACT 模型
      -RaceApiBase / -FactApiBase : 分别指定 RACE/FACT API 地址
      -RaceApiKey / -FactApiKey   : 分别指定 RACE/FACT API 密钥
      -JinaApiKey                 : Jina AI API 密钥（FACT 网页抓取）
      -Report                     : 同时生成 Markdown 报告
      -Output                     : 输出 JSON 路径

    环境变量配置:
      API_KEY / API_BASE_URL          : 全局 API 凭证
      RACE_MODEL / RACE_API_KEY / RACE_API_BASE : RACE judge 配置
      FACT_MODEL / FACT_API_KEY / FACT_API_BASE : FACT judge 配置

    支持任意 OpenAI 兼容 API:
      OpenAI, DeepSeek, OpenRouter, Groq, Together AI 等

    示例:
      .\run_deepresearch.ps1 -Pdf report.pdf -Reference ref.md
      .\run_deepresearch.ps1 -Pdf report.pdf -Model claude-sonnet-4 -ApiBase https://api.anthropic.com/v1
      .\run_deepresearch.ps1 -Latex paper.tex -FactOnly
      .\run_deepresearch.ps1 -Markdown paper.md -RaceModel deepseek-chat -RaceApiBase https://api.deepseek.com/v1

    Full Mode 参数:
      -ModelName                  : 模型名称（对应 raw_data 中的 JSONL 文件名）
      -SkipCleaning               : 跳过文章清洗步骤
      -Limit N                    : 仅处理 N 个任务
      -OnlyZh / -OnlyEn           : 仅处理特定语言
      -Force                      : 强制重新评测

.PARAMETER Simple
    使用 Simple Mode（单篇评测，默认）
.PARAMETER Full
    使用 Full Mode（官方 100 任务基准）
.PARAMETER Help
    显示帮助
#>

param(
    # Mode selection
    [switch]$Simple,
    [switch]$Full,
    [switch]$Help,

    # Simple mode params
    [string]$Pdf,
    [string]$Markdown,
    [string]$Latex,
    [string]$Text,
    [string]$Reference,
    [string]$TaskPrompt,
    [switch]$RaceOnly,
    [switch]$FactOnly,
    [switch]$SkipRace,
    [switch]$SkipFact,
    [string]$Model = "",
    [string]$ApiBase = "",
    [string]$ApiKey = "",
    [string]$RaceModel = "",
    [string]$RaceApiBase = "",
    [string]$RaceApiKey = "",
    [string]$FactModel = "",
    [string]$FactApiBase = "",
    [string]$FactApiKey = "",
    [string]$JinaApiKey = "",
    [switch]$Report,
    [string]$Output = "",

    # Full mode params
    [string]$ModelName = "",
    [switch]$SkipCleaning,
    [int]$Limit = 0,
    [switch]$OnlyZh,
    [switch]$OnlyEn,
    [switch]$Force
)

if ($Help) {
    Get-Help $PSCommandPath -Detailed
    exit 0
}

$ScriptDir = Split-Path -Parent $PSCommandPath
Set-Location $ScriptDir

# ── Environment check ──
$hasAnyKey = ($env:API_KEY) -or ($env:RACE_API_KEY) -or ($env:FACT_API_KEY) -or ($env:OPENAI_API_KEY)
if (-not $hasAnyKey -and -not $ApiKey -and -not $RaceApiKey -and -not $FactApiKey) {
    Write-Host "[!] No API key found. Set one of these environment variables:" -ForegroundColor Yellow
    Write-Host "    API_KEY, RACE_API_KEY, FACT_API_KEY, or OPENAI_API_KEY" -ForegroundColor Yellow
    Write-Host "    Or pass --api-key / --race-api-key / --fact-api-key" -ForegroundColor Yellow
    Write-Host "    Continuing anyway (will fail at LLM call if not configured)..." -ForegroundColor DarkYellow
}

# ── Simple Mode (default) ──
if (-not $Full) {
    if (-not $Pdf -and -not $Markdown -and -not $Latex -and -not $Text) {
        Get-Help $PSCommandPath -Detailed
        Write-Host "`n[!] Simple Mode: Provide an input file (-Pdf, -Markdown, -Latex, or -Text)" -ForegroundColor Red
        exit 1
    }

    $ArgsList = @()

    if ($Pdf)       { $ArgsList += "--pdf";       $ArgsList += $Pdf }
    if ($Markdown)  { $ArgsList += "--markdown";  $ArgsList += $Markdown }
    if ($Latex)     { $ArgsList += "--latex";     $ArgsList += $Latex }
    if ($Text)      { $ArgsList += "--text";      $ArgsList += $Text }
    if ($Reference) { $ArgsList += "--reference"; $ArgsList += $Reference }
    if ($TaskPrompt){ $ArgsList += "--task-prompt"; $ArgsList += $TaskPrompt }
    if ($RaceOnly)  { $ArgsList += "--race-only" }
    if ($FactOnly)  { $ArgsList += "--fact-only" }
    if ($SkipRace)  { $ArgsList += "--skip-race" }
    if ($SkipFact)  { $ArgsList += "--skip-fact" }
    if ($Model)     { $ArgsList += "--model";      $ArgsList += $Model }
    if ($ApiBase)   { $ArgsList += "--api-base";   $ArgsList += $ApiBase }
    if ($ApiKey)    { $ArgsList += "--api-key";    $ArgsList += $ApiKey }
    if ($RaceModel) { $ArgsList += "--race-model"; $ArgsList += $RaceModel }
    if ($RaceApiBase) { $ArgsList += "--race-api-base"; $ArgsList += $RaceApiBase }
    if ($RaceApiKey)  { $ArgsList += "--race-api-key";  $ArgsList += $RaceApiKey }
    if ($FactModel) { $ArgsList += "--fact-model"; $ArgsList += $FactModel }
    if ($FactApiBase) { $ArgsList += "--fact-api-base"; $ArgsList += $FactApiBase }
    if ($FactApiKey)  { $ArgsList += "--fact-api-key";  $ArgsList += $FactApiKey }
    if ($JinaApiKey){ $ArgsList += "--jina-api-key"; $ArgsList += $JinaApiKey }
    if ($Report)    { $ArgsList += "--report" }
    if ($Output)    { $ArgsList += "--output";    $ArgsList += $Output }

    Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║   DeepResearch-Bench (Simple Mode)       ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    try {
        python evaluate_simple.py @ArgsList
    } catch {
        Write-Host "[!] Evaluation failed: $_" -ForegroundColor Red
        exit 1
    }
    exit 0
}

# ── Full Mode ──
if (-not $ModelName) {
    Write-Host "[!] Full Mode requires -ModelName" -ForegroundColor Red
    exit 1
}

Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   DeepResearch-Bench (Full Mode)          ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "Note: Full Mode requires the official DeepResearch Bench dataset." -ForegroundColor Yellow
Write-Host "Run setup.ps1 first to download the data, or manually place files in:" -ForegroundColor Yellow
Write-Host "  data/prompt_data/query.jsonl" -ForegroundColor Yellow
Write-Host "  data/test_data/raw_data/<model>.jsonl" -ForegroundColor Yellow
Write-Host "  data/test_data/cleaned_data/reference.jsonl" -ForegroundColor Yellow
Write-Host "  data/criteria_data/criteria.jsonl" -ForegroundColor Yellow
Write-Host ""

# Check if data exists
$queryFile = Join-Path $ScriptDir "data/prompt_data/query.jsonl"
if (-not (Test-Path $queryFile)) {
    Write-Host "[!] Benchmark data not found. Run setup.ps1 first." -ForegroundColor Red
    Write-Host "    Or download from: https://github.com/Ayanami0730/deep_research_bench" -ForegroundColor Red
    exit 1
}

# Build RACE command
$RaceArgs = @()
$RaceArgs += $ModelName
$RaceArgs += "--query_file"; $RaceArgs += (Join-Path $ScriptDir "data/prompt_data/query.jsonl")
$RaceArgs += "--raw_data_dir"; $RaceArgs += (Join-Path $ScriptDir "data/test_data/raw_data")
$RaceArgs += "--cleaned_data_dir"; $RaceArgs += (Join-Path $ScriptDir "data/test_data/cleaned_data")
$RaceArgs += "--output_dir"; $RaceArgs += (Join-Path $ScriptDir "results/race/$ModelName")
$RaceArgs += "--max_workers"; $RaceArgs += "5"

if ($SkipCleaning) { $RaceArgs += "--skip_cleaning" }
if ($Limit -gt 0)  { $RaceArgs += "--limit"; $RaceArgs += $Limit.ToString() }
if ($OnlyZh)       { $RaceArgs += "--only_zh" }
if ($OnlyEn)       { $RaceArgs += "--only_en" }
if ($Force)        { $RaceArgs += "--force" }

Write-Host "[1/2] Running RACE evaluation..." -ForegroundColor Yellow
try {
    python evaluate_simple.py @RaceArgs
} catch {
    Write-Host "[!] RACE evaluation failed: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "[i] Full Mode RACE complete. FACT pipeline requires the official code." -ForegroundColor Yellow
Write-Host "    See: https://github.com/Ayanami0730/deep_research_bench" -ForegroundColor Yellow
Write-Host ""
