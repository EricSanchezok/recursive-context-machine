# MASA Automated Survey Pipeline
# One-shot execution: creates run dir, runs Phase 0 鈫?multi-round Core 鈫?Finish
# All output goes to MASA v2.0/runs/<timestamp>/

param(
    [int]$MaxRounds = 3
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# 鈹€鈹€ Paths 鈹€鈹€
$phase0Rcm = Join-Path $scriptDir "rcm\masa_phase0.rcm"
$coreRcm    = Join-Path $scriptDir "rcm\masa_core.rcm"
$finishRcm  = Join-Path $scriptDir "rcm\masa_finish.rcm"
$accelerator = Join-Path $scriptDir "..\..\target\release\accelerate.exe"

# 鈹€鈹€ API Key (must be set in environment, not hardcoded) 鈹€鈹€
if (-not $env:DEEPSEEK_API_KEY) {
    Write-Error "DEEPSEEK_API_KEY is not set. Please set it before running: `$env:DEEPSEEK_API_KEY='sk-...'"
    exit 1
}

# 鈹€鈹€ Create fixed run directory 鈹€鈹€
$timestamp = Get-Date -Format "yyyy-MM-ddTHHmmZ"
$runDir = Join-Path $scriptDir "runs\$timestamp"
New-Item -ItemType Directory -Path $runDir -Force | Out-Null

# Copy schema files into run dir so agents can read them
$schemaDir = Join-Path $scriptDir "schema"
$runSchemaDir = Join-Path $runDir "schema"
Copy-Item -Path "$schemaDir\*" -Destination $runSchemaDir -Recurse -Force

Write-Host "=== MASA Survey Pipeline ===" -ForegroundColor Cyan
Write-Host "Run Directory: $runDir"
Write-Host "Max Rounds: $MaxRounds"
Write-Host ""

$originalCwd = Get-Location
Set-Location $runDir

try {
    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    # Phase 0: Paper Discovery (run once)
    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    Write-Host "=== Phase 0: Paper Discovery ===" -ForegroundColor Yellow
    $output = & "$accelerator" run "$phase0Rcm" 2>&1
    $output | ForEach-Object { Write-Host $_ }
    $phase0ExitCode = $LASTEXITCODE
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
        "03c_semantic_expansion.md", "03_expansion.md"
    )
    foreach ($file in $phase0Files) {
        $source = Join-Path $runDir $file
        if (Test-Path $source) {
            Move-Item -Path $source -Destination $phase0Dir -Force
        }
    }

    # Initialize memory directory
    $memoryDir = Join-Path $runDir "memory"
    New-Item -ItemType Directory -Path $memoryDir -Force | Out-Null

    # Initialize iteration state
    Set-Content -Path (Join-Path $memoryDir "iteration_state.md") -Value @"
current_round: 1
last_total_score: 0.0
previous_round_score: 0.0
verdict: START
"@

    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    # Core: Multi-Round Iteration with Judge Control
    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    $prevScore = 0.0
    $verdict = "CONTINUE"

    for ($round = 1; $round -le $MaxRounds; $round++) {
        Write-Host "=== Core Round $round ===" -ForegroundColor Yellow

        # Update iteration state
        Set-Content -Path (Join-Path $memoryDir "iteration_state.md") -Value @"
current_round: $round
last_total_score: 0.0
previous_round_score: $prevScore
verdict: $verdict
"@

        $output = & "$accelerator" run "$coreRcm" 2>&1
        $output | ForEach-Object { Write-Host $_ }

        # Read Judge verdict from iteration_state.md (written by synthesizer)
        $iterationStateFile = Join-Path $memoryDir "iteration_state.md"
        if (Test-Path $iterationStateFile) {
            $stateContent = Get-Content $iterationStateFile -Raw
            if ($stateContent -match 'last_total_score:\s*(?<score>\d+\.\d+)') {
                $totalScore = [double]$Matches['score']
            } else {
                $totalScore = 0.0
            }
            if ($stateContent -match 'verdict:\s*(?<verdict>\S+)') {
                $verdict = $Matches['verdict']
            } else {
                $verdict = "STOP"
            }
            Write-Host "Judge: Score=$totalScore, Verdict=$verdict" -ForegroundColor Cyan
        } else {
            Write-Warning "iteration_state.md not found"
            $verdict = "STOP"
        }

        Write-Host "Round $round complete." -ForegroundColor Green
        Write-Host ""

        # Log retention: trim overgrown files after each round
        $retentionRules = @{
            "memory/agent_researcher.md"  = @{ MaxLines = 200; KeepLines = 50 }
            "memory/agent_generator.md"   = @{ MaxLines = 300; KeepLines = 80 }
            "memory/section_summaries.md" = @{ MaxLines = 200; KeepLines = 50 }
            "memory/supervisor_notes.md"  = @{ MaxLines = 300; KeepLines = 100 }
        }
        foreach ($relPath in $retentionRules.Keys) {
            $absPath = Join-Path $runDir $relPath
            if (-not (Test-Path $absPath)) { continue }
            $lines = Get-Content $absPath
            $rule = $retentionRules[$relPath]
            if ($lines.Count -gt $rule.MaxLines) {
                $lines[-$rule.KeepLines..-1] | Set-Content $absPath
                Write-Host "  Trimmed $relPath ($($lines.Count) → $($rule.KeepLines) lines)" -ForegroundColor DarkGray
            }
        }

        # Termination checks
        if ($verdict -eq "STOP") {
            Write-Host "Judge verdict is STOP. Ending iteration." -ForegroundColor Green
            break
        }
        $prevScore = $totalScore
    }

    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    # Phase Finish: Polishing
    # 鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺愨晲鈺?    Write-Host "=== Phase Finish: Polishing ===" -ForegroundColor Yellow
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

# 鈹€鈹€ Summary 鈹€鈹€
Write-Host "=== MASA Pipeline Complete ===" -ForegroundColor Green
Write-Host "Final survey: $runDir\07_survey.md" -ForegroundColor Cyan
Write-Host "Run directory: $runDir" -ForegroundColor Gray


