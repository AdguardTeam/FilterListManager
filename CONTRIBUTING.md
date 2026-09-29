# Contributing to Filter List Manager

If you want to contribute to Filter List Manager please follow the instructions below.

## Notes about versioning

The main versioning objects in this repository are the flm and ffi crates.
Both crates share a single version number and a single changelog (`CHANGELOG.md`
at the repository root).

In order to maintain the changelog correctly, we need refs in the form of git
tags: `v${version}` (e.g. `v2.6.13`).

**When and how should I tag and change crate versions?**

Never by hand. `CHANGELOG.md` is the only place that holds the version: the
manifests carry none, and builds take it from the `FLM_VERSION` environment
variable (else `git describe` over `v*` tags). To release:

1. Run the `Prepare release` workflow with the new tag (e.g. `v2.7.0`). It opens
   a `release-bump/` pull request that turns the `Unreleased` section of
   `CHANGELOG.md` into that version.
2. Merge the pull request. `Publish release` then creates the tag and builds and
   publishes the crates and the Apple, KMP and Windows adapters with that version.
