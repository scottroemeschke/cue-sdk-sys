![PlatformAgnostic-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/Platform-Agnostic-CI/badge.svg?branch=master)
![Windows-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/Windows-CI/badge.svg?branch=master)
![MacOS-CI](https://github.com/scottroemeschke/cue-sdk-sys/workflows/MacOS-CI/badge.svg?branch=master)

![GithubRepo](https://img.shields.io/badge/github-scottroemeschke%2Fcue--sdk--sys-darkblue?style=flat)
![CratesIO](https://img.shields.io/crates/v/cue-sdk-sys)
![DocsRS](https://img.shields.io/badge/docs.rs-cue--sdk--sys-blue?style=flat)

# Repo and Workspace Information

The `cue-sdk-sys` crate is a low-level unsafe Rust bindings to the native [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk).
This workspace contains the `cue-sdk-sys` crate itself, and a sanity integration test crate (`sys-test`) that performs a handshake with the SDK.