# Unified SurveyBench Evaluation Suite
# Evaluates a generated survey using multiple benchmarks.
# All results saved to <runDir>/eval_reports/
#
# Usage:
#   .\run_multi_bench_eval.ps1                          # Self-judge (DeepSeek)
#   .\run_multi_bench_eval.ps1 -CrossJudge              # Cross-judge (Claude/GPT)
#   .\run_multi_bench_eval.ps1 -CrossJudge -Benchmarks @('survey_bench','survey_lens')

param(
    [string]$RunDir = "",
    [switch]$CrossJudge = $false,
    [string[]]$Benchmarks = @("survey_bench","deepsurvey_bench","survey_lens","survey_eval","sgsim_eval","survey_scope")
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$accelerator = Join-Path $scriptDir "..\..\target\release\accelerate.exe"
$env:DEEPSEEK_API_KEY = "REDACTED"

# ── Benchmarks ──
$benchConfig = @{
    survey_bench      = @{name="SurveyBench"}
    deepsurvey_bench  = @{name="DeepSurvey-Bench"}
    survey_lens       = @{name="SurveyLens"}
    survey_eval       = @{name="SurveyEval"}
    sgsim_eval        = @{name="SGSimEval"}
    survey_scope      = @{name="SurveyScope"}
}

# ── Model selection ──
$judgeLabel = "Self-Judge (DeepSeek)"
if ($CrossJudge) {
    $judgeLabel = "Cross-Judge (gpt-5.5 via gmncode.com)"
    if (-not $env:CROSS_JUDGE_API_KEY) {
        $env:CROSS_JUDGE_API_KEY = Read-Host "Enter OpenAI API key"
    }
    Write-Host "Cross-judge: gpt-5.5 via gmncode.com" -ForegroundColor Magenta
}

# Determine run directory
if (-not $RunDir) {
    $runsDir = Join-Path $scriptDir "runs"
    $latestRun = Get-ChildItem -Path $runsDir -Directory | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $latestRun) { Write-Error "No runs found."; exit 1 }
    $RunDir = $latestRun.FullName
}

$surveyFile = Join-Path $RunDir "07_survey.md"
if (-not (Test-Path $surveyFile)) { Write-Error "Survey not found at $surveyFile"; exit 1 }

# Create eval reports directory
$reportDir = Join-Path $RunDir "eval_reports"
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null

$suffix = if ($CrossJudge) { "_cross" } else { "" }

# For cross-judge: inject API key into RCM files (in-place, then restore)
$tempRcmDir = $null
$crossRcmBackup = @{}
if ($CrossJudge) {
    $crossDir = Join-Path $scriptDir "rcm\eval_cross"
    Get-ChildItem "$crossDir\*.rcm" | ForEach-Object {
        $original = Get-Content $_.FullName -Raw
        $crossRcmBackup[$_.Name] = $original
        $modified = $original -replace '__CROSS_KEY__', $env:CROSS_JUDGE_API_KEY
        [System.IO.File]::WriteAllText($_.FullName, $modified, [System.Text.UTF8Encoding]::new($false))
    }
    Write-Host "Cross-judge API key injected into RCM files" -ForegroundColor Magenta
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Multi-Benchmark Evaluation" -ForegroundColor Cyan
Write-Host "  Judge mode: $judgeLabel" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Survey:    $surveyFile"
Write-Host "Output:    $reportDir"
Write-Host "Benchmarks: $($Benchmarks -join ', ')"
Write-Host ""

$originalCwd = Get-Location
$results = @()

try {
    foreach ($benchKey in $Benchmarks) {
        $cfg = $benchConfig[$benchKey]
        if (-not $cfg) { Write-Warning "Unknown benchmark: $benchKey"; continue }

        # Build RCM path based on judge mode
        $benchName = if ($CrossJudge) { "Cross-$($cfg.name)" } else { $cfg.name }
        if ($CrossJudge) {
            $rcmPath = Join-Path $scriptDir "rcm\eval_cross\$benchKey.rcm"
        } elseif ($benchKey -eq "survey_bench") {
            $rcmPath = Join-Path $scriptDir "rcm\surveybench_eval.rcm"
        } else {
            $rcmPath = Join-Path $scriptDir "rcm\eval\$benchKey.rcm"
        }

        if (-not (Test-Path $rcmPath)) {
            Write-Warning "RCM file not found: $rcmPath"
            continue
        }

        Write-Host "────────────────────────────────────" -ForegroundColor Yellow
        Write-Host "Running: $benchName" -ForegroundColor Yellow
        Write-Host "────────────────────────────────────" -ForegroundColor Yellow

        Set-Location $RunDir

        $timer = [System.Diagnostics.Stopwatch]::StartNew()
        try {
            $output = & $accelerator run $rcmPath 2>&1
            $timer.Stop()

            # Save
            $reportFile = Join-Path $reportDir "$benchKey$suffix.md"
            $output | Out-File -FilePath $reportFile -Encoding utf8

            # Extract score
            $score = "N/A"
            $text = $output -join [string]::Concat([char]13, [char]10)
            $m = [regex]::Match($text, '\*\*Total\*\*.*?\*{0,2}([0-9]\.[0-9]{2,})')
            if (-not $m.Success) { $m = [regex]::Match($text, '\*\*Overall\*\*.*?\*{0,2}([0-9]\.[0-9]{2,})') }
            if (-not $m.Success) { $m = [regex]::Match($text, '([0-9]\.[0-9]{2,})') }
            if ($m.Success) { $score = $m.Groups[1].Value }

            Write-Host "[OK] $benchName complete ($([math]::Round($timer.Elapsed.TotalMinutes,1)) min) Score: $score" -ForegroundColor Green

            $results += [PSCustomObject]@{
                Benchmark = $benchName
                Key       = $benchKey
                Score     = $score
                TimeMin   = [math]::Round($timer.Elapsed.TotalMinutes, 1)
                Report    = $reportFile
            }
        }
        catch {
            $timer.Stop()
            Write-Host "[FAIL] $benchName : $_" -ForegroundColor Red
            $results += [PSCustomObject]@{
                Benchmark = $benchName
                Key       = $benchKey
                Score     = "ERROR"
                TimeMin   = [math]::Round($timer.Elapsed.TotalMinutes, 1)
                Report    = ""
            }
        }
    }

    # ── Write summary ──
    Set-Location $originalCwd
    $summaryFile = if ($CrossJudge) { "00_summary_cross.md" } else { "00_summary.md" }
    $summaryPath = Join-Path $reportDir $summaryFile

    $lines = @()
    $lines += "# Multi-Benchmark Evaluation Summary"
    $lines += ""
    $lines += "**Survey**: $surveyFile"
    $lines += "**Run directory**: $RunDir"
    $lines += "**Judge model**: $judgeLabel"
    $lines += "**Date**: $(Get-Date -Format 'yyyy-MM-dd HH:mm UTC')"
    $lines += ""
    $lines += "## Score Summary"
    $lines += ""
    $lines += "| Benchmark | Score | Time (min) | Report |"
    $lines += "|-----------|:-----:|:----------:|--------|"

    foreach ($r in $results) {
        if ($r.Report) {
            $lines += "| $($r.Benchmark) | $($r.Score) | $($r.TimeMin) | [link]($($r.Key)$suffix.md) |"
        } else {
            $lines += "| $($r.Benchmark) | $($r.Score) | $($r.TimeMin) | - |"
        }
    }

    # If cross-judge, try to add comparison with self-judge
    if ($CrossJudge) {
        $selfSummary = Join-Path $reportDir "00_summary.md"
        if (Test-Path $selfSummary) {
            $lines += ""
            $lines += "---"
            $lines += ""
            $lines += "## Cross-Judge vs. Self-Judge Comparison"
            $lines += ""
            $lines += "| Benchmark | Self-Judge (DeepSeek) | Cross-Judge | Difference |"
            $lines += "|-----------|:--------------------:|:-----------:|:----------:|"

            foreach ($r in $results) {
                $selfScore = "N/A"
                # r.Benchmark is "Cross-SurveyBench", strip "Cross-" to match self-judge table
                $selfName = $r.Benchmark -replace '^Cross-', ''
                $selfMatch = Select-String -Path $selfSummary -Pattern "\| $selfName \| ([0-9]\.[0-9]+)" | Select-Object -First 1
                if ($selfMatch) { $selfScore = $selfMatch.Matches.Groups[1].Value }

                $diff = if ($selfScore -ne "N/A" -and $r.Score -ne "N/A" -and $r.Score -ne "ERROR") {
                    $d = [double]$r.Score - [double]$selfScore
                    if ($d -ge 0) { "+$($d.ToString('F2'))" } else { $d.ToString('F2') }
                } else { "-" }

                $lines += "| $($r.Benchmark) | $selfScore | $($r.Score) | $diff |"
            }

            $lines += ""
            $lines += "**Interpretation**: Positive difference = cross-judge rated higher than self-judge,"
            $lines += "Negative = self-judge rated higher. Large gaps (>0.5) suggest self-enhancement bias."
        }
    }

    $lines += ""
    $lines += "---"
    $lines += ""
    $lines += "### Notes"
    $lines += ""
    $lines += "- Dims vary per benchmark (see individual reports)"
    $lines += "- All scores out of 5.0"

    [System.IO.File]::WriteAllText($summaryPath, ($lines -join [Environment]::NewLine), [System.Text.UTF8Encoding]::new($false))

    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Complete" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "Summary: $summaryPath"
    Write-Host ""

    Write-Host "Score Summary:" -ForegroundColor Cyan
    Write-Host ("{0,-30} {1,8} {2,12}" -f "Benchmark", "Score", "Time(min)")
    Write-Host ("-" * 52)
    foreach ($r in $results) {
        Write-Host ("{0,-30} {1,8} {2,12}" -f $r.Benchmark, $r.Score, $r.TimeMin)
    }
}
finally {
    Set-Location $originalCwd
    # Restore original cross-judge RCM files
    if ($crossRcmBackup.Count -gt 0) {
        $crossDir = Join-Path $scriptDir "rcm\eval_cross"
        foreach ($entry in $crossRcmBackup.GetEnumerator()) {
            $restorePath = Join-Path $crossDir $entry.Key
            [System.IO.File]::WriteAllText($restorePath, $entry.Value, [System.Text.UTF8Encoding]::new($false))
        }
    }
}
