param(
    [switch]$SkipClone
)

$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoDir = Join-Path $scriptDir "SciReplicate-Bench"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  SciReplicate-Bench Setup" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

# ── 1. Clone official repo ──
if (-not $SkipClone) {
    if (Test-Path $repoDir) {
        Write-Host "[1/3] Repo already exists: $repoDir" -ForegroundColor Yellow
    }
    else {
        Write-Host "[1/3] Cloning official SciReplicate-Bench repo..." -ForegroundColor Yellow
        git clone https://github.com/xyzCS/SciReplicate-Bench.git $repoDir
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  OK" -ForegroundColor Green
        } else {
            Write-Host "  FAILED" -ForegroundColor Red
            exit 1
        }
    }
} else {
    Write-Host "[1/3] Skipped clone" -ForegroundColor DarkGray
}

# ── 2. Install SciReproducer dependencies ──
Write-Host "[2/3] Installing SciReproducer dependencies..." -ForegroundColor Yellow
$envFile = Join-Path $repoDir "environment.yml"
if (Test-Path $envFile) {
    Write-Host "  Conda environment file found: environment.yml"
    Write-Host "  Run manually: conda env create -f $envFile" -ForegroundColor Yellow
} else {
    $reqFile = Join-Path $repoDir "requirements.txt"
    if (Test-Path $reqFile) {
        pip install -r $reqFile 2>&1 | Out-Null
        Write-Host "  Installed from requirements.txt" -ForegroundColor Green
    }
}

# ── 3. Download benchmark data ──
Write-Host "[3/3] Benchmark data..." -ForegroundColor Yellow
Write-Host "  Official data available at:"
Write-Host "  - envs_sci.zip: see repo README for SharePoint link"
Write-Host "  - Benchmark/: see repo README for download link"
Write-Host "  Follow official instructions in $repoDir\README.md" -ForegroundColor Yellow

Write-Host ""
Write-Host "SciReplicate-Bench setup complete!" -ForegroundColor Cyan
Write-Host "Repo: $repoDir"
Write-Host ""
Write-Host "IMPORTANT:" -ForegroundColor Red
Write-Host "This benchmark requires Ubuntu + CUDA 12.2 + A100 GPU to run fully."
Write-Host "See official docs: $repoDir\README.md"
