# Supply Chain Integrity — whyyoulying

## Dependency Sources

All dependencies sourced from **crates.io** over HTTPS. No private registries.
No git dependencies in the release binary (exopack is test-only, feature-gated).

## Version Pinning

`Cargo.lock` committed to repository. Exact versions of all transitive dependencies pinned.
Builds are reproducible from the same lockfile + Rust toolchain version.

## Build Reproducibility

- **Profile:** `opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = 'abort'`, `strip = true`
- **No build scripts:** No `build.rs` files. No code generation at build time.
- **No proc macros with side effects:** Only serde_derive, clap_derive, thiserror-impl (all widely audited).
- **No network access at build time:** No downloads during compilation.

## Binary Integrity

- Release binary is statically linked (no dynamic library dependencies beyond system libc).
- `strip = true` removes debug symbols. Binary contains no source paths.
- Binary size: 635,920 bytes (621 KB). Minimal attack surface.

## No Vendored Binaries

- No pre-compiled .so, .dylib, .dll, or .wasm files in repository.
- No checked-in archives (.tar, .zip, .jar).
- All code compiles from Rust source.

## Source Availability

Full source on GitHub: https://github.com/GotEmCoach/whyyoulying
License: Unlicense (public domain equivalent). Full source audit possible.
