# Proof of Artifacts — whyyoulying

Verifiable metrics from the current build. Updated 2026-03-27.

## Binary

| Metric | Value |
|--------|-------|
| Release binary | 586,176 bytes (572 KB) |
| Target | aarch64-apple-darwin (ARM) |
| Profile | opt-level=z, lto=true, codegen-units=1, panic=abort, strip=true |
| Deps removed | chrono (replaced with 30-line std::time utility) |
| Previous size | 1,331,360 bytes (1.3 MB) |
| Reduction | 56% |

## Test Results

| Suite | Count | Result |
|-------|-------|--------|
| cargo test (unit) | 67 | All pass |
| whyyoulying-test (e2e) | 12 | f49-f60 all pass |
| TRIPLE SIMS | 3/3 | All passes OK |
| Total | 79 | |

### QA Round 1 (2026-03-27)

| Check | Result |
|-------|--------|
| Compiles | Zero errors |
| Warnings | Zero (clippy -D warnings) |
| Bugs | Fixed nexus filter for cross-entity alerts |
| Existing tests | All pass |
| Code clean | No debug prints, no TODOs, no slop |
| Ship today? | PASS |

### QA Round 2 (2026-03-27)

| Check | Result |
|-------|--------|
| cargo clean + build | Zero errors, zero warnings (43s) |
| clippy -D warnings | Zero |
| TRIPLE SIMS | 3/3 |
| git status | Clean, up to date |
| Last commit | Makes sense |
| Verdict | PASS |

## Code Metrics

| Metric | Value |
|--------|-------|
| Lines of Rust | 3,025 |
| Source files | 15 (.rs) |
| Public functions | 21 |
| Public types | 21 |
| Public fields | 65 |
| Detection rules | 8 |
| Direct dependencies | 6 (release) |
| Test dependencies | +2 (exopack, tokio) |

## P13 Tokenization

| Category | Count |
|----------|-------|
| Functions (f1-f20, f30) | 21 |
| Types (t1-t21) | 21 |
| Fields (s1-s65) | 65 |
| Enum variants (E1-E14) | 14 |
| CLI commands (c1-c3) | 3 |
| Total symbols | 124 |

## Detection Rules

| # | Rule | Type | Since |
|---|------|------|-------|
| 1 | LABOR_VARIANCE | Labor | v0.2.0 (a90714b) |
| 2 | LABOR_QUAL_BELOW | Labor | v0.2.0 (a90714b) |
| 3 | GHOST_NO_EMPLOYEE | Ghost | v0.2.0 (a90714b) |
| 4 | GHOST_NOT_VERIFIED | Ghost | v0.2.0 (a90714b) |
| 5 | GHOST_BILLED_NOT_PERFORMED | Ghost | v0.2.0 (a90714b) |
| 6 | LABOR_RATE_OVERBILL | Labor | beded33 |
| 7 | TIME_OVERCHARGE | Ghost | bf4f32f |
| 8 | DUPLICATE_BILLING | Labor | bf4f32f |

## Federal Compliance Docs

| Document | Standard |
|----------|----------|
| SBOM.md | EO 14028 |
| SSDF.md | NIST SP 800-218 |
| SECURITY.md | General posture |
| PRIVACY.md | PIA |
| FIPS.md | FIPS 140-2/3 |
| FedRAMP_NOTES.md | FedRAMP |
| CMMC.md | CMMC L1-2 |
| SUPPLY_CHAIN.md | Supply chain integrity |
| ITAR_EAR.md | EAR/ITAR |
| ACCESSIBILITY.md | Section 508 |
| FEDERAL_USE_CASES.md | Agency mapping |

## Dependencies (release binary)

| Crate | Version | License |
|-------|---------|---------|
| anyhow | 1.0.102 | MIT/Apache-2.0 |
| clap | 4.5.60 | MIT/Apache-2.0 |
| serde | 1.0.228 | MIT/Apache-2.0 |
| serde_json | 1.0.149 | MIT/Apache-2.0 |
| tempfile | 3.26.0 | MIT/Apache-2.0 |
| thiserror | 1.0.69 | MIT/Apache-2.0 |
