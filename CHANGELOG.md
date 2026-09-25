# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.7.0] - 2026-09-25

### Added

- `Configuration.always_compare_filters_hashes_on_update` (default `true`): forced updates compare filter hashes and report only really changed filters.

### Fixed

- Filters with unchanged contents are no longer re-downloaded on every update after they expire.

## [2.6.13] - 2026-07-08

### Fixed

- Windows adapter build script: invalid `--features` argument list in
  `build_adapter.ps1`.

## [2.6.12] - 2026-07-08

### Added

- Windows NuGet package now includes `metadata.json` with the `adguard-flm-ffi`
  crate version.

## [2.6.11] - 2026-07-06

### Added

- Kotlin Multiplatform adapter layer: `FlmAdapter` interface,
  `FlmAdapterFactory`, `FlmAdapterImpl`
- KDoc documentation for `FlmAdapterFactory` and its `create` method

## [2.6.10] - 2026-06-30

### Changed

- SQLite journal mode is now set to WAL (Write-Ahead Logging) on every
  connection to reduce disk write volume. WAL is a persistent, database-level
  setting stored in the DB header, so subsequent connections use it
  automatically. This creates `<dbname>-wal` and `<dbname>-shm` sidecar files
  alongside the database. Setting WAL is best-effort: on filesystems that do
  not support it (e.g. some network/shared mounts) the pragma has no effect and
  the database keeps its existing journal mode; the effective mode can be
  checked on demand via `PRAGMA journal_mode`.

## [2.6.9] - 2026-05-13

### Added

- Windows FFI tests

### Fixed

- `pull_metadata` left stale `integrity_signature` on filter metadata rows
  after index merge mutated signed fields (`expires`, `last_update_time`,
  `download_url`, `subscription_url`), causing subsequent reads to fail with
  `FilterIntegrityCheckFailed`
- `filter_count_signature` was not refreshed by `pull_metadata`, so
  adding/removing filters via the index could desync the count signature
- Orphan `filter_includes` rows were left behind when a filter was removed or
  moved to custom during `pull_metadata`

## [2.6.0] - 2026-04-13

### Added

- Configuration field `filter_update_concurrency` to control the maximum number
  of concurrent filter download/compilation threads (default: 16, max: 16)
- Configuration field `filter_update_dispatch_delay_ms` to set minimum delay in
  milliseconds between consecutive filter download dispatches to avoid HTTP 429
  errors (default: 60)

### Changed

- Method names: `sign_all_rules` → `sign_all_data`,
  `sign_all_rules_with_new_key` → `sign_all_data_with_new_key`
- All methods that read rules will verify signatures of the rules
- All methods that write rules will verify signatures of the rules, then sign

## [2.5.0] - 2026-03-11

### Added

- `FilterListType::MISC` for managing a collection of miscellaneous filters

### Changed

- Constants `STANDARD_FILTERS_DATABASE_FILENAME`,
  `DNS_FILTERS_DATABASE_FILENAME` replaced with
  `build_database_name_for_filter_list_type(FilterListType)`
- Index localizations are now optional: when
  `Configuration::metadata_locales_url` is empty (default), localizations are
  skipped (previously this could fail)
- Filter updates now use multithreading, speeding up the operation by up to 8x

### Fixed

- `save_rules_to_file_blob` now respects filter includes
- Reformatted, cleaned up, and fixed `README.md`

## [2.4.0] - 2026-02-18

### Added

- Storage integrity control

### Changed

- Build images updates

## [2.3.4] - 2025-11-19

### Changed

- Windows adapter version bumped to 2.3.4

### Fixed

- 404 error of filter diff patch was considered as HTTP error rather than no
  content

## [2.3.1] - 2025-11-12

### Fixed

- Incorrect diff application

## [2.3.0] - 2025-11-06

### Changed

- Diff updates are now recursive, up to 10 iterations

## [2.2.0] - 2025-10-08

### Added

- `update_filters_by_ids` method for more flexible filters update

## [2.1.0] - 2025-09-09

### Added

- `should_ignore_expires_for_local_urls` field to `Configuration` which allows
  to ignore expires for local urls during update (default: `false`)
- Rust filter library now restricts adding `pdf/gif/png/jpeg` files as filter

## [2.0.1] - 2025-08-25

### Changed

- `update*` methods will check filters contents during update

### Fixed

- `has_directives` field was not set to `false` if there are no directives in
  the filter

## [2.0.0] - 2025-08-18

### Changed

- Version 2.0 is released

## [2.0.0-rc.3] - 2025-07-10

### Added

- `get_active_rules_raw` method

### Fixed

- `InvalidChecksum` hash error format for diff updates

## [2.0.0-rc.2] - 2025-06-25

### Changed

- Apply clippy for workspace

## [2.0.0-rc.1] - 2025-06-23

### Added

- Kotlin for Android FLM bindings
- `pull_metadata` now returns `PullMetadataResult`

### Changed

- Windows: change the way to handle FLM errors
- Filters parsing changed to two-step process:
  1. **Compilation** — takes the main filter body, saves it as-is into the
     `rules_list` table without resolving directives. Includes are collected
     and saved into a separate `filter_includes` table. All recursive includes
     are inlined during compilation, and all conditional directives in includes
     are resolved too.
  2. **Collection** — takes the compiled main filter from `rules_list` and
     includes from `filter_includes`. All directives are resolved, and all
     includes are injected into the main filter body. Collected filters are
     returned to the caller.

### Removed

- Uniffi bindings and builds are completely removed

### Fixed

- `get_filters_*` and `fetch_filter_*` method names corrected

## [1.9.0] - 2025-04-14

### Added

- `filter_url`, `http_client_error` fields to `UpdateFilterError`

## [1.8.0] - 2025-03-31

### Fixed

- Protobuf optional fields
- `save_rules_to_file_blob` method saves disabled rules

## [1.7.6] - 2025-03-28

### Fixed

- Bamboo increments custom version

## [1.7.5] - 2025-03-26

### Added

- Service layer between manager and storage

## [1.7.4] - 2025-03-25

### Fixed

- `update_filters` updates user defined title and description

## [1.7.3] - 2025-03-25

### Fixed

- `update_filters`, `force_update_filters_by_ids` methods should not fetch
  indexes for diff update

## [1.7.0] - 2025-03-20

### Changed

- Windows protobuf-based adapter is default now. Uniffi one is obsolete and
  moved to `platform/windows/uniffi`

## [1.6.3] - 2025-03-19

### Changed

- Docs for `update_filters`, `force_update_filters_by_ids`, `pull_metadata`
  methods

## [1.6.1] - 2025-03-17

### Added

- `get_rules_count` method for getting rules count by filter ids

## [1.5.10] - 2025-03-14

### Fixed

- Suggest fallback locale in `change_locale`

## [1.5.2] - 2025-03-11

### Fixed

- OR expressions in `BooleanExpressionParser`

## [1.5.1] - 2025-03-06

### Added

- Client app name and version in configuration

## [1.4.3] - 2025-03-06

### Added

- `get_active_rules` method for Apple platform

## [1.4.1] - 2025-02-24

### Added

- `fetch_filter_list_metadata_with_body` method

## [1.3.5] - 2025-02-10

### Added

- `flm_get_constants` method for the new native API

## [1.3.1] - 2025-01-31

### Added

- Clippy linting
- Update routines perform index versions checking before downloading filters,
  and do not update up-to-date filters

### Changed

- Title and description fields will be renewed while updating filters
- `pull_metadata` won't set versions of filters; `update_filters` does

### Fixed

- Tests support multithreading back again

## [1.2.1] - 2025-01-24

### Added

- Tries to normalize slightly malformed filter urls
- Proxy mode in configuration and FLM interface

### Fixed

- Speed up HTTP clients

## [1.1.21] - 2025-01-17

### Fixed

- Unnecessary filter rules selection in `save_disabled_rules`
- `file:///` urls support in `IndexesParser`

## [1.1.20] - 2024-12-19

### Added

- Static CRT link in Windows libraries
- Windows `.rc` file

## [1.1.19] - 2024-12-19

### Fixed

- File checksum should respect file newline
- Checksums will be checked only for index filters
- Install custom list is not setting last download time

## [1.1.13] - 2024-12-06

### Fixed

- Diffupdates lines count for files without `\n` on end

## [1.1.10] - 2024-12-04

### Fixed

- Diffupdates now respects trailing newlines + fix checksum validator

## [1.1.2] - 2024-11-18

### Added

- Protobuf-based FFI
- `flm_default_configuration_protobuf` as default `Configuration` object
- `flm_init_protobuf` as new `FLM` constructor
- `flm_call_protobuf` as `FLM` methods caller
- `flm_get_constants` as library constants holder
- `flm_free_handle` as cleanup handler for `FLM Handle`
- `flm_free_response` as cleanup handler for `RustResponse`

### Changed

- `FilterId` type changed from `i64` to `i32`
- Uniffi interface dropped for Apple build by default
- Uniffi build for Windows is obsolete now

### Removed

- `Configuration.encryption_key` key removed
- `get_full_filter_lists` method

### Fixed

- `get_database_path` returns the absolute path to the database, even if a
  relative path was specified in the configuration

## [0.8.17] - 2024-11-07

### Added

- Tests for `update_filters`

### Fixed

- Update filters bug when filter is not yet installed but metadata has the same
  version of filter as body
- `disabled_rules` were dropped after update

## [0.8.15] - 2024-11-07

### Fixed

- SQL error in `update_filters`

## [0.8.13] - 2024-11-06

### Fixed

- Where clause for empty entities list

## [0.8.11] - 2024-11-05

### Fixed

- Fixed `flm-0.8.5` release

## [0.8.9] - 2024-11-05

### Added

- `get_disabled_rules` method

### Fixed

- Disabled rules of filters were removed after filters update

## [0.8.7] - 2024-11-01

### Fixed

- `file:` protocol-based paths are being decoded the right way
  (e.g. `Path%20With%20Spaces`)
- Now when filters are updated their versions are checked

## [0.8.5] - 2024-10-29

### Added

- Method `save_rules_to_file_blob` for incremental writing filter rules to file

## [0.8.2] - 2024-10-25

### Added

- `DatabaseBusy` error code

### Changed

- DB queries are now executed through mutex queue
- Change mutexes at FFI to r/w lock, which write-locks only for `change_locale`

## [0.7.7] - 2024-10-10

### Fixed

- Method `get_filter_rules_as_strings` wasn't exported in previous version

## [0.7.6] - 2024-10-10

### Added

- Method `get_filter_rules_as_strings`

## [0.7.1] - 2024-09-23

### Added

- `auto_lift_up_database: bool` to `Configuration` for disabling/enabling
  autolifting in the constructor

### Changed

- Now FLM constructor can throw exceptions
- Automatic database uplifting now called in the constructor, not after very
  first database connection

## [0.6.3] - 2024-09-23

### Added

- `get_stored_filters_metadata*` methods

### Changed

- Reduced Apple framework build size

## [0.6.0] - 2024-09-17

### Added

- `lift_up_database` method
- Migrations that run when the `lift_up_database` method is called
- Automatic "lift" database after the very first connection to database

### Removed

- Drop `download_url` unique constraint

### Fixed

- Windows rust lib and Windows adapter build
- Reduce build size by `panic=abort`, remove symbols for all platforms and do
  not bundle SQLite for Apple
- `install_custom_filter_*` methods with `download_url=<empty string>` drops
  user rules filter when called
- `save_custom_filter_rules` must update `filter.time_updated` too

## [0.5.10] - 2024-09-03

### Changed

- FLM version restrictions

### Fixed

- Service and custom groups should not be deleted during index update

## [0.5.9] - 2024-09-02

### Fixed

- `get_active_rules` contains empty rules if `filter.rules.disabled_rules` is
  empty

## [0.5.7] - 2024-08-29

### Added

- Filters with the `deprecated = true` field will not be saved to the database
  when parsing indexes

### Fixed

- Filters downloading must fail when status code >= 400

## [0.5.6] - 2024-08-28

### Added

- Pre-validate filters body before parsing (HTML or XML documents will be
  rejected)
- `HttpStrict200Response` error if filter downloading response has success code
  but not 200
- `FilterContentIsLikelyNotAFilter` pre-validate filter error

## [0.5.1] - 2024-08-23

### Added

- Changelog

### Fixed

- Cleanup in `README.md`
- Split changelog files by crates

## [0.5.0] - 2024-08-19

### Added

- Changelog

### Fixed

- Documentation of the filter-list-manager crate was cleaned up

[Unreleased]: https://github.com/AdguardTeam/FilterListManager/compare/v2.7.0...HEAD
[2.7.0]: https://github.com/AdguardTeam/FilterListManager/compare/v2.6.13...v2.7.0
[2.6.13]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.6.12...v2.6.13
[2.6.12]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.6.11...ffi-2.6.12
[2.6.11]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.6.10...ffi-2.6.11
[2.6.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.6.9...ffi-2.6.10
[2.6.9]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.6.0...ffi-2.6.9
[2.6.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.5.0...ffi-2.6.0
[2.5.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.4.0...ffi-2.5.0
[2.4.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.3.4...ffi-2.4.0
[2.3.4]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.3.1...ffi-2.3.4
[2.3.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.3.0...ffi-2.3.1
[2.3.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.2.0...ffi-2.3.0
[2.2.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.1.0...ffi-2.2.0
[2.1.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.0.1...ffi-2.1.0
[2.0.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.0.0...ffi-2.0.1
[2.0.0]: https://github.com/AdguardTeam/FilterListManager/releases/tag/ffi-2.0.0
[2.0.0-rc.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.0.0-rc.2...ffi-2.0.0-rc.3
[2.0.0-rc.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-2.0.0-rc.1...ffi-2.0.0-rc.2
[2.0.0-rc.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.9.0...ffi-2.0.0-rc.1
[1.9.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.8.0...ffi-1.9.0
[1.8.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.7.6...ffi-1.8.0
[1.7.6]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.7.5...ffi-1.7.6
[1.7.5]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.7.4...ffi-1.7.5
[1.7.4]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.7.3...ffi-1.7.4
[1.7.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.7.0...ffi-1.7.3
[1.7.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.6.3...ffi-1.7.0
[1.6.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.6.1...ffi-1.6.3
[1.6.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.10...ffi-1.6.1
[1.5.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.2...ffi-1.5.10
[1.5.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.1...ffi-1.5.2
[1.5.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.4.3...ffi-1.5.1
[1.4.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.4.1...ffi-1.4.3
[1.4.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.3.5...ffi-1.4.1
[1.3.5]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.3.1...ffi-1.3.5
[1.3.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.2.1...ffi-1.3.1
[1.2.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.21...ffi-1.2.1
[1.1.21]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.20...ffi-1.1.21
[1.1.20]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.19...ffi-1.1.20
[1.1.19]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.13...ffi-1.1.19
[1.1.13]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.10...ffi-1.1.13
[1.1.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.2...ffi-1.1.10
[1.1.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.17...ffi-1.1.2
[0.8.17]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.15...ffi-0.8.17
[0.8.15]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.13...ffi-0.8.15
[0.8.13]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.11...ffi-0.8.13
[0.8.11]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.9...ffi-0.8.11
[0.8.9]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.7...ffi-0.8.9
[0.8.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.5...ffi-0.8.7
[0.8.5]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.2...ffi-0.8.5
[0.8.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.7...ffi-0.8.2
[0.7.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.6...ffi-0.7.7
[0.7.6]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.1...ffi-0.7.6
[0.7.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.6.3...ffi-0.7.1
[0.6.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.6.0...ffi-0.6.3
[0.6.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.10...ffi-0.6.0
[0.5.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.9...ffi-0.5.10
[0.5.9]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.7...ffi-0.5.9
[0.5.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.6...ffi-0.5.7
[0.5.6]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.1...ffi-0.5.6
[0.5.1]: https://github.com/AdguardTeam/FilterListManager/releases/tag/ffi-0.5.1
