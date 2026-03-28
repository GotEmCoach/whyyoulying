# Software Bill of Materials — whyyoulying

Per EO 14028 (Improving the Nation's Cybersecurity), Section 4.

**Generated:** 2026-03-27
**Binary:** whyyoulying v0.2.0
**Language:** Rust (edition 2021)
**License:** Unlicense

## Direct Dependencies (release binary)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| anyhow | 1.0.102 | MIT OR Apache-2.0 | Error handling with context |
| clap | 4.5.60 | MIT OR Apache-2.0 | CLI argument parsing (derive) |
| serde | 1.0.228 | MIT OR Apache-2.0 | Serialization framework |
| serde_json | 1.0.149 | MIT OR Apache-2.0 | JSON parsing and output |
| tempfile | 3.26.0 | MIT OR Apache-2.0 | Temp files for unit tests (not in release path) |
| thiserror | 1.0.69 | MIT OR Apache-2.0 | Error type derivation |

## Test-Only Dependencies (feature-gated)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| exopack | 0.1.0 (git) | Proprietary | TRIPLE SIMS test harness |
| tokio | 1.50.0 | MIT | Async runtime for test binary |

## Transitive Dependencies (notable)

| Crate | License | Pulled by |
|-------|---------|-----------|
| syn | MIT OR Apache-2.0 | serde_derive, clap_derive |
| proc-macro2 | MIT OR Apache-2.0 | syn |
| itoa | MIT OR Apache-2.0 | serde_json |
| memchr | MIT OR Apache-2.0 | clap |
| rustix | MIT OR Apache-2.0 | tempfile |

## Verification

All dependencies sourced from crates.io (HTTPS). Versions pinned in `Cargo.lock`.
No vendored binaries. No C/C++ dependencies. No pre-built .so/.dylib files.
Full dependency tree: `cargo tree` from project root.

## License Compatibility

All dependencies are dual-licensed MIT OR Apache-2.0, compatible with the project's Unlicense.
No GPL, AGPL, or copyleft dependencies.
