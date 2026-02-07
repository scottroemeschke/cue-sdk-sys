[![CI](https://github.com/scottroemeschke/cue-sdk-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/scottroemeschke/cue-sdk-sys/actions/workflows/ci.yml)
[![CratesIO](https://img.shields.io/crates/v/cue-sdk-sys)](https://crates.io/crates/cue-sdk-sys)
[![DocsRS](https://img.shields.io/badge/docs.rs-cue--sdk--sys-blue?style=flat)](https://docs.rs/cue-sdk-sys/latest)

# cue-sdk-sys

Low-level unsafe Rust FFI bindings to the [Corsair iCUE SDK](https://github.com/CorsairOfficial/cue-sdk) v4.

This workspace contains:
- `cue-sdk-sys/` — the published sys crate
- `sys-test/` — integration test crate that validates FFI linkage against the native SDK

## Releasing

Releases are triggered by pushing a version tag. The CI workflow will create a GitHub Release (with notes extracted from `CHANGELOG.md`) and publish to crates.io.

```bash
# Use the helper script:
./scripts/release.sh 0.1.0

# Or manually:
git tag v0.1.0
git push origin v0.1.0
```

Tag pushes are restricted to the repository owner via GitHub tag protection rules.