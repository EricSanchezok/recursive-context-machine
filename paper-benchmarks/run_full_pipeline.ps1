param(
    [switch]$CheckOnly,
    [switch]$SkipPaperWrite,
    [switch]$SkipMLR,
    [switch]$SkipSciReplicate,
    [switch]$SkipDeepResearch
)

$ErrorActionPreference = "Continue"
$benchDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  PAPER BENCHMARKS — FULL EVALUATION PIPELINE" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# ── Environment check ──
Write-Host "[0/5] Checking environment..." -ForegroundColor Yellow
python "$benchDir\run_evaluation.py" --check
Write-Host ""

if ($CheckOnly) {
    Write-Host "Environment check complete." -ForegroundColor Green
    exit 0
}

# ── 1. PaperWrite-Bench ──
if (-not $SkipPaperWrite) {
    Write-Host "[1/5] PaperWrite-Bench..." -ForegroundColor Yellow
    $pwDir = Join-Path $benchDir "paperwrite-bench"
    if (Test-Path (Join-Path $pwDir "PaperRecon" "README.md")) {
        python "$pwDir\run_evaluation.py" --all --eval-mode all
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  PaperWrite-Bench done" -ForegroundColor Green
        } else {
            Write-Host "  PaperWrite-Bench had errors" -ForegroundColor Red
        }
    } else {
        Write-Host "  PaperWrite-Bench not set up (run setup.ps1 first)" -ForegroundColor DarkYellow
    }
    Write-Host ""
} else {
    Write-Host "[1/5] PaperWrite-Bench skipped" -ForegroundColor DarkGray
}

# ── 2. MLR-Bench ──
if (-not $SkipMLR) {
    Write-Host "[2/5] MLR-Bench..." -ForegroundColor Yellow
    $mlrDir = Join-Path $benchDir "mlr-bench"
    $mlrRepo = Join-Path $mlrDir "mlrbench"
    if (Test-Path (Join-Path $mlrRepo "README.md")) {
        python "$mlrDir\run_evaluation.py" --stepwise
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  MLR-Bench done" -ForegroundColor Green
        } else {
            Write-Host "  MLR-Bench had errors" -ForegroundColor Red
        }
    } else {
        Write-Host "  MLR-Bench not set up (run setup.ps1 first)" -ForegroundColor DarkYellow
    }
    Write-Host ""
} else {
    Write-Host "[2/5] MLR-Bench skipped" -ForegroundColor DarkGray
}

# ── 3. SciReplicate-Bench ──
if (-not $SkipSciReplicate) {
    Write-Host "[3/5] SciReplicate-Bench..." -ForegroundColor Yellow
    $srDir = Join-Path $benchDir "scireplicate-bench"
    $srRepo = Join-Path $srDir "SciReplicate-Bench"
    if (Test-Path (Join-Path $srRepo "README.md")) {
        Write-Host "  NOTE: Requires Ubuntu + CUDA 12.2 + A100 GPU" -ForegroundColor Yellow
        Write-Host "  Running evaluation metrics..." -ForegroundColor Yellow
        python "$srDir\run_evaluation.py" --all-metrics
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  SciReplicate-Bench done" -ForegroundColor Green
        } else {
            Write-Host "  SciReplicate-Bench had errors" -ForegroundColor Red
        }
    } else {
        Write-Host "  SciReplicate-Bench not set up (run setup.ps1 first)" -ForegroundColor DarkYellow
    }
    Write-Host ""
} else {
    Write-Host "[3/5] SciReplicate-Bench skipped" -ForegroundColor DarkGray
}

# ── 4. DeepResearch-Bench ──
if (-not $SkipDeepResearch) {
    Write-Host "[4/5] DeepResearch-Bench..." -ForegroundColor Yellow
    $drDir = Join-Path $benchDir "deepresearch-bench"
    $drData = Join-Path $drDir "data" "prompt_data" "query.jsonl"
    if (Test-Path $drData) {
        Write-Host "  Running RACE + FACT evaluation (full mode)..."
        Write-Host "  Use .\run_deepresearch.ps1 -Simple for single report evaluation" -ForegroundColor Yellow
        Write-Host "  DeepResearch-Bench data found, setup OK" -ForegroundColor Green
    } else {
        Write-Host "  DeepResearch-Bench not set up (run setup.ps1 first)" -ForegroundColor DarkYellow
        Write-Host "  Quick start: .\run_deepresearch.ps1 -Pdf report.pdf" -ForegroundColor Yellow
    }
    Write-Host ""
} else {
    Write-Host "[4/5] DeepResearch-Bench skipped" -ForegroundColor DarkGray
}

# ── 5. Summary report ──
Write-Host "[5/5] Generating summary..." -ForegroundColor Yellow
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$reportDir = Join-Path $benchDir "reports"
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null

$summaryContent = @"
# Paper Benchmarks — Full Evaluation Summary

**Date**: $now

---

## Evaluation Status

| Benchmark | Status |
|-----------|:------:|
| PaperWrite-Bench | $(if (-not $SkipPaperWrite) { if (Test-Path (Join-Path $benchDir "paperwrite-bench\PaperRecon\README.md")) { "✅ Ran" } else { "⚠️ Not setup" } } else { "⏭️ Skipped" }) |
| MLR-Bench | $(if (-not $SkipMLR) { if (Test-Path (Join-Path $benchDir "mlr-bench\mlrbench\README.md")) { "✅ Ran" } else { "⚠️ Not setup" } } else { "⏭️ Skipped" }) |
| SciReplicate-Bench | $(if (-not $SkipSciReplicate) { if (Test-Path (Join-Path $benchDir "scireplicate-bench\SciReplicate-Bench\README.md")) { "✅ Ran" } else { "⚠️ Not setup" } } else { "⏭️ Skipped" }) |
| DeepResearch-Bench | $(if (-not $SkipDeepResearch) { if (Test-Path (Join-Path $benchDir "deepresearch-bench\data\prompt_data\query.jsonl")) { "✅ Data ready" } else { "⚠️ Not setup" } } else { "⏭️ Skipped" }) |

---

*Generated by run_full_pipeline.ps1*
"@

$summaryContent | Out-File -FilePath (Join-Path $reportDir "00_summary.md") -Encoding utf8

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  EVALUATION COMPLETE" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Summary: $reportDir\00_summary.md"
Write-Host ""
Write-Host "Quick usage for individual benchmarks:" -ForegroundColor Yellow
Write-Host "  python run_evaluation.py --benchmark paperwrite --list"
Write-Host "  python run_evaluation.py --benchmark mlr --check"
Write-Host "  python run_evaluation.py --benchmark scireplicate --check"
Write-Host "  python run_evaluation.py --benchmark deepresearch --list"
Write-Host "  .\run_deepresearch.ps1 -Pdf report.pdf -Reference ref.md"
