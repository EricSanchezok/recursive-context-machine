# Repo Snapshot

A single-agent RCM for answering: "What state is this repository in, and what should I do next?" It autonomously inspects git state, recent commits, project files, and local conventions, then writes an evidence-backed project snapshot.

## Run

From the repository you want to inspect:

```sh
accelerate run /path/to/RCM/examples/repo-snapshot/repo_snapshot.rcm \
  --purpose "看一下这个项目现在是什么状态，我下一步该做什么"
```

When running from this repository without installing:

```sh
cargo run --manifest-path /path/to/RCM/Cargo.toml --bin accelerate -- \
  run /path/to/RCM/examples/repo-snapshot/repo_snapshot.rcm \
  --purpose "看一下这个项目现在是什么状态，我下一步该做什么"
```

## What It Collects

- `git status`
- Recent commits and current branch
- Local diffs when present
- README, instruction files, tests, docs, TODO-like markers

## Output

- `outputs/repo-snapshot.md`

## Tools

- Built-in: `fs`, `find`, `git`, `shell`
- MCP: none

## Safety

This example is read-only by design. It should not modify files, create commits, switch branches, or run long commands.
