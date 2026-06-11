param(
    [switch]$Help,

    # Full mode (uses official MLR-Bench repo)
    [ValidateSet("end-to-end", "stepwise")]
    [string]$Mode = "stepwise",
    [string]$Review,
    [string]$CodingAgent,
    [string]$InputDir,

    # Simple mode (no GT required, self-contained evaluator)
    [switch]$Simple,
    [string]$Pdf,
    [string]$Latex,
    [string]$Markdown,
    [ValidateSet("overall", "consistency", "citations", "figure-table", "sanity", "all")]
    [string]$Eval = "overall",

    # Common
    [string]$Model = "gpt-4o",
    [switch]$SkipSetupCheck
)

$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$benchDir = Split-Path -Parent $scriptDir

function ShowHelp {
    Write-Host @"
MLR-Bench — One-click evaluation runner
========================================

Evaluates open-ended ML research quality (1-10 rubric).

USAGE:

  ## Full mode (requires official MLR-Bench repo)
  .\run_mlr.ps1                                        Stepwise (default)
  .\run_mlr.ps1 -Mode end-to-end                       End-to-end
  .\run_mlr.ps1 -Review overall                        Only overall review
  .\run_mlr.ps1 -Mode stepwise -CodingAgent claude     Specific coding agent
  .\run_mlr.ps1 -Model claude-sonnet-4                 Different eval model

  ## Simple mode (no GT required, self-contained)
  .\run_mlr.ps1 -Simple -Pdf paper.pdf -All
  .\run_mlr.ps1 -Simple -Markdown paper.md
  .\run_mlr.ps1 -Simple -Latex paper.tex -Eval consistency
  .\run_mlr.ps1 -Simple -Pdf paper.pdf -All -Model deepseek-chat

PARAMETERS (Full mode):
  -Help           Show this help message
  -Mode           Evaluation mode: end-to-end or stepwise (default)
  -Model          Model name (default: gpt-4o)
  -Review         Single review stage: overall, idea, proposal, experiments, writeup
  -CodingAgent    Coding agent name (end-to-end mode only)
  -InputDir       Input directory with results to review
  -SkipSetupCheck Skip the setup existence check

PARAMETERS (Simple mode):
  -Simple         Enable simple mode (self-contained evaluator)
  -Pdf            Path to PDF file (auto-converted)
  -Latex          Path to LaTeX file
  -Markdown       Path to Markdown file
  -Eval           Evaluation scope:
                  overall      Overall review (Clarity/Novelty/Soundness/Significance)
                  consistency  Self-consistency check only
                  citations    Citation sanity check only
                  figure-table Figure/table coverage check only
                  sanity       All extra checks (no overall review)
                  all          Overall review + all extra checks
  -Model          LLM model (default: gpt-4o). Use -Model deepseek-chat for DeepSeek.
"@
    exit 0
}

if ($Help) { ShowHelp }

# ═══════════════════════════════════════════════════════════════════════
#  SIMPLE MODE
# ═══════════════════════════════════════════════════════════════════════

if ($Simple) {
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host "  MLR-BENCH — Simple Mode" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host ""

    # ── Validate input ──
    $inputFile = $null
    $inputType = ""
    if ($Pdf)     { $inputFile = $Pdf;     $inputType = "pdf" }
    if ($Latex)   { $inputFile = $Latex;   $inputType = "latex" }
    if ($Markdown) { $inputFile = $Markdown; $inputType = "markdown" }

    if (-not $inputFile) {
        Write-Host "[X] Simple mode requires -Pdf, -Latex, or -Markdown" -ForegroundColor Red
        Write-Host "  Example: .\run_mlr.ps1 -Simple -Pdf paper.pdf -All" -ForegroundColor Yellow
        exit 1
    }
    if (-not (Test-Path $inputFile)) {
        Write-Host "[X] File not found: $inputFile" -ForegroundColor Red
        exit 1
    }

    # ── Map eval scope to flags ──
    $evalFlags = @{}
    switch ($Eval) {
        "overall"      { $evalFlags = @{ } }
        "consistency"  { $evalFlags = @{ 'self-consistency' = $true } }
        "citations"    { $evalFlags = @{ 'citations' = $true } }
        "figure-table" { $evalFlags = @{ 'figure-table' = $true } }
        "sanity"       { $evalFlags = @{ 'full-sanity' = $true } }
        "all"          { $evalFlags = @{ 'all' = $true } }
    }

    # ── Build command ──
    $cmd = @("python", "$scriptDir\evaluate_simple.py")
    $cmd += @("--$inputType", (Resolve-Path $inputFile).Path)
    $cmd += @("--model", $Model)
    $cmd += @("--output", (Join-Path $benchDir "reports\mlr_simple_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"))
    $cmd += "--report"
    foreach ($kv in $evalFlags.GetEnumerator()) {
        $cmd += "--$($kv.Key)"
    }

    # ── Run ──
    Write-Host "  >> $($cmd -join ' ')" -ForegroundColor Gray
    & $cmd[0] $cmd[1..$($cmd.Length-1)]

    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "Done" -ForegroundColor Green
        $out = $cmd | Where-Object { $_ -like "*.json" } | Select-Object -First 1
        if ($out) {
            $reportMd = $out -replace '\.json$', '.md'
            Write-Host "Report: $reportMd" -ForegroundColor Green
            Write-Host "Results: $out" -ForegroundColor Green
        }
    } else {
        Write-Host "Evaluation failed" -ForegroundColor Red
        exit 1
    }
    exit 0
}

# ═══════════════════════════════════════════════════════════════════════
#  FULL MODE (original MLR-Bench pipeline)
# ═══════════════════════════════════════════════════════════════════════

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  MLR-BENCH" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$repoDir = Join-Path $scriptDir "mlrbench"
if (-not $SkipSetupCheck -and -not (Test-Path (Join-Path $repoDir "README.md"))) {
    Write-Host "[!] MLR-Bench repo not set up." -ForegroundColor Yellow
    Write-Host "    Running setup.ps1 (clone only)..." -ForegroundColor Yellow
    & (Join-Path $scriptDir "setup.ps1") -SkipClone:$false
    Write-Host ""
    if (-not (Test-Path (Join-Path $repoDir "README.md"))) {
        Write-Host "[X] Setup failed. Please run setup.ps1 manually." -ForegroundColor Red
        exit 1
    }
}

Write-Host "[1/3] Checking environment..." -ForegroundColor Yellow
python "$benchDir\run_evaluation.py" --check
Write-Host ""

Write-Host "[2/3] Running evaluation..." -ForegroundColor Yellow
$cmd = @("python", "$scriptDir\run_evaluation.py")

if ($Review) {
    $cmd += @("--review", $Review, "--model-name", $Model)
    if ($InputDir) { $cmd += @("--input-dir", $InputDir) }
} elseif ($Mode -eq "end-to-end") {
    $cmd += @("--end-to-end", "--model-name", $Model)
    if ($CodingAgent) { $cmd += @("--coding-agent", $CodingAgent) }
} else {
    $cmd += @("--stepwise", "--model-name", $Model)
}

Write-Host "  >> $($cmd -join ' ')" -ForegroundColor Gray
& $cmd[0] $cmd[1..$($cmd.Length-1)]
$evalOk = $LASTEXITCODE -eq 0
if ($evalOk) {
    Write-Host "  Evaluation done" -ForegroundColor Green
} else {
    Write-Host "  Evaluation had errors" -ForegroundColor Red
}
Write-Host ""

Write-Host "[3/3] Generating summary report..." -ForegroundColor Yellow
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$reportDir = Join-Path $benchDir "reports"
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null

$summaryContent = @"
# MLR-Bench — Evaluation Summary

**Date**: $now
**Mode**: $(if ($Review) { "review:$Review" } else { $Mode })
**Model**: $Model

---

## Result

| Item | Status |
|------|:------:|
| Evaluation | $(if ($evalOk) { "✅" } else { "❌" }) |

---

*Generated by run_mlr.ps1*
"@

$summaryContent | Out-File -FilePath (Join-Path $reportDir "mlr_summary.md") -Encoding utf8

Write-Host "Done" -ForegroundColor Yellow
Write-Host ""
Write-Host "Report: $reportDir\mlr_summary.md" -ForegroundColor Green
Write-Host ""
Write-Host "Quick links:" -ForegroundColor Yellow
Write-Host "  .\run_mlr.ps1 -Simple -Pdf paper.pdf -All            # PDF → overall review + checks"
Write-Host "  .\run_mlr.ps1 -Simple -Pdf paper.pdf                  # PDF → overall only"
Write-Host "  .\run_mlr.ps1 -Simple -Markdown paper.md -Eval all    # Markdown → all"
Write-Host "  .\run_mlr.ps1                                         # Full: stepwise"
Write-Host "  .\run_mlr.ps1 -Mode end-to-end                         # Full: end-to-end"
Write-Host "  .\run_mlr.ps1 -Model claude-sonnet-4                   # Different judge"

exit $(if ($evalOk) { 0 } else { 1 })
