# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.2] - 2024-10-10

### Added
- Method `get_filter_rules_as_strings`

[0.7.2]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.7.1...flm-0.7.2

## [0.7.1] - 2024-09-23

### Added 
- `auto_lift_up_database: bool` - to Configuration for disabling/enabling autolifting in the constructor

### Changed
- Now flm `constructor` can throw exceptions.
- Automatic database uplifting now called in the constructor, not after very first database connection 

[0.7.1]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.6.2...flm-0.7.1

## [0.6.2] - 2024-09-23

### Added
- `get_stored_filters_metadata*` methods

[0.6.2]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.6.0...flm-0.6.2

## [0.6.0] - 2024-09-17

### Fixed

- Reduce build size by panic=abort, remove symbols for all platforms and do not bundle sqlite for apple
- `install_custom_filter_*` methods with `download_url=<empty string>` drops user rules filter when called

### Removed

- Drop `download_url` unique constraint

### Added
- New `filter_list_manager` method `lift_up_database`
- Migrations that run when the `lift_up_database` method is called
- Automatic "lift" database after the very first connection to database

[0.6.0]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.10...flm-0.6.0

### Fixed

- `save_custom_filter_rules` must update `filter.time_updated` too

## [0.5.10] - 2024-09-11

[0.5.10]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.9...flm-0.5.10

### Fixed

- `save_custom_filter_rules` must update `filter.time_updated` too

## [0.5.9] - 2024-09-09

[0.5.9]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.7...flm-0.5.9

### Fixed

- Service and custom groups should not be deleted during index update

## [0.5.7] - 2024-09-03

[0.5.7]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.6...flm-0.5.7

### Added

- Filters with the `deprecated = true` field will not be saved to the database when parsing indexes

## [0.5.6] - 2024-09-02

[0.5.6]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.5...flm-0.5.6

### Fixed

- `get_active_rules` contains empty rules if `filter.rules.disabled_rules` is empty

## [0.5.5] - 2024-08-29

[0.5.5]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.4...flm-0.5.5

### Fixed

- Filters downloading must fail then status code >= 400

## [0.5.4] - 2024-08-28

[0.5.4]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.1...flm-0.5.4

### Added

- Pre-validate filters body before parsing. i.e. html or xml documents will be rejected.
- `crate::HttpClientError::Strict200Response` - Strict 200 response for filters downloading
- `crate::FilterParserError::FilterContentIsLikelyNotAFilter` - Pre-validate filter error

## [0.5.1] - 2024-08-23

[0.5.1]: https://github.com/AdguardTeam/FilterListManager/compare/flm-0.5.0...flm-0.5.1

### Fixed

- More cleanup in readme.md
- Split changelog files by crates

## [0.5.0] - 2024-08-19

[0.5.0]: https://github.com/AdguardTeam/FilterListManager/releases/tag/flm-0.5.0

### Fixed

- Documentation of the filter-list-manager crate was cleaned up

### Added

- Add changelog
