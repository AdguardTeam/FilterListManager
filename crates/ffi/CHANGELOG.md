# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [0.8.25] - 2024-12-06

### Fixed
- Fix diffupdates lines count for files without "\n" on end

[0.8.25]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.24...ffi-0.8.25

## [0.8.24] - 2024-12-04

### Fixed
- Diffupdates now respects trailing newlines + fix checksum validator

[0.8.24]: https://github.com/AdguardTeam/FilterListManager/compare/ffi-0.8.17...ffi-0.8.24

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
