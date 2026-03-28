# NIST SP 800-218 SSDF Compliance — whyyoulying

Secure Software Development Framework mapping.

## PS: Prepare the Organization

| Practice | Status | Evidence |
|----------|--------|----------|
| PS.1 Define security requirements | Done | Detection rules mapped to DoDI 5505.02/03, AG Guidelines (docs/USER_STORY_ANALYSIS.md) |
| PS.2 Implement roles and responsibilities | Done | Unlicense + contributor attribution in file headers |
| PS.3 Implement supporting toolchains | Done | Rust compiler, clippy linter, cargo test, TRIPLE SIMS gate |

## PW: Protect the Software

| Practice | Status | Evidence |
|----------|--------|----------|
| PW.1 Design software to meet security requirements | Done | No network access in release binary. Reads local files only. No unsafe code blocks |
| PW.2 Review software design | Done | TRIPLE_SIMS_ARCH.md documents domain model, pipeline, phases |
| PW.4 Reuse existing well-secured software | Done | All deps from crates.io with MIT/Apache-2.0 licenses. No custom crypto |
| PW.5 Create source code following secure practices | Done | No SQL (sled/JSON only), no eval, no shell exec in library. Input validated at CLI boundary (clap validators) |
| PW.6 Configure compilation to reduce vulnerabilities | Done | panic=abort, LTO, strip=true in release profile. No debug info in release binary |
| PW.7 Review/analyze human-readable code | Done | cargo clippy --release -- -D warnings enforced. Zero warnings |
| PW.8 Test executable code | Done | 67 unit tests + 12 E2E tests (f49-f60). TRIPLE SIMS: 3 consecutive passes required |
| PW.9 Configure software to have secure settings by default | Done | Default threshold 15%, default min_confidence 50, no network listeners |

## RV: Respond to Vulnerabilities

| Practice | Status | Evidence |
|----------|--------|----------|
| RV.1 Identify and confirm vulnerabilities | Partial | cargo audit not yet configured. Manual dep review. Cargo.lock pins versions |
| RV.2 Assess and prioritize vulnerabilities | N/A | No known vulnerabilities in current deps |
| RV.3 Remediate vulnerabilities | Done | chrono dependency removed (reduced attack surface). Zero optional features enabled by default |

## PO: Protect Operations

| Practice | Status | Evidence |
|----------|--------|----------|
| PO.1 Provision and decommission software securely | Done | Single static binary. No installer. No registry entries. No services. Delete binary to decommission |
| PO.2 Protect data | Done | No data persistence. Reads input, writes stdout/file, exits. No temp files in release path |
| PO.3 Monitor software | N/A | CLI tool, not a service. No daemon mode. No telemetry |
