# Contributing to Filter List Manager

If you want to contribute to Filter List Manager please follow the instructions below.

## Notes about versioning

The main versioning objects in this repository are the flm and ffi crates.
Both crates share a single version number and a single changelog (`CHANGELOG.md`
at the repository root).

In order to maintain the changelog correctly, we need refs in the form of git
tags: `v${version}` (e.g. `v2.6.13`).

**When and how should I tag and change crate versions?**

By default, versioning of crates is automatic and our CI raises patch versions in the crates by itself.
In this case, after PR merge into the master and after version increment, it is worth to set new tags for those crates
that were incremented.
