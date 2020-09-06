# cue-sdk-sys

[![GithubRepo](https://img.shields.io/badge/github-scottroemeschke%2Fcue--sdk--sys-darkblue?style=flat)](https://github.com/scottroemeschke/cue-sdk-sys)
[![CratesIO](https://img.shields.io/crates/v/cue-sdk-sys)](https://crates.io/crates/cue-sdk-sys)
[![DocsRS](https://img.shields.io/badge/docs.rs-cue--sdk--sys-blue?style=flat)](https://docs.rs/cue-sdk-sys/latest)

The `cue-sdk-sys` crate is a low-level unsafe Rust bindings to the native [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk).

# Source Generation Process

The follow describes the steps taken to generate the source code, and what manual tweaks were made to the generated code.

1. Run bindgen with the following arguments: `bindgen <path_to_CUESDK.h> -- -x c++ -std=gnu++14`
2. Combine function calls into a single `extern` block.
3. Add `#[must_use]` to functions which return boolean "success" values.
4. Import rust FFI types.

# Building

- Download the proper iCUE SDK [release](https://github.com/CorsairOfficial/cue-sdk/releases) for your operating system.

## MacOS
- Set the environment variable `CUE_SDK_FRAMEWORK_PATH` to point to the CUESDK directory with the framework (named `CUESDK.framework`).

## Windows 
- Set the environment variable `CUE_SDK_LIB_FILES_PATH` to point to the `lib` folder of the CUESDK..
- Add the `redist` CUESDK folder to your path to develop and run end-to-end tests, and include it in your final (built) release package to the end user.