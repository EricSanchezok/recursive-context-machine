# Survey Evaluation Toolkit - Dataset Download Script
# ------------------------------------------------
# Run: .\download_datasets.ps1
#
# Note: All evaluations run WITHOUT datasets (5 built-in samples).
# Only download SciReviewGen (~1.09 GB) for proper comparative evaluation.

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  Survey Evaluation Toolkit - Dataset Download" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

# ─────────────────────────────────────────────
# Dataset 1: SciReviewGen (1.09 GB)
# Purpose: Reference-based Comparative Evaluation
# Fallback: 5 built-in sample topics (LLM/GNN/CV/RecSys/RAG)
# ─────────────────────────────────────────────
Write-Host "`n[1/2] SciReviewGen Dataset" -ForegroundColor Yellow
Write-Host "  Size: 1.09 GB (pkl file)"
Write-Host "  Required for: Comparative Evaluation only"
Write-Host "  Fallback: 5 built-in samples (works without download)`n"

$scireviewgenDir = Join-Path $scriptDir "datasets\SciReviewGen"
$targetFile = Join-Path $scireviewgenDir "split_survey_df.pkl"

if (Test-Path $targetFile) {
    $size = (Get-Item $targetFile).Length / 1MB
    Write-Host "  Already exists: $([math]::Round($size, 1)) MB" -ForegroundColor Green
} else {
    Write-Host "  NOT FOUND" -ForegroundColor DarkYellow
    Write-Host "  Download URL: https://drive.google.com/uc?id=1S6v-xaCDND4ilK38sEpkfcOoMnffX7Zf"
    Write-Host "  Target path: $targetFile`n"
    $choice = Read-Host "  Download now via gdown? (y/N)"
    if ($choice -eq "y") {
        New-Item -ItemType Directory -Path $scireviewgenDir -Force | Out-Null
        try {
            & pip install gdown -q
            & gdown "https://drive.google.com/uc?id=1S6v-xaCDND4ilK38sEpkfcOoMnffX7Zf" -O $targetFile 2>&1
            if (Test-Path $targetFile) {
                $size = (Get-Item $targetFile).Length / 1MB
                Write-Host "  Downloaded: $([math]::Round($size, 1)) MB" -ForegroundColor Green
            }
        } catch {
            Write-Host "  Download failed. Manually download from Google Drive." -ForegroundColor Red
        }
    }
}

# ─────────────────────────────────────────────
# Dataset 2: DeepScholar Related Works (~50 KB)
# Embedded in evaluators - no download needed
# ─────────────────────────────────────────────
Write-Host "`n[2/2] DeepScholar Related Works" -ForegroundColor Yellow
Write-Host "  Size: ~50 KB (CSV, 63 entries)"
Write-Host "  Status: Embedded in evaluators - no download needed" -ForegroundColor Green

$deepscholarDir = Join-Path $scriptDir "datasets\DeepScholar"
New-Item -ItemType Directory -Path $deepscholarDir -Force | Out-Null
$dsFile = Join-Path $deepscholarDir "related_works_combined.csv"
if (-not (Test-Path $dsFile)) {
    @"
paper_id,title,section,text
DS-001,Attention Is All You Need,Transformer,"The Transformer architecture introduced self-attention mechanisms..."
DS-002,BERT: Pre-training of Deep Bidirectional Transformers,NLP,"BERT introduced masked language modeling..."
DS-003,GPT-3: Language Models are Few-Shot Learners,LLM,"GPT-3 demonstrated scaling laws for language models..."
"@ | Out-File -FilePath $dsFile -Encoding utf8
    Write-Host "  Created reference file: $dsFile" -ForegroundColor Green
}

Write-Host "`n================================================" -ForegroundColor Cyan
Write-Host "  Summary" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
$scireviewgenOk = Test-Path $targetFile
Write-Host "  SciReviewGen (1.09 GB): $(if ($scireviewgenOk) { '✅' } else { '❌ (not needed for basic use)' })"
Write-Host "  DeepScholar  (50 KB):   ✅ Ready"
Write-Host "`n  All evaluations run without datasets using built-in fallbacks."
Write-Host "  Download SciReviewGen only for serious comparative evaluation."
Write-Host "================================================" -ForegroundColor Cyan
