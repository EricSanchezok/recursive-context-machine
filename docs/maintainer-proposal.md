# RCM Maintainer — AI-Native Project Maintenance Agent

## Vision

An open-source, pipeline-driven AI maintainer that lives inside your GitHub repo. It responds to issues, reviews PRs, reproduces bugs, fixes code, and opens pull requests — all defined as composable `.rcm` pipelines that users can inspect, customize, and extend.

Unlike hosted SaaS products (Dosu, Sweep), every behavior is a declarative `.rcm` file. There is no black box. Users own their workflows.

## Problem

Open-source maintainers face repetitive mechanical work:
- Triaging new issues (duplicate detection, classification, priority)
- Responding to common questions
- Reviewing PRs for basic correctness
- Stale issue management

Existing tools either offer shallow automation (GitHub Actions with hardcoded scripts) or are closed SaaS products with opaque behavior.

## Approach

Leverage RCM's composable pipeline architecture to build a stateless, event-driven maintainer agent. Each GitHub event type maps to a dedicated `.rcm` pipeline. The agent is stateless — GitHub itself serves as the persistent store (issue bodies, comment threads, PR diffs are all retrieved via `gh` API on every invocation).

### Key Principles

1. **Pipeline-per-scenario**: Each event type (issue opened, @mention, PR opened, scheduled) has its own `.rcm` file. Small, auditable, composable.
2. **Stateless**: No database. Every invocation starts fresh and pulls full context from GitHub API. Conversation history lives in the issue/PR comment thread.
3. **Tool-driven**: New capabilities (`github`, `git`) are tools in the accelerator crate. LLMs use them through the existing tool-call protocol — no custom orchestration code.
4. **User-customizable**: Users edit `.rcm` files to change behavior (response style, triage logic, review criteria). No recompilation needed for prompt changes.

## Architecture

```
GitHub Events (webhook / Actions)
  │
  │  event type → dispatch to corresponding .rcm
  ▼
┌─────────────────────────────────────┐
│  .rcm Pipelines (one per scenario)  │
│                                     │
│  issue_opened.rcm                   │
│    reproduce → dedup → comment      │
│                                     │
│  issue_mentioned.rcm                │
│    fetch history → intent → route   │
│      ├→ analyze                     │
│      ├→ fix (edit code → PR)        │
│      ├→ explain                     │
│      └→ generic reply               │
│                                     │
│  pr_opened.rcm                      │
│    fetch diff → review → comment    │
│                                     │
│  health_check.rcm                   │
│    fetch issues/PRs → report        │
└─────────────────────────────────────┘
  │
  │  tool calls
  ▼
┌─────────────────────────────────────┐
│  Tools                              │
│  ─ existing ─                       │
│  shell, fs (read/write/edit/list),  │
│  arxiv                              │
│  ─ to add ─                         │
│  github (comment, label, react)     │
│  git (branch, commit, push)         │
└─────────────────────────────────────┘
```

## Deployment

**Phase 1 — GitHub Actions** (recommended starting point)

One workflow YAML dispatches events to `.rcm` files. `GITHUB_TOKEN` is provided automatically by Actions — no App registration, no server, no SSL certificates. Works on any repo by adding the workflow file.

**Phase 2 — GitHub App** (for multi-repo, always-on)

Package as a GitHub App with webhook receiver (axum server). Enables real-time response, cross-repo installation, and finer permission control. The `.rcm` pipelines remain identical — only the gateway layer changes.

## Scenarios

### Issue Opened
Automatically triggered on new issue. Attempts to understand the problem, searches for duplicates across existing issues and PRs, suggests labels, and posts an initial analysis comment.

### Issue Mentioned (@maintainer)
Triggered when a comment mentions `@maintainer`. Fetches the full comment thread (conversation history), classifies intent (fix / analyze / explain / general), and routes to the appropriate pipeline branch. The "fix" branch reads relevant code, proposes changes, commits to a new branch, and opens a PR — all within the pipeline.

### PR Opened
Triggered on new pull request. Fetches the diff, summarizes changes, checks for common issues (missing tests, large diff, unclear description), and posts a review summary comment.

### Health Check
Triggered manually or on schedule. Fetches all open issues and PRs, runs triage and review analysis, generates a project health report posted as a discussion or issue comment.

## New Tools Required

### `github` tool
Wraps `gh` CLI for GitHub interactions:
- `comment` — post comment on issue/PR
- `label` — add/remove labels
- `react` — add emoji reaction
- `list-issues` / `list-prs` — search with filters
- `diff` — fetch PR diff

### `git` tool
Wraps git operations for code changes:
- `branch` — create/switch branch
- `add` / `commit` — stage and commit changes
- `push` — push to remote
- Combined with existing `fs` tools (read/write/edit) for code modification

## Phased Roadmap

| Phase | Scope | Deliverable |
|-------|-------|-------------|
| 0 | Current state | `health_check.rcm` — fetch issues/PRs, generate report (done) |
| 1 | Tools | Implement `github` and `git` tools in accelerator crate |
| 2 | Issue opened | `issue_opened.rcm` + Actions workflow, end-to-end verified |
| 3 | @mention routing | `issue_mentioned.rcm` with intent classification and condition-based routing |
| 4 | Code fix flow | "fix" intent branch: read code → edit → commit → push → open PR |
| 5 | PR review | `pr_opened.rcm` with diff analysis and review comments |
| 6 | GitHub App | Package as installable App with webhook server for always-on deployment |

## What Makes This Different

| | Dosu / Sweep | RCM Maintainer |
|---|---|---|
| Behavior definition | Closed / config-only | Open `.rcm` pipelines, fully editable |
| Customization | Limited toggles | Rewrite any pipeline, add new tools |
| State | Server-side database | Stateless (GitHub is the store) |
| Deployment | SaaS only | Self-hosted Actions OR GitHub App |
| Transparency | Black box | Every step is a declared accelerator with visible purpose |
