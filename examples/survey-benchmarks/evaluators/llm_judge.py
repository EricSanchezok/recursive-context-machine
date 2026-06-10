"""LLM-as-Judge evaluator for benchmark evaluations.
Calls OpenAI-compatible API (gmncode / DeepSeek) directly."""

import os
import sys
import json
import requests
import time
import re

API_KEY = os.environ.get("CROSS_JUDGE_API_KEY") or os.environ.get("DEEPSEEK_API_KEY")
API_ENDPOINT = os.environ.get("LLM_API_ENDPOINT", "https://api.gmncode.com/v1/chat/completions")
API_MODEL = os.environ.get("LLM_MODEL", "gpt-5.5")

def call_llm(prompt: str, system_prompt: str = None, max_retries: int = 3) -> str:
    """Call LLM with the given prompt. Returns the response text."""
    if not API_KEY:
        return None

    messages = []
    if system_prompt:
        messages.append({"role": "system", "content": system_prompt})
    messages.append({"role": "user", "content": prompt})

    for attempt in range(max_retries):
        try:
            resp = requests.post(
                API_ENDPOINT,
                headers={"Authorization": f"Bearer {API_KEY}"},
                json={
                    "model": API_MODEL,
                    "messages": messages,
                    "temperature": 0.0,
                    "max_tokens": 16384,
                },
                timeout=300,
            )
            if resp.status_code == 200:
                return resp.json()["choices"][0]["message"]["content"]
            elif resp.status_code == 429:
                wait = min(2 ** attempt * 5, 60)
                time.sleep(wait)
                continue
            else:
                return f"[API Error {resp.status_code}]: {resp.text[:200]}"
        except Exception as e:
            if attempt < max_retries - 1:
                time.sleep(5)
            else:
                return f"[Error]: {str(e)[:200]}"
    return None


def run_benchmark_evaluation(survey_path: str, prompt_path: str, output_path: str) -> dict:
    """Run a benchmark evaluation by calling LLM with prompt + survey content."""
    # Read survey
    with open(survey_path, "r", encoding="utf-8") as f:
        survey_text = f.read()
    
    # Read prompt
    with open(prompt_path, "r", encoding="utf-8") as f:
        prompt_template = f.read()
    
    # Skip the Instruction section (RCM-specific) since we're calling API directly
    if prompt_template.startswith("## Instruction"):
        lines = prompt_template.split("\n")
        cleaned = []
        skip_block = False
        for line in lines:
            if line.strip() == "---":
                skip_block = True
                continue
            if skip_block:
                cleaned.append(line)
        if cleaned:
            prompt_template = "\n".join(cleaned)
    
    # Build full prompt: append survey text
    full_prompt = f"""{prompt_template}

## Survey to Evaluate

Below is the survey content to evaluate:

```markdown
{survey_text}
```

---

Please provide your complete evaluation following the criteria above. Use the exact format specified in the evaluation dimensions. Include scores for each dimension and a final overall score."""

    print(f"  Calling LLM ({API_MODEL})...")
    response = call_llm(full_prompt)
    
    if response is None:
        print(f"  ERROR: No API key available")
        return {"status": "error", "message": "No API key. Set CROSS_JUDGE_API_KEY or DEEPSEEK_API_KEY."}
    
    # Save raw response
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(f"# Benchmark Evaluation Report\n\n")
        f.write(f"**Model**: {API_MODEL}\n")
        f.write(f"**Survey**: {survey_path}\n\n")
        f.write(f"---\n\n")
        f.write(response)
    
    print(f"  Saved to: {output_path}")
    
    # Extract scores for summary
    scores = extract_scores(response)
    
    return {"status": "ok", "output": output_path, "scores": scores}


def extract_scores(text: str) -> dict:
    """Extract numerical scores from evaluation text."""
    scores = {}
    
    # Pattern: "X/Y" or "X/5" or "X%" or "Score: X"
    patterns = [
        (r"(\w+[^:：]*?)[:：]\s*(\d+(?:\.\d+)?)\s*/\s*(\d+)", lambda m: (m.group(1).strip(), float(m.group(2)) / float(m.group(3)) * 100)),
        (r"(\w+[^:：]*?)[:：]\s*(\d+(?:\.\d+)?)%", lambda m: (m.group(1).strip(), float(m.group(2)))),
    ]
    
    for pattern, extractor in patterns:
        for m in re.finditer(pattern, text):
            try:
                name, score = extractor(m)
                if name not in scores:
                    scores[name] = score
            except:
                pass
    
    return scores
