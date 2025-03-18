# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [1.6.1] - 2025-03-17

### Added
- `get_rules_count` method for getting rules count by filter ids

[1.6.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.10...ffi-1.6.1

## [1.5.10] - 2025-03-14

### Fixed
- Suggest fallback locale in `change_locale`

[1.5.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.2...ffi-1.5.10

## [1.5.2] - 2025-03-11

### Fixed
- OR expressions in BooleanExpressionParser

[1.5.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.5.1...ffi-1.5.2

## [1.5.1] - 2025-03-06

### Added
- Add client app name and version in configuration

[1.5.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.4.3...ffi-1.5.1

## [1.4.3] - 2025-03-06

### Added
- `get_active_rules` method for apple platform

[1.4.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.4.1...ffi-1.4.3

## [1.4.1] - 2025-02-24

### Added
- `fetch_filter_list_metadata_with_body` method for fetch filter metadata with filter body

[1.4.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.3.5...ffi-1.4.1

## [1.3.5] - 2025-02-10

### Added
- `flm_get_constants` method for the new native API

[1.3.5]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.3.1...ffi-1.3.5

## [1.3.1] - 2025-01-31

### Changed
- Update to flm 1.3

[1.3.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.2.1...ffi-1.3.1

## [1.2.1] - 2025-01-24

### Added
- Tries to normalize slightly malformed filter urls
- Proxy mode in configuration and flm interface

### Fixed
- Speed up http clients

[1.2.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.21...ffi-1.2.1

## [1.1.21] - 2025-01-17

### Fixed
- Unnecessary filter rules selection in `save_disabled_rules`
- `file:///` urls support in `IndexesParser`

[1.1.21]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.20...ffi-1.1.21

## [1.1.20] - 2024-12-19

### Added
- Static CRT link in windows libraries
- Windows .rc file

[1.1.20]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.19...ffi-1.1.20

## [1.1.19] - 2024-12-19

### Fixed
- File checksum should respect file newline
- Checksums will be checked only for index filters
- Install custom list is not setting last download time

[1.1.19]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.13...ffi-1.1.19

## [1.1.13] - 2024-12-06

### Fixed
- Fix diffupdates lines count for files without "\n" on end

[1.1.13]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-1.1.10...flm-1.1.13

## [1.1.10] - 2024-12-04

### Fixed
- Diffupdates now respects trailing newlines + fix checksum validator

[1.1.10]: https://github.com/AdguardTeam/FilterListManager/compare/flm-1.1.2...flm-1.1.10

## [1.1.2] - 2024-11-18

### Added
- Protobuf-based ffi
- `flm_default_configuration_protobuf` as default `Configuration` object
- `flm_init_protobuf` as new `FLM` constructor
- `flm_call_protobuf` as `FLM` methods caller
- `flm_get_constants` as library constants holder
- `flm_free_handle` as cleanup handler for `FLM Handle`
- `flm_free_response` as cleanup handler for `RustReponse`

### Changed
- `FilterId` type changed from `i64` to `i32`
- Uniffi interface dropped for apple build by default
- Uniffi build for windows is obsolete now

### Fixed
- `get_database_path` returns the absolute path to the database, even if a relative path was specified in the configuration

### Removed
- `Configuration.encryption_key` key removed
- `get_full_filter_lists` method

[1.1.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.17...ffi-1.1.2

## [0.8.17] - 2024-11-07

### Fixed
- Fixed all problems in `update_filters` and write tests

[0.8.17]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.13...ffi-0.8.15

## [0.8.15] - 2024-11-07

### Fixed
- Fix SQL error in update_filters

[0.8.15]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.13...ffi-0.8.15

## [0.8.15] - 2024-11-06

### Fixed
- Fix where clause for empty entities list

[0.8.13]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.11...ffi-0.8.13

## [0.8.11] - 2024-11-05

### Fixed
- fixed `flm-0.8.5` release 

[0.8.11]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.9...ffi-0.8.11

## [0.8.9] - 2024-11-05

### Fixed
- Disabled rules of filters were removed after filters update

### Added
- `get_disabled_rules` method

[0.8.9]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.7...ffi-0.8.9

## [0.8.7] - 2024-11-01

### Fixed
- `file:` protocol-based paths are being decoded the right way i.e. `Path%20With%20Spaces`
- Now when filters are updated their versions are checked

[0.8.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.5...ffi-0.8.7

## [0.8.5] - 2024-10-29

### Added
- Method `save_rules_to_file_blob` for incremental writing filter rules to file

[0.8.5]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.2...ffi-0.8.5

## [0.8.2] - 2024-10-25

### Changed
- Db queries are now executed through mutex queue
- Change mutexes at ffi to r/w lock, which w locks only for change_locale

### Added
- `DatabaseBusy` error code

[0.8.2]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.7...ffi-0.8.2

## [0.7.7] - 2024-10-10

### Fixed
- Method `get_filter_rules_as_strings` wasn't exported in previous version 

[0.7.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.6...ffi-0.7.7

## [0.7.6] - 2024-10-10

### Added
- Method `get_filter_rules_as_strings`

[0.7.6]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.7.1...ffi-0.7.6

## [0.7.1] - 2024-09-23

### Changed
- Now flm `constructor` can throw exceptions.
- Upd `flm` dependency to `0.7`

[0.7.1]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.6.3...ffi-0.7.1

## [0.6.3] - 2024-09-23

### Added
- `get_stored_filters_metadata*` methods

### Changed
- Reduced apple framework build size

[0.6.3]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.6.0...ffi-0.6.3

## [0.6.0] - 2024-09-17

### Fixed
- Fix windows rust lib and windows adapter build
- Reduce build size by panic=abort, remove symbols for all platforms and do not bundle sqlite for apple

### Added
- `flm.lift_up_database` method

### Changed
- Update `adguard-flm` to `0.6`

[0.6.0]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.10...ffi-0.6.0

## [0.5.10] - 2024-09-03

[0.5.10]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.9...ffi-0.5.10

### Fixed
- Change flm version restrictions

## [0.5.9] - 2024-09-02

[0.5.9]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.7...ffi-0.5.9

### Changed

- Upd flm to 0.5.6

## [0.5.7] - 2024-08-29

[0.5.7]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.6...ffi-0.5.7

### Changed

- Upd flm to 0.5.5

## [0.5.6] - 2024-08-28

[0.5.6]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.5.1...ffi-0.5.6

### Added

- Pre-validate filters body before parsing. i.e. html or xml documents will be rejected.
- `HttpStrict200Response` error, if filter downloading response has success code but not 200
- `FilterContentIsLikelyNotAFilter` Pre-validate filter error

## [0.5.1] - 2024-08-23

[0.5.1]: https://github.com/AdguardTeam/FilterListManager/releases/tag/ffi-0.5.1

### Added

- Add changelog
