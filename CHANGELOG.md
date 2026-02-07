# CHANGELOG

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.0]
### Changed
- (BREAKING CHANGE) Updated to iCUE SDK v4.0.84. This is a complete API rewrite:
  - Native library renamed from `CUESDK` to `iCUESDK`.
  - New session-based connection model (`CorsairConnect`/`CorsairDisconnect`/`CorsairGetSessionDetails`).
  - Device-index-based API replaced with device-ID-based API.
  - `CorsairLedColor` now uses `CorsairLedLuid` instead of `CorsairLedId`, and includes alpha channel.
  - Many new types: `CorsairSessionState`, `CorsairSessionDetails`, `CorsairVersion`, `CorsairDeviceFilter`, `CorsairAccessLevel`, `CorsairDataType`, `CorsairProperty`, `CorsairDataValue`, `CorsairKeyEventConfiguration`, `CorsairLedGroup`, etc.
  - `CorsairDeviceType` is now a bitmask (was sequential enum).
  - LED IDs reorganized into `CorsairLedGroup`-based enums (e.g., `CorsairLedId_Keyboard`).
  - Dropped i386 support (x64 only).
- Updated Rust edition from 2018 to 2021.
- Switched FFI type imports from `std::os::raw` to `core::ffi`.
- Updated layout assertions to use modern `offset_of!` macro.
- Improved build.rs error messages (panic with actionable message instead of silent wrong string).
- Added `cargo:rerun-if-env-changed` directives to build.rs.
- All extern functions now return `CorsairError` and have `#[must_use]`.

### Removed
- All v3-specific types and functions (e.g., `CorsairPerformProtocolHandshake`, `CorsairGetDeviceCount`, `CorsairGetLastError`, `CorsairSetLedsColors`, `CorsairProtocolDetails`, `CorsairChannelInfo`, `CorsairChannelsInfo`, `CorsairLedPositions`, `CorsairDeviceCaps`, `CorsairAccessMode`, `CorsairKeyId`, etc.).

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
