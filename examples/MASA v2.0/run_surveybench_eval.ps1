# SurveyBench Evaluation — evaluate a generated survey using SurveyBench methodology
# Usage: .\run_surveybench_eval.ps1 [-RunDir <path>]

param(
    [string]$RunDir = ""
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# ── Paths ──
$evalRcm = Join-Path $scriptDir "rcm\surveybench_eval.rcm"
$accelerator = Join-Path $scriptDir "..\..\target\debug\accelerate.exe"

# ── API Key ──
$env:DEEPSEEK_API_KEY = "sk-b9ebba94ad7943faaad0ae877390a5cc"

# ── Determine run directory ──
if (-not $RunDir) {
    $runsDir = Join-Path $scriptDir "runs"
    $latestRun = Get-ChildItem -Path $runsDir -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $latestRun) {
        Write-Error "No runs found. Specify -RunDir or run MASA pipeline first."
        exit 1
    }
    $RunDir = $latestRun.FullName
}

# ── Verify survey exists ──
$surveyFile = Join-Path $RunDir "07_survey.md"
if (-not (Test-Path $surveyFile)) {
    Write-Error "Survey not found at $surveyFile"
    exit 1
}

Write-Host "=== SurveyBench Evaluation ===" -ForegroundColor Cyan
Write-Host "Survey: $surveyFile"
Write-Host "Binary: $accelerator"
Write-Host ""

$originalCwd = Get-Location

try {
    Set-Location $RunDir

    Write-Host "Launching SurveyBench evaluator..." -ForegroundColor Yellow

    & $accelerator run $evalRcm

    Write-Host ""
    Write-Host "=== Evaluation Complete ===" -ForegroundColor Green
}
catch {
    Write-Error "Evaluation failed: $_"
    exit 1
}
finally {
    Set-Location $originalCwd
}
