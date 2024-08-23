# Contributing to Filter List Manager

If you want to contribute to Filter List Manager please follow the instructions below.

## Notes about versioning

The main versioning objects in this repository are the flm and ffl crates.
They are versioned separately, and each has its own changelog.

In order to maintain changelogs correctly, we need some refs, and in this case for each changelog the refs are tags in a special format:
- For crate `adguard-flm` we use tags of the form `flm-${crate.version}`.
- For crate `adguard-flm-ffi` we use tags of the form `ffi-${crate.version}`.

**When and how should I tag and change crate versions?**

By default, versioning of crates is automatic and our CI raises patch versions in the crates by itself.
In this case, after PR merge into the master and after version increment, it is worth to set new tags for those crates that were incremented.

