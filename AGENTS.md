# AGENTS.md — AI Agent Instructions for cue-sdk-sys

## Project Overview

`cue-sdk-sys` is a Rust `-sys` crate providing raw FFI bindings to the [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk) v4.0.84. It is a workspace containing:

- `cue-sdk-sys/` — the published sys crate (v0.1.0)
- `sys-test/` — integration test crate (requires native SDK libs + running iCUE)

The SDK headers live in a sibling directory: `../cue-sdk/src/include/`

## Build & Check Commands

```bash
# Check compilation (works without native libs)
cargo check

# Run rustfmt
cargo fmt --all -- --check

# Run clippy
cargo clippy -- -D warnings

# Run tests (requires native SDK libs and iCUE running — will fail on Linux)
cargo test -p sys-test
```

`cargo check` and `cargo clippy` work on any platform since the build script only emits link directives on Windows/macOS. Tests in `sys-test` require the actual native library and iCUE running.

## Code Style & Conventions

- **Edition 2021**, workspace resolver v2
- FFI types imported from `core::ffi` (not `std::os::raw`)
- All `#[allow]` lints: `non_upper_case_globals`, `non_snake_case`, `non_camel_case_types`
- Enum-style types use `pub type Foo = c_uint;` with `pub const Foo_Variant: Foo = N;` (bindgen convention)
- Structs are `#[repr(C)]` with `#[derive(Debug, Copy, Clone)]` where possible (`Copy`/`Clone` only; `Debug` omitted for unions)
- Layout assertions use compile-time `const _: () = { ... };` blocks with `std::mem::offset_of!`
- The anonymous union in `CorsairEvent` is manually renamed to `CorsairEventUnion`

## Safety Notes

- `unsafe impl Send/Sync` is applied to all types containing raw pointers — this is justified by the iCUE SDK documentation stating the SDK is thread-safe
- All extern functions are `#[must_use]` since they return `CorsairError`
- All extern functions are in a single `unsafe extern "C" { ... }` block

## Regenerating Bindings

If SDK headers change:

1. Create a wrapper header including both `iCUESDK.h` and `iCUESDKLedIdEnum.h`
2. Run: `bindgen wrapper.h -- -x c++ -std=gnu++14 -I <path_to_sdk_include>`
3. Post-process per the steps documented in `cue-sdk-sys/README.md`

## Key Files

| File | Purpose |
|------|---------|
| `cue-sdk-sys/src/lib.rs` | All FFI bindings (generated + post-processed) |
| `cue-sdk-sys/build.rs` | Native library linking (Windows: `iCUESDK.x64_2019`, macOS: `iCUESDK` framework) |
| `cue-sdk-sys/Cargo.toml` | Crate metadata, `links = "iCUESDK"` |
| `cue_sdk_version` | Tracks SDK version (`CUE_SDK_VERSION=4.0.84`) |
| `sys-test/src/lib.rs` | Integration test using `CorsairGetSessionDetails` |

## Environment Variables

- `CUE_SDK_LIB_FILES_PATH` — (Windows) path to directory containing `iCUESDK.x64_2019.lib`
- `CUE_SDK_FRAMEWORK_PATH` — (macOS) path to directory containing `iCUESDK.framework`
