# Release policy

This repository follows **Conventional Commits** for commit messages and a
decision-log / CHANGELOG / RFC division for durable writing. The commit-msg hook
enforces the message format mechanically.

## Commit messages

The subject line follows Conventional Commits:

```
type(scope)!: subject
```

Allowed types are `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
`build`, `ci`, `chore`, and `revert`. A `!` marks a breaking change; a
`BREAKING CHANGE:` trailer in the body explains it. Merge commits are exempt.

## Writing durable changes

- **Decision log** ([docs/decisions/](../docs/decisions/)) records why one
  meaningful alternative won. It is append-only and is not a changelog.
- **CHANGELOG** records what changed between releases for users.
- **Specs** ([docs/specs/](../docs/specs/)) define intent, observable contract,
  implementation seams, verification, and evidence for risk-boundary work.

## RCM release automation

Release tags are created from `dev`. The release workflow builds the five
platform archives, publishes them in this repository, renders `Formula/rcm.rb`,
and opens a `release/vX.Y.Z` promotion pull request to `main`. The promotion PR
is the only path that changes `main`; required checks must pass before its
configured auto-merge can complete.

- Commit messages are the release contract — keep them honest.
- Publish steps run in CI, never locally.
- The archived distribution repository is not a release dependency.

## Versioning

Use semantic versioning: `MAJOR.MINOR.PATCH`. Breaking changes bump MAJOR;
features bump MINOR; fixes bump PATCH. Pre-release tags (`-alpha`, `-rc.1`) are
allowed for staged releases.
