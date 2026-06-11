param(
    [switch]$SkipClone,
    [switch]$DockerSetup
)

$ErrorActionPreference = "Continue"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoDir = Join-Path $scriptDir "PaperRecon"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  PaperWrite-Bench (PaperRecon) Setup" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

# ── 1. Clone official repo ──
if (-not $SkipClone) {
    if (Test-Path $repoDir) {
        Write-Host "[1/3] Repo already exists: $repoDir" -ForegroundColor Yellow
    }
    else {
        Write-Host "[1/3] Cloning official PaperRecon repo..." -ForegroundColor Yellow
        git clone https://github.com/Agent4Science-UTokyo/PaperRecon.git $repoDir
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

# ── 2. Install Python dependencies ──
Write-Host "[2/3] Installing Python dependencies..." -ForegroundColor Yellow
$reqFile = Join-Path $repoDir "requirements.txt"
if (Test-Path $reqFile) {
    pip install -r $reqFile 2>&1 | Out-Null
    Write-Host "  OK" -ForegroundColor Green
} else {
    $setupFile = Join-Path $repoDir "setup.py"
    $pyprojFile = Join-Path $repoDir "pyproject.toml"
    if (Test-Path $setupFile) {
        pip install -e $repoDir 2>&1 | Out-Null
        Write-Host "  OK (editable install)" -ForegroundColor Green
    } elseif (Test-Path $pyprojFile) {
        pip install -e $repoDir 2>&1 | Out-Null
        Write-Host "  OK (pyproject install)" -ForegroundColor Green
    } else {
        Write-Host "  No requirements found - manual install may be needed" -ForegroundColor Yellow
    }
}

# ── 3. Docker setup (optional) ──
if ($DockerSetup) {
    Write-Host "[3/3] Setting up Docker..." -ForegroundColor Yellow
    Push-Location $repoDir
    try {
        docker compose build 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Docker build OK" -ForegroundColor Green
        } else {
            Write-Host "  Docker build failed (may need Linux)" -ForegroundColor Red
        }
    }
    finally {
        Pop-Location
    }
} else {
    Write-Host "[3/3] Skipped Docker setup (use -DockerSetup flag)" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "PaperWrite-Bench setup complete!" -ForegroundColor Cyan
Write-Host "Repo: $repoDir"
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Copy your API keys to .env file"
Write-Host "  2. See official docs: $repoDir\README.md"
