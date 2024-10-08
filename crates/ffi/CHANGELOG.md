# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

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
