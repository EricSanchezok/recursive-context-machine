param(
    [Parameter(Mandatory=$true)]
    [string]$SurveyPath,

    [string]$OutputDir = "",

    [string]$Topic = "",

    # API configuration (no hardcoded values)
    # Priority: CLI arg > Environment variable > Error if missing
    [string]$ApiKey = "",

    [string]$Model = "",

    [string]$Endpoint = "",

    [string]$TestSet = "",             # Path to test set JSON (enables benchmark mode)

    [switch]$SkipQuiz,
    [switch]$SkipComparative,
    [switch]$SkipVerifiability,
    [switch]$SkipImportance
)

$ErrorActionPreference = "Continue"
$benchDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# ── Resolve API configuration ──
if (-not $ApiKey) {
    # Try multiple env var names for flexibility
    $envCandidates = @("EVA_API_KEY", "OPENAI_API_KEY", "LLM_API_KEY")
    foreach ($candidate in $envCandidates) {
        if (Get-Item -Path "env:$candidate" -ErrorAction SilentlyContinue) {
            $ApiKey = [Environment]::GetEnvironmentVariable($candidate)
            Write-Host "  Using API key from env:$$candidate" -ForegroundColor DarkGray
            break
        }
    }
}

if (-not $Endpoint) {
    $Endpoint = [Environment]::GetEnvironmentVariable("EVA_ENDPOINT")
    if (-not $Endpoint) {
        $Endpoint = "https://gmncode.com/v1/chat/completions"
        Write-Host "  Using default endpoint: $Endpoint" -ForegroundColor DarkGray
    } else {
        Write-Host "  Using endpoint from env:EVA_ENDPOINT" -ForegroundColor DarkGray
    }
}

if (-not $Model) {
    $Model = [Environment]::GetEnvironmentVariable("EVA_MODEL")
    if (-not $Model) {
        $Model = "gpt-5.5"
        Write-Host "  Using default model: $Model" -ForegroundColor DarkGray
    } else {
        Write-Host "  Using model from env:EVA_MODEL" -ForegroundColor DarkGray
    }
}

$apiUrl = $Endpoint

# ── Validate API key ──
if (-not $ApiKey) {
    Write-Host "FATAL: No API key provided." -ForegroundColor Red
    Write-Host "  Options:" -ForegroundColor Yellow
    Write-Host "  1. Pass -ApiKey 'your-key'" -ForegroundColor Yellow
    Write-Host "  2. Set env var EVA_API_KEY (or OPENAI_API_KEY)" -ForegroundColor Yellow
    Write-Host "  3. Create a .env file with EVA_API_KEY=your-key" -ForegroundColor Yellow
    exit 1
}

# Try load .env if exists
$envFile = Join-Path $benchDir ".env"
if (Test-Path $envFile) {
    Get-Content $envFile | ForEach-Object {
        if ($_ -match '^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.+)$') {
            $key = $matches[1]
            $val = $matches[2].Trim('"', "'")
            if (-not [Environment]::GetEnvironmentVariable($key)) {
                Set-Item -Path "env:$key" -Value $val
            }
        }
    }
}

# ── Load test set (if provided) ──
$TestSetQuizFile = ""
$TestSetReference = ""
$TestSetMatchedTopic = $Topic

if ($TestSet -and (Test-Path $TestSet)) {
    Write-Host "  Loading test set: $TestSet" -ForegroundColor DarkGray
    $testSetData = Get-Content $TestSet -Raw -Encoding utf8 | ConvertFrom-Json
    $matched = $null
    foreach ($t in $testSetData.topics) {
        if ($Topic -and $t.topic -like "*$Topic*") {
            $matched = $t
            break
        }
    }
    if (-not $matched -and -not $Topic) {
        $matched = $testSetData.topics[0]
        $TestSetMatchedTopic = $matched.topic
        Write-Host "  No topic specified, using first: $($matched.topic)" -ForegroundColor DarkGray
    } elseif ($matched) {
        $TestSetMatchedTopic = $matched.topic
        Write-Host "  Matched topic: $($matched.topic)" -ForegroundColor Green
    }

    if ($matched) {
        # Save quiz questions to temp file
        $TestSetQuizFile = Join-Path $OutputDir "_testset_quiz_$($matched.id).json"
        $matched.quiz | ConvertTo-Json -Depth 5 | Out-File $TestSetQuizFile -Encoding utf8
        Write-Host "  Quiz questions: $((@($matched.quiz)).Count) predefined" -ForegroundColor Green

        # Store reference excerpt
        if ($matched.reference_excerpt) {
            $TestSetReference = $matched.reference_excerpt
            $refFile = Join-Path $OutputDir "_testset_reference_$($matched.id).md"
            "# Reference Survey: $($matched.topic)`n`n$($matched.reference_excerpt)" | Out-File $refFile -Encoding utf8
            Write-Host "  Reference excerpt: available" -ForegroundColor Green
        }
    }
} elseif ($TestSet) {
    Write-Host "  Warning: Test set file not found: $TestSet" -ForegroundColor DarkYellow
}

# Override Topic if test set matched
if ($TestSetMatchedTopic) { $Topic = $TestSetMatchedTopic }
# Override QuizFile if test set provided one
if ($TestSetQuizFile -and -not $QuizFile) {
    $QuizFile = $TestSetQuizFile
}

# ── Validate ──
if (-not (Test-Path $SurveyPath)) { Write-Error "Survey not found: $SurveyPath"; exit 1 }

$surveyName = [System.IO.Path]::GetFileNameWithoutExtension($SurveyPath)
if (-not $OutputDir) { $OutputDir = Join-Path $benchDir "reports\${surveyName}_full_eval" }
New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

# Read survey once
$surveyText = Get-Content $SurveyPath -Raw -Encoding utf8
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  SURVEY FULL EVALUATION PIPELINE" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Survey: $SurveyPath"
Write-Host "Size: $($surveyText.Length) chars"
Write-Host "Topic: $(if ($Topic) { $Topic } else { '(auto-detected)' })"
Write-Host "Output: $OutputDir"
Write-Host "Model: $Model"
Write-Host "Endpoint: $($apiUrl -replace '/v1/chat/completions', '...')"
Write-Host "ApiKey: $($ApiKey.Substring(0, [Math]::Min(8, $ApiKey.Length)) + '...')"
Write-Host "TestSet: $(if ($TestSet -and (Test-Path $TestSet)) { Split-Path $TestSet -Leaf } else { '(none)' })"
Write-Host "Quiz: $(if ($QuizFile) { Split-Path $QuizFile -Leaf } else { 'dynamic (exploratory mode)' })"
Write-Host ""

# ── Helper function: call LLM API ──
function Call-LLM {
    param(
        [string]$PromptText,
        [string]$OutputFile,
        [int]$MaxTokens = 16384,
        [string]$ExtraContext = ""
    )

    $userMsg = @"
$PromptText

## Survey Content to Evaluate (READ THIS - NO FILE ACCESS NEEDED)

$ExtraContext

The survey content is below. Read it and provide your evaluation with scores.

````markdown
$surveyText
```

Please provide your complete evaluation with scores.
"@

    $body = @{
        model = $Model
        messages = @(@{role = "user"; content = $userMsg})
        temperature = 0.0
        max_tokens = $MaxTokens
    } | ConvertTo-Json -Depth 5

    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Method Post `
            -Headers @{Authorization = "Bearer $ApiKey"} `
            -Body $body -ContentType "application/json" -UseBasicParsing

        $resultText = $response.choices[0].message.content
        if ($OutputFile) {
            $resultText | Out-File -FilePath $OutputFile -Encoding utf8
        }
        return $resultText
    }
    catch {
        Write-Host "  LLM API Error: $_" -ForegroundColor Red
        return "LLM_API_ERROR: $_"
    }
}

function Call-LLM-Messages {
    param(
        [array]$Messages,
        [string]$OutputFile,
        [int]$MaxTokens = 16384
    )

    $body = @{
        model = $Model
        messages = $Messages
        temperature = 0.0
        max_tokens = $MaxTokens
    } | ConvertTo-Json -Depth 5

    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Method Post `
            -Headers @{Authorization = "Bearer $ApiKey"} `
            -Body $body -ContentType "application/json" -UseBasicParsing

        $resultText = $response.choices[0].message.content
        if ($OutputFile) {
            $resultText | Out-File -FilePath $OutputFile -Encoding utf8
        }
        return $resultText
    }
    catch {
        Write-Host "  LLM API Error: $_" -ForegroundColor Red
        return "LLM_API_ERROR: $_"
    }
}

# ── Score extraction helper ──
function Extract-Scores {
    param([string]$Text)
    $scores = @{}
    $patterns = @(
        '(?:Score|Total|Overall|Rating)[:\s]*(\d+(?:\.\d+)?)\s*/\s*(\d+(?:\.\d+)?)',
        '(\d+(?:\.\d+)?)\s*/\s*5',
        '(\d+(?:\.\d+)?)\s*/\s*100',
        '(\d+(?:\.\d+)?)%'
    )
    foreach ($p in $patterns) {
        $m = [regex]::Matches($Text, $p)
        foreach ($match in $m) {
            $scores["score$($scores.Count)"] = $match.Groups[1].Value
        }
    }
    return $scores
}

# ────────────────────────────────────────────────
# 1. SURVEYBENCH EVALUATION
# ────────────────────────────────────────────────
Write-Host "`n[1/7] SurveyBench (Content-based)..." -ForegroundColor Yellow -NoNewline
$promptFile = Join-Path $benchDir "surveybench/prompt.txt"
$promptText = Get-Content $promptFile -Raw -Encoding utf8
$idx = $promptText.IndexOf("---`n")
if ($idx -ge 0 -and $idx -le 100) { $promptText = $promptText.Substring($idx + 4) }

$reportFile = Join-Path $OutputDir "01_surveybench_report.md"
$report = Call-LLM -PromptText $promptText -OutputFile $reportFile
if ($report -notlike "LLM_API_ERROR*") { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }

# ────────────────────────────────────────────────
# 2. DEEPSURVEY-BENCH EVALUATION
# ────────────────────────────────────────────────
Write-Host "[2/7] DeepSurvey-Bench..." -ForegroundColor Yellow -NoNewline
$promptFile = Join-Path $benchDir "deepsurvey-bench/prompt.txt"
$promptText = Get-Content $promptFile -Raw -Encoding utf8
$idx = $promptText.IndexOf("---`n")
if ($idx -ge 0 -and $idx -le 100) { $promptText = $promptText.Substring($idx + 4) }

$reportFile = Join-Path $OutputDir "02_deepsurvey_report.md"
$report = Call-LLM -PromptText $promptText -OutputFile $reportFile
if ($report -notlike "LLM_API_ERROR*") { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }

# ────────────────────────────────────────────────
# 3. DEEPSCHOLAR-BENCH EVALUATION
# ────────────────────────────────────────────────
Write-Host "[3/7] DeepScholar-Bench..." -ForegroundColor Yellow -NoNewline
$promptFile = Join-Path $benchDir "deepscholar-bench/prompt.txt"
$promptText = Get-Content $promptFile -Raw -Encoding utf8
$idx = $promptText.IndexOf("---`n")
if ($idx -ge 0 -and $idx -le 100) { $promptText = $promptText.Substring($idx + 4) }

$reportFile = Join-Path $OutputDir "03_deepscholar_report.md"
$report = Call-LLM -PromptText $promptText -OutputFile $reportFile
if ($report -notlike "LLM_API_ERROR*") { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }

# ────────────────────────────────────────────────
# 4. QUIZ EVALUATION (via Python QuizEvaluator)
# ────────────────────────────────────────────────
Write-Host "[4/7] Quiz Evaluation..." -ForegroundColor Yellow

if (-not $SkipQuiz -and $Topic) {
    $surveyPathPy = $SurveyPath -replace '\\', '/'
    $outputDirPy = (Join-Path $OutputDir "04_quiz_report.md") -replace '\\', '/'
    $benchDirPy = $benchDir -replace '\\', '/'

    if ($QuizFile) {
        # Benchmark mode: use predefined quiz questions
        $quizFilePy = $QuizFile -replace '\\', '/'
        Write-Host "  Using predefined quiz: $(Split-Path $QuizFile -Leaf)" -ForegroundColor Cyan
        & python -c @"
import sys
sys.path.insert(0, '$benchDirPy')
from evaluators.quiz_evaluator import QuizEvaluator
import json
survey = open('$surveyPathPy', encoding='utf-8').read()
quiz = json.load(open('$quizFilePy', encoding='utf-8'))
e = QuizEvaluator()
report = e.generate_report(survey, '$Topic', predefined_quiz=quiz)
open('$outputDirPy', 'w', encoding='utf-8').write(report)
print('Quiz evaluation complete (benchmark mode)')
"@ 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Green }
    } else {
        # Exploratory mode: dynamic quiz generation
        & python -c @"
import sys
sys.path.insert(0, '$benchDirPy')
from evaluators.quiz_evaluator import QuizEvaluator
survey = open('$surveyPathPy', encoding='utf-8').read()
e = QuizEvaluator()
report = e.generate_report(survey, '$Topic')
open('$outputDirPy', 'w', encoding='utf-8').write(report)
print('Quiz evaluation complete (exploratory mode)')
"@ 2>&1 | ForEach-Object { Write-Host "  $_" -ForegroundColor Green }
    }
}
else {
    if (-not $Topic) { Write-Host "  Skipped (no --Topic)" -ForegroundColor DarkGray }
    else { Write-Host "  Skipped (--SkipQuiz)" -ForegroundColor DarkGray }
}

# ────────────────────────────────────────────────
# 5. REFERENCE COMPARATIVE EVALUATION
# ────────────────────────────────────────────────
Write-Host "[5/7] Comparative Evaluation..." -ForegroundColor Yellow

if (-not $SkipComparative) {
    $refTopic = $Topic
    if (-not $refTopic) {
        $firstLine = ($surveyText -split "`n")[0]
        $refTopic = $firstLine -replace '^#\s*', '' -replace '^Survey:\s*', '' -replace 'A Survey of\s*', ''
        Write-Host "  Auto-detected topic: $refTopic"
    }

    $compOut = (Join-Path $OutputDir "05_comparative_report.md") -replace '\\', '/'
    $surveyPathPy = $SurveyPath -replace '\\', '/'
    $compRefTopic = $refTopic
    $benchDirPy = $benchDir -replace '\\', '/'
    $compOk = $false

    # If test set reference is available, use it directly
    if ($TestSetReference) {
        $refFile = Get-ChildItem $OutputDir -Filter "_testset_reference_*.md" | Select-Object -First 1
        if ($refFile) {
            Write-Host "  Using test set reference: $($refFile.Name)" -ForegroundColor Green
            try {
                & python -c @"
import sys; sys.path.insert(0, '$benchDirPy'); sys.path.insert(0, '$benchDirPy/evaluators')
from evaluators.comparative_evaluator import ComparativeEvaluator
from loaders.scireviewgen_loader import ReferenceSurvey
ref_text = open('$($refFile.FullName -replace "'", "''")', encoding='utf-8').read()
ref = ReferenceSurvey('$compRefTopic', '', {'reference': ref_text}, [])
e = ComparativeEvaluator('$surveyPathPy', reference_survey=ref)
result = e.compare_key_points()
with open('$compOut', 'w', encoding='utf-8') as f:
    f.write('# Comparative Evaluation (Test Set Reference)\n\n')
    f.write(f'**Reference:** $($compRefTopic -replace "'", "''")\n\n')
    f.write(json.dumps(result, indent=2))
print('Comparative complete')
"@ 2>&1 | Out-Null
                if (Test-Path $compOut) { Write-Host "  Done (test set)" -ForegroundColor Green; $compOk = $true }
            } catch { Write-Host "  Error: $_" -ForegroundColor Red }
        }
    }

    # Fallback to SciReviewGen dataset
    if (-not $compOk) {
        try {
        & python -c @"
import sys; sys.path.insert(0, '$benchDirPy'); sys.path.insert(0, '$benchDirPy/evaluators')
sys.argv = ['comparative_evaluator.py', '--generated', '$surveyPathPy', '--topic', '$compRefTopic', '--output', '$compOut']
from evaluators.comparative_evaluator import main as comp_main
comp_main()
"@ 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0 -and (Test-Path $compOut)) {
            Write-Host "  Done (SciReviewGen reference)" -ForegroundColor Green
        } else {
            Write-Host "  No matching reference found in dataset" -ForegroundColor DarkYellow
            Write-Host "  Fallback: LLM-based self-comparison..." -NoNewline
            $compPrompt = @"
You are evaluating a survey on "$refTopic". Assess its coverage compared to a typical comprehensive survey on this topic.

Rate each dimension 1-5:
1. Breadth of topic coverage
2. Depth of technical detail
3. Completeness of related work
4. Organization and flow
5. Quality of insights

Provide scores and brief justification.
"@
            $compFallback = Call-LLM -PromptText $compPrompt -OutputFile $compOut -MaxTokens 4000
            if ($compFallback -notlike "LLM_API_ERROR*") { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }
        }
    }
    catch {
        Write-Host "  Error: $_" -ForegroundColor Red
    }
}
else {
    Write-Host "  Skipped (--SkipComparative)" -ForegroundColor DarkGray
}

# ────────────────────────────────────────────────
# 6. VERIFIABILITY EVALUATION
# ────────────────────────────────────────────────
Write-Host "[6/7] Verifiability..." -ForegroundColor Yellow -NoNewline

if (-not $SkipVerifiability) {
    $surveyPathPy = $SurveyPath -replace '\\', '/'
    $outVerifPy = (Join-Path $OutputDir "06_verifiability_report.md") -replace '\\', '/'
    $benchDirPy = $benchDir -replace '\\', '/'

    try {
        & python -c @"
import sys; sys.path.insert(0, '$benchDirPy')
from evaluators.verifiability_evaluator import VerifiabilityEvaluator
survey = open('$surveyPathPy', encoding='utf-8').read()
e = VerifiabilityEvaluator()
r = e.generate_report(survey)
open('$outVerifPy', 'w', encoding='utf-8').write(r)
print('OK')
"@ 2>&1
        if ($LASTEXITCODE -eq 0) { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }
    }
    catch {
        Write-Host " ERROR" -ForegroundColor Red
    }
}
else {
    Write-Host " Skipped" -ForegroundColor DarkGray
}

# ────────────────────────────────────────────────
# 7. DOCUMENT IMPORTANCE
# ────────────────────────────────────────────────
Write-Host "[7/7] Document Importance..." -ForegroundColor Yellow -NoNewline

if (-not $SkipImportance) {
    $surveyPathPy = $SurveyPath -replace '\\', '/'
    $outImpPy = (Join-Path $OutputDir "07_importance_report.md") -replace '\\', '/'
    $benchDirPy = $benchDir -replace '\\', '/'

    try {
        & python -c @"
import sys; sys.path.insert(0, '$benchDirPy')
from evaluators.nugget_evaluator import DocumentImportanceEvaluator
survey = open('$surveyPathPy', encoding='utf-8').read()
e = DocumentImportanceEvaluator()
r = e.generate_report(survey)
open('$outImpPy', 'w', encoding='utf-8').write(r)
print('OK')
"@ 2>&1
        if ($LASTEXITCODE -eq 0) { Write-Host " OK" -ForegroundColor Green } else { Write-Host " ERROR" -ForegroundColor Red }
    }
    catch {
        Write-Host " ERROR" -ForegroundColor Red
    }
}
else {
    Write-Host " Skipped" -ForegroundColor DarkGray
}

# ────────────────────────────────────────────────
# GENERATE SUMMARY
# ────────────────────────────────────────────────
Write-Host "`nGenerating summary report..." -ForegroundColor Cyan

$summaryPath = Join-Path $OutputDir "00_summary.md"
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss UTC"

@"
# Full Evaluation Summary

**Survey**: $SurveyPath
**Model**: $Model
**Endpoint**: $($apiUrl -replace '/v1/chat/completions', '...')
**Date**: $now
**Topic**: $(if ($Topic) { $Topic } else { "N/A" })

---

## Score Overview

| # | Benchmark | Report | Status |
|---|-----------|--------|:------:|
"@ | Out-File -FilePath $summaryPath -Encoding utf8

$reports = @(
    @{Num=1; Name="SurveyBench"; File="01_surveybench_report.md"}
    @{Num=2; Name="DeepSurvey-Bench"; File="02_deepsurvey_report.md"}
    @{Num=3; Name="DeepScholar-Bench"; File="03_deepscholar_report.md"}
    @{Num=4; Name="Quiz Evaluation"; File="04_quiz_report.md"}
    @{Num=5; Name="Comparative Evaluation"; File="05_comparative_report.md"}
    @{Num=6; Name="Verifiability"; File="06_verifiability_report.md"}
    @{Num=7; Name="Document Importance"; File="07_importance_report.md"}
)

foreach ($r in $reports) {
    $rPath = Join-Path $OutputDir $r.File
    $exists = Test-Path $rPath
    $status = if ($exists) { "OK" } else { "N/A" }
    $link = if ($exists) { "[report]($($r.File))" } else { "-" }
    "$($r.Num) | $($r.Name) | $link | $status" | Add-Content $summaryPath
}

Add-Content $summaryPath "`n---`n## Quick Score Summary`n"

foreach ($r in $reports[0..2]) {
    $rPath = Join-Path $OutputDir $r.File
    if (Test-Path $rPath) {
        $content = Get-Content $rPath -Raw
        $scoreLines = $content -split "`n" | Where-Object { $_ -match '(?:Overall|Total|Final|Score)[^:]*[:]\s*\d+(?:\.\d+)?\s*/\s*\d+' }
        if ($scoreLines) {
            Add-Content $summaryPath "**$($r.Name):**"
            foreach ($sl in $scoreLines[0..3]) {
                Add-Content $summaryPath "- $($sl.Trim())"
            }
            Add-Content $summaryPath ""
        }
    }
}

Write-Host "`n================================================" -ForegroundColor Cyan
Write-Host "  EVALUATION COMPLETE" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Summary: $summaryPath"
Write-Host "Reports: $OutputDir"
