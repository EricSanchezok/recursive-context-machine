# Release artifacts live in the primary repository

## Status
Accepted
Class: process

## Context and Problem Statement

RCM's source repository is public and already owns the version tags and release
workflow. A second `rcm-dist` repository duplicates release state, requires a
cross-repository token, and makes the latest binary differ from the source tag.
The repository name is now `recursive-context-machine`, so public installation and
release URLs can share one source of truth.

## Decision Drivers

- Release assets must be traceable to the exact source tag that built them.
- Main must remain protected and updated only after release checks succeed.
- Homebrew metadata needs the same release URLs and checksums as other installers.
- The workflow should not require a second repository or token.

## Considered Options

- Keep source releases in a separate `rcm-dist` repository.
- Publish assets in the primary repository while keeping Formula updates external.
- Publish assets and Formula from the primary repository, then promote the tagged
  snapshot to `main` through an automated PR.

## Decision Outcome

The primary repository publishes the platform archives and `checksums.txt`,
generates `Formula/rcm.rb` from those hashes, and creates a `release/vX.Y.Z` PR to
`main`. Required CI and a release-only promotion check must pass before GitHub
auto-merges the PR. `rcm-dist` is archived as historical context and is not read by
the release workflow.

## Pros and Cons of the Options

### Separate distribution repository

- Good: a tap-oriented repository can contain only distribution files.
- Bad: source, tags, assets, and Formula drift; a cross-repository token is needed.

### Primary assets with external Formula

- Good: source tags own the binary assets.
- Bad: installation metadata still has a second authority and update path.

### Primary assets and Formula with promotion PR

- Good: one repository owns source, release assets, checksums, Formula, and history.
- Good: protected main still receives only verified release snapshots.
- Trade-off: release automation owns a promotion branch and a small PR lifecycle.

## Links

- [Release migration spec](../specs/0000-primary-repository-release-migration.md)
- [Release workflow](../../.github/workflows/release-accelerate.yml)
- [Main promotion gate](../../.github/workflows/release-promotion.yml)
