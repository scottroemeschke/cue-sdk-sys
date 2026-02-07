# cue-sdk-sys

[![GithubRepo](https://img.shields.io/badge/github-scottroemeschke%2Fcue--sdk--sys-darkblue?style=flat)](https://github.com/scottroemeschke/cue-sdk-sys)
[![CratesIO](https://img.shields.io/crates/v/cue-sdk-sys)](https://crates.io/crates/cue-sdk-sys)
[![DocsRS](https://img.shields.io/badge/docs.rs-cue--sdk--sys-blue?style=flat)](https://docs.rs/cue-sdk-sys/latest)

The `cue-sdk-sys` crate is a low-level unsafe Rust bindings to the native [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk) **v4.0.84**.

**Note:** v0.1.0 is a breaking change from v0.0.x. The iCUE SDK v4 is a complete API rewrite — the device-index-based API has been replaced with a session/device-ID-based API, and the native library was renamed from `CUESDK` to `iCUESDK`.

# Source Generation Process

The following describes the steps taken to generate the source code, and what manual tweaks were made to the generated code.

1. Create a wrapper header that includes both `iCUESDK.h` and `iCUESDKLedIdEnum.h`.
2. Run bindgen: `bindgen wrapper.h -- -x c++ -std=gnu++14 -I <path_to_sdk_include>`
3. Combine function calls into a single `unsafe extern "C"` block.
4. Add `#[must_use]` to all extern functions (they all return `CorsairError`).
5. Import Rust FFI types from `core::ffi` instead of `std::os::raw`.
6. Add `Send` and `Sync` traits to non-auto send/sync structs since the iCUE SDK documentation shares that the SDK is thread-safe.
7. Rename the anonymous union in `CorsairEvent` to `CorsairEventUnion`.

# Building

- Download the proper iCUE SDK [release](https://github.com/CorsairOfficial/cue-sdk/releases) for your operating system.
- **Note:** v4 only ships x64 libraries (i386 is no longer supported).

## macOS
- Set the environment variable `CUE_SDK_FRAMEWORK_PATH` to point to the directory containing `iCUESDK.framework`.

## Windows
- Set the environment variable `CUE_SDK_LIB_FILES_PATH` to point to the `lib/x64` folder of the iCUE SDK.
- Add the `redist` iCUE SDK folder to your path to develop and run end-to-end tests, and include it in your final (built) release package to the end user.
