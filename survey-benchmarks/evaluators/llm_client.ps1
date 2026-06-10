param(
    [Parameter(Mandatory=$true)][string]$Prompt,
    [Parameter(Mandatory=$true)][string]$SurveyPath,
    [string]$OutputPath,
    [string]$ApiKey = "",
    [string]$Model = "",
    [string]$Endpoint = "",
    [switch]$ReturnJson,
    [int]$MaxTokens = 16384
)

$ErrorActionPreference = "Stop"

# ── Resolve API configuration ──
if (-not $ApiKey) {
    $envCandidates = @("EVA_API_KEY", "OPENAI_API_KEY", "LLM_API_KEY")
    foreach ($candidate in $envCandidates) {
        if (Get-Item -Path "env:$candidate" -ErrorAction SilentlyContinue) {
            $ApiKey = [Environment]::GetEnvironmentVariable($candidate)
            break
        }
    }
}
if (-not $ApiKey) {
    Write-Error "No API key provided. Use -ApiKey or set EVA_API_KEY env var."
    exit 1
}

if (-not $Endpoint) {
    $Endpoint = [Environment]::GetEnvironmentVariable("EVA_ENDPOINT")
    if (-not $Endpoint) { $Endpoint = "https://gmncode.com/v1/chat/completions" }
}

if (-not $Model) {
    $Model = [Environment]::GetEnvironmentVariable("EVA_MODEL")
    if (-not $Model) { $Model = "gpt-5.5" }
}

$apiUrl = $Endpoint

# Read survey
$surveyText = Get-Content $SurveyPath -Raw -Encoding utf8

# Build the full user message
$userMsg = @"
$Prompt

## Survey Content to Evaluate

The survey content is provided below. Evaluate it according to the criteria above.

````markdown
$surveyText
```

Please provide your complete evaluation with scores following the format specified in the criteria.
"@

# API call
$body = @{
    model = $Model
    messages = @(
        @{role = "user"; content = $userMsg}
    )
    temperature = 0.0
    max_tokens = $MaxTokens
} | ConvertTo-Json -Depth 5

try {
    $response = Invoke-RestMethod -Uri $apiUrl -Method Post `
        -Headers @{Authorization = "Bearer $ApiKey"} `
        -Body $body -ContentType "application/json" -UseBasicParsing

    $resultText = $response.choices[0].message.content

    if ($OutputPath) {
        $resultText | Out-File -FilePath $OutputPath -Encoding utf8
    }

    if ($ReturnJson) {
        $resultText | ConvertTo-Json
    } else {
        $resultText
    }
}
catch {
    $errorMsg = "LLM_API_ERROR: $_"
    if ($OutputPath) {
        $errorMsg | Out-File -FilePath $OutputPath -Encoding utf8
    }
    Write-Error $errorMsg
    exit 1
}
