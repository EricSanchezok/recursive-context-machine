# Folder Concierge

A single-agent RCM that turns one sentence into a conservative local folder cleanup packet. It inspects the current working directory, infers what the folder is for, and writes a reviewable plan plus a non-destructive move script.

## Run

From the folder you want to inspect, run the installed `accelerate` binary with this RCM file:

```sh
accelerate run /path/to/RCM/examples/folder-concierge/folder_concierge.rcm \
  --purpose "帮我整理当前文件夹，给出可执行的整理方案"
```

When running from this repository without installing:

```sh
cargo run --manifest-path /path/to/RCM/Cargo.toml --bin accelerate -- \
  run /path/to/RCM/examples/folder-concierge/folder_concierge.rcm \
  --purpose "帮我整理当前文件夹，给出可执行的整理方案"
```

## What It Collects

- Directory structure and file names
- Recently modified files
- Large or suspicious clutter categories
- Sensitive-looking files that should not be touched

## Output

- `outputs/folder-concierge-plan.md`
- `outputs/folder-concierge-move-files.sh`

The script is intentionally review-only. It should not delete files or overwrite existing data.

## Tools

- Built-in: `fs`, `find`, `shell`
- MCP: none

## Safety

This example should not mutate the folder directly. It writes recommendations and a reviewable script only.
