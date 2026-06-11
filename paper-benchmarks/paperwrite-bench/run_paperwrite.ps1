param(
    [switch]$Help,
    # ── Simple mode (no GT required) ──
    [switch]$Simple,
    [string]$Latex,
    [string]$Markdown,
    [string]$Pdf,
    [string]$EvalPoints,
    [switch]$AutoEvalPoints,
    [switch]$All,
    [switch]$FullSanity,
    [switch]$SelfConsistency,
    [switch]$Citations,
    [switch]$FigureTable,
    # ── Full mode (needs GT + official PaperRecon) ──
    [string]$Paper,
    [switch]$AllPapers,
    [ValidateSet("rubric", "hallucination", "citation", "all")]
    [string]$EvalMode = "all",
    # ── Shared ──
    [string]$Model = "gpt-4o",
    [switch]$Force,
    [switch]$SkipSetupCheck
)

$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$benchDir = Split-Path -Parent $scriptDir

function ShowHelp {
    Write-Host @"
PaperWrite-Bench — One-click evaluation runner
===============================================

Two modes available:

SIMPLE MODE (no ground truth required):
  .\run_paperwrite.ps1 -Simple -Pdf paper.pdf -All
  .\run_paperwrite.ps1 -Simple -Markdown paper.md -AutoEvalPoints
  .\run_paperwrite.ps1 -Simple -Latex paper.tex -EvalPoints points.json -Citations

FULL MODE (needs GT + official PaperRecon dataset):
  .\run_paperwrite.ps1                        Evaluate paper_1 (default)
  .\run_paperwrite.ps1 -Paper paper_3         Evaluate specific paper
  .\run_paperwrite.ps1 -AllPapers -EvalMode all

Parameters:
  -Simple         Simple mode: no GT needed
  -Pdf           Path to PDF file (auto-converted, simple mode)
  -Latex         Path to LaTeX file (simple mode)
  -Markdown      Path to Markdown file (simple mode, e.g., PDF converted)
  -EvalPoints     Path to eval_points.json (simple mode)
  -AutoEvalPoints Auto-generate eval_points via LLM (simple mode)
  -All            Rubric + all extra checks (simple mode)
  -FullSanity     Self-consistency + citations + figure/table (simple mode)
  -SelfConsistency Self-consistency hallucination check (simple mode)
  -Citations      Citation sanity check (simple mode)
  -FigureTable    Figure/table coverage check (simple mode)
  -Paper          Paper ID (e.g., paper_1). Default: paper_1
  -AllPapers      Evaluate all available papers
  -Model          Evaluation model (default: gpt-4o)
  -EvalMode       Full mode: rubric, hallucination, citation, all (default)
  -Force          Re-evaluate existing results
  -SkipSetupCheck Skip the setup existence check
"@
    exit 0
}

if ($Help) { ShowHelp }

# ════════════════════════════════════════════════
#  SIMPLE MODE — no GT required
# ════════════════════════════════════════════════
if ($Simple) {
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host "  PAPERWRITE-BENCH — SIMPLE MODE" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host ""

    $inputFile = $null
    if ($Pdf)     { $inputFile = $Pdf;     $type = "pdf" }
    if ($Latex)   { $inputFile = $Latex;   $type = "latex" }
    if ($Markdown) { $inputFile = $Markdown; $type = "markdown" }

    if (-not $inputFile) {
        Write-Host "[X] Specify --latex or --markdown" -ForegroundColor Red
        exit 1
    }
    if (-not (Test-Path $inputFile)) {
        Write-Host "[X] File not found: $inputFile" -ForegroundColor Red
        exit 1
    }

    Write-Host "Input: $inputFile" -ForegroundColor Gray
    Write-Host "Model: $Model" -ForegroundColor Gray

    # Build command
    $cmd = @("python", "$scriptDir\evaluate_simple.py", "--$type", $inputFile, "--model", $Model)

    # Rubric options
    if ($EvalPoints) {
        $cmd += @("--eval-points", $EvalPoints)
        Write-Host "Eval points: $EvalPoints" -ForegroundColor Gray
    }
    if ($AutoEvalPoints -or (-not $EvalPoints)) {
        if (-not $EvalPoints) {
            $cmd += "--auto-eval-points"
            Write-Host "Eval points: auto-generated via LLM" -ForegroundColor Yellow
        }
    }

    # Extra checks
    if ($All)     { $cmd += "--all"; Write-Host "Running: rubric + all extra checks" -ForegroundColor Green }
    if ($FullSanity) { $cmd += "--full-sanity"; Write-Host "Running: extra sanity checks" -ForegroundColor Green }
    if ($SelfConsistency) { $cmd += "--self-consistency" }
    if ($Citations) { $cmd += "--citations" }
    if ($FigureTable) { $cmd += "--figure-table" }

    $reportDir = Join-Path $scriptDir "reports"
    New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $outputPath = Join-Path $reportDir "paperwrite_simple_$timestamp.json"
    $cmd += @("--output", $outputPath)
    $cmd += "--report"

    Write-Host "`nRunning: $($($cmd -join ' '))" -ForegroundColor Gray
    & $cmd[0] $cmd[1..$($cmd.Length-1)]
    $evalOk = $LASTEXITCODE -eq 0

    $now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $summaryContent = @"
# PaperWrite-Bench — Simple Evaluation Summary

**Date**: $now
**Input**: $(Split-Path $inputFile -Leaf)
**Model**: $Model
**Checks**: $(if ($All) { 'Rubric + All Sanity' } elseif ($FullSanity) { 'Self-Consistency + Citations + Figure/Table' } else { 'Rubric' })

---

*Generated by run_paperwrite.ps1 (simple mode)*
"@
    $summaryContent | Out-File -FilePath (Join-Path $reportDir "paperwrite_summary.md") -Encoding utf8

    Write-Host ""
    $reportMd = $outputPath -replace '\.json$', '.md'
    Write-Host "Report: $reportMd" -ForegroundColor Green
    Write-Host "Results: $outputPath" -ForegroundColor Green
    Write-Host ""
    Write-Host "Quick links:" -ForegroundColor Yellow
    Write-Host "  .\run_paperwrite.ps1 -Simple -Pdf paper.pdf -All         # End-to-end: PDF→rubric+sanity"
    Write-Host "  .\run_paperwrite.ps1 -Simple -Markdown paper.md -All"
    Write-Host "  .\run_paperwrite.ps1 -Simple -Latex paper.tex -SelfConsistency"

    exit $(if ($evalOk) { 0 } else { 1 })
}

# ════════════════════════════════════════════════
#  FULL MODE — needs GT + PaperRecon dataset
# ════════════════════════════════════════════════
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  PAPERWRITE-BENCH — FULL MODE (requires GT)" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

if (-not $Paper -and -not $AllPapers) { $Paper = "paper_1" }
$paperArg = if ($AllPapers) { "--all" } else { "--paper", $Paper }

$repoDir = Join-Path $scriptDir "PaperRecon"
if (-not $SkipSetupCheck -and -not (Test-Path (Join-Path $repoDir "README.md"))) {
    Write-Host "[!] PaperRecon repo not set up." -ForegroundColor Yellow
    Write-Host "    Running setup.ps1 (clone only)..." -ForegroundColor Yellow
    & (Join-Path $scriptDir "setup.ps1") -SkipClone:$false
    if (-not (Test-Path (Join-Path $repoDir "README.md"))) {
        Write-Host "[X] Setup failed." -ForegroundColor Red
        exit 1
    }
}

Write-Host "[1/4] Checking environment..." -ForegroundColor Yellow
python "$benchDir\run_evaluation.py" --check
Write-Host ""

Write-Host "[2/4] Running evaluation..." -ForegroundColor Yellow
$modelArgs = @()
if ($Model) { $modelArgs = @("--model", $Model) }

$cmd = @(
    "python", "$scriptDir\run_evaluation.py",
    $paperArg[0], $paperArg[1],
    "--eval-mode", $EvalMode
) + $modelArgs
if ($Force) { $cmd += "--force" }

Write-Host "  >> $($cmd -join ' ')" -ForegroundColor Gray
& $cmd[0] $cmd[1..$($cmd.Length-1)]
$evalOk = $LASTEXITCODE -eq 0
Write-Host ""

Write-Host "[3/4] Generating summary report..." -ForegroundColor Yellow
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$reportDir = Join-Path $benchDir "reports"
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null

$target = if ($AllPapers) { "all papers" } else { $Paper }
$summaryContent = @"
# PaperWrite-Bench — Full Evaluation Summary

**Date**: $now
**Target**: $target
**Model**: $(if ($Model) { $Model } else { "default (config)" })
**Eval Mode**: $EvalMode

---

*Generated by run_paperwrite.ps1 (full mode)*
"@
$summaryContent | Out-File -FilePath (Join-Path $reportDir "paperwrite_summary.md") -Encoding utf8

Write-Host "[4/4] Done" -ForegroundColor Yellow
Write-Host ""
Write-Host "Report: $reportDir\paperwrite_summary.md" -ForegroundColor Green
Write-Host ""
Write-Host "Quick links:" -ForegroundColor Yellow
Write-Host "  .\run_paperwrite.ps1 -AllPapers                         # Full: all papers"
Write-Host "  .\run_paperwrite.ps1 -Simple -Markdown paper.md -All    # Simple: all checks"

exit $(if ($evalOk) { 0 } else { 1 })
