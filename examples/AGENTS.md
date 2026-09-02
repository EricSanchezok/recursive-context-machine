# Examples

Each child directory is a runnable RCM project. Keep its `rcm.toml`, `.rcm` files,
prompts, and README self-contained and use paths relative to that example.

- Runtime outputs belong under the example's ignored `outputs/` or run directory.
- Do not commit credentials, downloaded papers, generated reports, or trajectory
  data.
- Update the example README and CLI example tests when a project layout changes.
- Use the root `accelerate` build and the real project entry point when verifying an
  example; do not replace a parser or graph test with a hand-wired mock.
