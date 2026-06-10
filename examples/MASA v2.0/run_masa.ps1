# MASA Automated Survey Pipeline (Single-Pass)
# One-shot execution: creates run dir, runs Phase 0 鈫?Core 鈫?Revise 鈫?Finish
# All output goes to MASA v2.0/runs/<timestamp>/

param(
    [switch]$SkipPhase0
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# 钬┏ Paths 钬┏
$phase0Rcm   = Join-Path $scriptDir "rcm\masa_phase0.rcm"
$coreRcm     = Join-Path $scriptDir "rcm\masa_core.rcm"
$reviseRcm   = Join-Path $scriptDir "rcm\masa_revise.rcm"
$finishRcm   = Join-Path $scriptDir "rcm\masa_finish.rcm"
$accelerator = Join-Path $scriptDir "..\..\target\release\accelerate.exe"

# 钬┏ API Key (must be set in environment, not hardcoded) 钬┏
if (-not $env:DEEPSEEK_API_KEY) {
    Write-Error "DEEPSEEK_API_KEY is not set. Please set it before running: `$env:DEEPSEEK_API_KEY='sk-...'"
    exit 1
}

# 钬┏ Create fixed run directory 钬┏
$timestamp = Get-Date -Format "yyyy-MM-ddTHHmmZ"
$runDir = Join-Path $scriptDir "runs\$timestamp"
New-Item -ItemType Directory -Path $runDir -Force | Out-Null

# Copy schema files into run dir so agents can read them
$schemaDir = Join-Path $scriptDir "schema"
$runSchemaDir = Join-Path $runDir "schema"
if (Test-Path $schemaDir) {
    Copy-Item -Path "$schemaDir\*" -Destination $runSchemaDir -Recurse -Force
}

Write-Host "=== MASA Survey Pipeline (Single-Pass) ===" -ForegroundColor Cyan
Write-Host "Run Directory: $runDir"
Write-Host ""

$originalCwd = Get-Location
Set-Location $runDir

try {
    # -------------------------------------------------------------------------
    # Phase 0: Paper Discovery (run once)
    # -------------------------------------------------------------------------
    if (-not $SkipPhase0) {
        Write-Host "=== Phase 0: Paper Discovery ===" -ForegroundColor Yellow
        $output = & "$accelerator" run "$phase0Rcm" 2>&1
        $output | ForEach-Object { Write-Host $_ }
        Write-Host "Phase 0 complete." -ForegroundColor Green
        Write-Host ""

        # Organize Phase 0 output into phase0/ subdirectory
        $phase0Dir = Join-Path $runDir "phase0"
        New-Item -ItemType Directory -Path $phase0Dir -Force | Out-Null
        $phase0Files = @(
            "00_survey_spec.md", "01_query_plan.md", "02_candidate_pool.md",
            "02a_method_candidates.md", "02b_benchmark_candidates.md",
            "02c_survey_candidates.md", "02d_frontier_candidates.md",
            "03a_seed_papers.md", "03b_citation_expansion.md",
            "03c_semantic_expansion.md", "03_expansion.md",
            "paper_fetch_report.md"
        )
        foreach ($file in $phase0Files) {
            $source = Join-Path $runDir $file
            if (Test-Path $source) {
                Move-Item -Path $source -Destination $phase0Dir -Force
            }
        }
    } else {
        Write-Host "=== Skipping Phase 0 (-SkipPhase0) ===" -ForegroundColor Yellow
    }

    # Initialize memory directory
    $memoryDir = Join-Path $runDir "memory"
    New-Item -ItemType Directory -Path $memoryDir -Force | Out-Null

    # -------------------------------------------------------------------------
    # Core: Researcher 鈫?Generator 鈫?Supervisor 鈫?4 Judges 鈫?Synthesizer
    # -------------------------------------------------------------------------
    Write-Host "=== Core: Full Pipeline ===" -ForegroundColor Yellow
    $output = & "$accelerator" run "$coreRcm" 2>&1
    $output | ForEach-Object { Write-Host $_ }
    Write-Host "Core complete." -ForegroundColor Green
    Write-Host ""

    # -------------------------------------------------------------------------
    # Revise: Generator applies supervisor + judge feedback (focused rewrite)
    # -------------------------------------------------------------------------
    Write-Host "=== Revision: Generator Applies Feedback ===" -ForegroundColor Yellow
    $output = $runDir | & "$accelerator" run "$reviseRcm" 2>&1
    $output | ForEach-Object { Write-Host $_ }
    Write-Host "Revision complete." -ForegroundColor Green
    Write-Host ""

    # -------------------------------------------------------------------------
    # Phase Finish: Polisher → Reference Compiler
    # -------------------------------------------------------------------------
    Write-Host "=== Phase Finish: Polishing ===" -ForegroundColor Yellow
    $output = & "$accelerator" run "$finishRcm" 2>&1
    $output | ForEach-Object { Write-Host $_ }
    Write-Host "Phase Finish complete." -ForegroundColor Green
    Write-Host ""
}
catch {
    Set-Location $originalCwd
    Write-Error "Pipeline failed: $_"
    exit 1
}

Set-Location $originalCwd

# 钬┏ Summary 钬┏
Write-Host "=== MASA Pipeline Complete ===" -ForegroundColor Green
Write-Host "Final survey: $runDir\07_survey.md" -ForegroundColor Cyan
Write-Host "Run directory: $runDir" -ForegroundColor Gray
