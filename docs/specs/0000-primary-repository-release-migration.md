# Primary-repository release migration

Artifact-Version: 1
Status: Implemented

## Intent

Publish RCM binaries, installers, checksums, and the Homebrew Formula from
`recursive-context-machine` itself. Development remains on `dev`; `main` changes
only through an automated release promotion pull request. The old `rcm-dist`
repository is no longer a release dependency.

## Contract

- A `v*` tag must point to a commit reachable from `dev` and match the CLI package
  version.
- The release workflow builds the five supported platform archives and a
  `checksums.txt` file, then publishes them to the current repository Release.
- The workflow creates a `release/vX.Y.Z` branch, writes Formula hashes, opens a
  promotion PR to `main`, and enables squash auto-merge only after required checks
  pass.
- Installers resolve releases from the current repository and never require
  `rcm-dist` credentials or checkout.
- The maintainer workflow downloads and extracts the Linux archive format emitted
  by the release workflow.

## Plan

Keep the existing build matrix and compatibility checks. Remove cross-repository
release and Formula updates, generate the Formula from the release artifacts, add
a release-only promotion check, and enable repository auto-merge while retaining
main branch protection.

## Verification

- `actionlint` validates the workflow definitions.
- `shellcheck install.sh` and `ruby -c Formula/rcm.rb` validate distribution files.
- Rust formatting, clippy, build, nextest, doctests, dependency hygiene, and
  repo-seed gates pass on the release branch.
- A `v0.2.20` release rehearsal confirms archive names, checksums, Formula URLs,
  and promotion-PR behavior.

## Evidence

- [Release workflow](../../.github/workflows/release-accelerate.yml)
- [Main promotion gate](../../.github/workflows/release-promotion.yml)
- [Formula renderer](../../scripts/render-formula.sh)
- [Repository governance manifest](../../.repo-seed/manifest.json)
