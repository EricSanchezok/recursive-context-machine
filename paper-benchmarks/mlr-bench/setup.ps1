param(
    [switch]$SkipClone
)

$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoDir = Join-Path $scriptDir "mlrbench"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  MLR-Bench Setup" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

# ── 1. Clone official repo ──
if (-not $SkipClone) {
    if (Test-Path $repoDir) {
        Write-Host "[1/2] Repo already exists: $repoDir" -ForegroundColor Yellow
    }
    else {
        Write-Host "[1/2] Cloning official MLR-Bench repo..." -ForegroundColor Yellow
        git clone https://github.com/chchenhui/mlrbench.git $repoDir
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  OK" -ForegroundColor Green
        } else {
            Write-Host "  FAILED" -ForegroundColor Red
            exit 1
        }
    }
} else {
    Write-Host "[1/2] Skipped clone" -ForegroundColor DarkGray
}

# ── 2. Install dependencies ──
Write-Host "[2/2] Installing dependencies..." -ForegroundColor Yellow
Push-Location $repoDir
try {
    # Check for requirements.txt or pyproject.toml
    if (Test-Path "requirements.txt") {
        pip install -r requirements.txt 2>&1 | Out-Null
        Write-Host "  requirements.txt installed" -ForegroundColor Green
    }
    if (Test-Path "pyproject.toml") {
        pip install -e . 2>&1 | Out-Null
        Write-Host "  Package installed (editable)" -ForegroundColor Green
    }
} finally {
    Pop-Location
}

Write-Host ""
Write-Host "MLR-Bench setup complete!" -ForegroundColor Cyan
Write-Host "Repo: $repoDir"
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Set API keys in environment:"
Write-Host "     OPENAI_API_KEY, ANTHROPIC_API_KEY, OPENROUTER_API_KEY"
Write-Host "  2. Run huggingface-cli login for dataset access"
Write-Host "  3. See official docs: $repoDir\README.md"
