<#
.SYNOPSIS
    DeepResearch-Bench — 环境设置脚本
.DESCRIPTION
    克隆官方 DeepResearch Bench 仓库并安装依赖。
.PARAMETER SkipClone
    跳过克隆官方仓库（如果已有）
.PARAMETER Help
    显示帮助
#>

param(
    [switch]$SkipClone,
    [switch]$Help
)

if ($Help) {
    Get-Help $PSCommandPath -Detailed
    exit 0
}

$ScriptDir = Split-Path -Parent $PSCommandPath
Set-Location $ScriptDir

Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║     DeepResearch-Bench Setup             ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ── Step 1: Check Python ──────────────────────────────
Write-Host "[1/4] Checking Python..." -ForegroundColor Yellow
$pyVersion = python --version 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[!] Python not found. Please install Python 3.10+" -ForegroundColor Red
    exit 1
}
Write-Host "  Found: $pyVersion"

# ── Step 2: Install Python dependencies ───────────────
Write-Host "[2/4] Installing dependencies..." -ForegroundColor Yellow
$reqFile = Join-Path $ScriptDir "requirements.txt"
if (Test-Path $reqFile) {
    pip install -r $reqFile 2>&1 | Out-Null
    Write-Host "  [✓] Dependencies installed"
} else {
    # Install minimal dependencies
    pip install requests tqdm pandas numpy 2>&1 | Out-Null
    Write-Host "  [✓] Basic dependencies installed"
}

# ── Step 3: Clone official repo (optional) ────────────
Write-Host "[3/4] Official repo..." -ForegroundColor Yellow
$dataDir = Join-Path $ScriptDir "data"

if (-not $SkipClone) {
    $repoUrl = "https://github.com/Ayanami0730/deep_research_bench.git"
    $tempDir = Join-Path $ScriptDir "_deep_research_bench_repo"

    if (Test-Path $tempDir) {
        Write-Host "  [!] Temp directory already exists, skipping clone"
    } else {
        Write-Host "  Cloning official DeepResearch Bench repository..."
        git clone $repoUrl $tempDir 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [✓] Repository cloned"
        } else {
            Write-Host "  [!] Clone failed (network issue?). Manual setup needed." -ForegroundColor Yellow
        }
    }

    # Copy data directory if needed
    $repoDataDir = Join-Path $tempDir "data"
    if ((Test-Path $repoDataDir) -and -not (Test-Path $dataDir)) {
        Write-Host "  Copying benchmark data..."
        Copy-Item -Recurse $repoDataDir $dataDir -Force
        Write-Host "  [✓] Benchmark data copied"
    } elseif (-not (Test-Path $dataDir)) {
        Write-Host "  [!] No data found in cloned repo. The official dataset" -ForegroundColor Yellow
        Write-Host "      requires manual download from the Hugging Face page." -ForegroundColor Yellow
    } else {
        Write-Host "  [i] Data directory already exists, skipping copy"
    }
}

if (Test-Path $dataDir) {
    Write-Host "  [✓] Data: $dataDir"
} else {
    Write-Host "  [!] Data not found. Run setup again or manually download." -ForegroundColor Yellow
    Write-Host "      See: https://github.com/Ayanami0730/deep_research_bench" -ForegroundColor Yellow
}

# ── Step 4: Verify API keys ───────────────────────────
Write-Host "[4/4] Checking API keys..." -ForegroundColor Yellow
$keys = @(
    @{Name="OPENAI_API_KEY"; Required=$true}
    @{Name="JINA_API_KEY"; Required=$false}
)

foreach ($key in $keys) {
    $value = [Environment]::GetEnvironmentVariable($key.Name)
    if ($value) {
        $masked = $value.Substring(0, [Math]::Min(8, $value.Length)) + "..."
        Write-Host "  [✓] $($key.Name): $masked"
    } elseif ($key.Required) {
        Write-Host "  [!] $($key.Name) is NOT set (REQUIRED)" -ForegroundColor Red
    } else {
        Write-Host "  [i] $($key.Name) not set (optional)"
    }
}

Write-Host ""
Write-Host "──────────────────────────────────────────" -ForegroundColor Cyan
Write-Host "Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Set required API keys:"
Write-Host "     \$env:OPENAI_API_KEY = 'sk-...'"
Write-Host "  2. Run Simple Mode (single report):"
Write-Host "     .\run_deepresearch.ps1 -Simple -Pdf report.pdf -Reference ref.md"
Write-Host "  3. Run Full Mode (official 100 tasks):"
Write-Host "     .\run_deepresearch.ps1 -ModelName my_model"
Write-Host "──────────────────────────────────────────" -ForegroundColor Cyan
