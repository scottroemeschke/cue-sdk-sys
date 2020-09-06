[![PlatformAgnostic-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/Platform-Agnostic-CI/badge.svg?branch=master)](https://github.com/scottroemeschke/cue-sdk-sys/actions?query=workflow%3APlatform-Agnostic-CI)
[![Windows-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/.github/workflows/windows_ci.yml/badge.svg)](https://github.com/scottroemeschke/cue-sdk-sys/actions?query=workflow%3AWindows-CI)
[![MacOS-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/MacOS-CI/badge.svg?branch=master)](https://github.com/scottroemeschke/cue-sdk-sys/actions?query=workflow%3AMacOS-CI)

[![GithubRepo](https://img.shields.io/badge/github-scottroemeschke%2Fcue--sdk--sys-darkblue?style=flat)](https://github.com/scottroemeschke/cue-sdk-sys)
[![CratesIO](https://img.shields.io/crates/v/cue-sdk-sys)](https://crates.io/crates/cue-sdk-sys)
[![DocsRS](https://img.shields.io/badge/docs.rs-cue--sdk--sys-blue?style=flat)](https://docs.rs/cue-sdk-sys/latest)

# Repo and Workspace Information

The `cue-sdk-sys` crate is a low-level unsafe Rust bindings to the native [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk).
This workspace contains the `cue-sdk-sys` crate itself, and a sanity integration test crate (`sys-test`) that performs a handshake with the SDK.