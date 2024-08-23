# AdGuard filter list manager

This repository contains a library for managing AdGuard filter lists and its
tools and wrappers.

This library is used by different AdGuard applications to integrate filter
registries ([FiltersRegistry][filtersregistry] and
[HostlistsRegistry][hostlistsregistry]), check for updates and download them,
implement differential updates, etc.

[filtersregistry]: https://github.com/AdguardTeam/FiltersRegistry
[hostlistsregistry]: https://github.com/AdguardTeam/HostlistsRegistry

## Crates

- [Filter List Manager][flmreadme]: Filter List Manager library core. The main
  module containing the library itself. This section also contains the main
  documentation for using the library.
- [FFI][ffireadme]: FFI interface for the library. In other words, this is a
  wrapper for integrating the library into programs written in other programming
  languages.
- [CLI][clireadme]: CLI tools for the library.

[flmreadme]: ./crates/filter-list-manager/README.md
[ffireadme]: ./crates/ffi/README.md
[clireadme]: ./crates/cli/README.md

## Development

Install [rust][rust].

[rust]: https://www.rust-lang.org/tools/install

### Requirements

Rust 1.75 (versions 1.76+ don't support Windows 7)

### Linters

- Run `cargo fmt --all -- --check` to check the code formatting.
- Run `npx markdownlint-cli .` to lint the documentation.

### Tests

Read here how to [run tests][coverage] with coverage.

[coverage]: ./COVERAGE.md
