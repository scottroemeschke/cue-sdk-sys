# CHANGELOG

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.0.1]

Initial release for CUE SDK version [3.0.355](https://github.com/CorsairOfficial/cue-sdk/releases/tag/v3.0.355).

## [v0.0.2]
### Changed
- (BREAKING CHANGE) Renamed union field in `CorsairEvent` from a generated `bindgen` name, to `CorsairEventUnion`.

## [v0.0.3]
### Fixed
- Updated README badge links.

## [v0.0.4]
### Changed
- All structs (including those with pointers and not just values) are now `Send` and `Sync` as suggested from the iCUE SDK documentation.

## [v0.0.5]
### Changed
- Updated CUE SDK to version [3.0.361](https://github.com/CorsairOfficial/cue-sdk/releases/tag/v3.0.361).
