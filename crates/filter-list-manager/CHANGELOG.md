# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
