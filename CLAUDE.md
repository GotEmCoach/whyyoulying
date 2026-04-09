# whyyoulying

Proactive detection of **Labor Category Fraud** and **Ghost Billing** for DoD IG and FBI fraud investigators.
Per DoDI 5505.02/03, DoD OIG Fraud Scenarios, and Attorney General Guidelines.

## Quick reference

- Build: `cargo build --release`
- Run on fixtures: `cargo run --release -- --data-path fixtures run`
- Demo (baked-in scenarios): `cargo run --release -- demo`
- Unit tests (111): `cargo test`
- E2E tests (14, f49-f62): `cargo run --bin whyyoulying-test --features tests`
- Export DoD IG referral: `cargo run --release -- --data-path fixtures export-referral`
- Export FBI case-opening: `cargo run --release -- --data-path fixtures export-referral --fbi`

## Architecture

Two binaries, one library:
- `whyyoulying` — production CLI (default-run)
- `whyyoulying-test` — e2e binary, gated on `--features tests`
- `lib` (`whyyoulying`) — public API surface for both binaries

| Module | Purpose |
|--------|---------|
| `src/types.rs` | t5 Alert, t6 Contract, t7 Employee, t8 LaborCharge, t9 BillingRecord, t10/t11/t12 enums |
| `src/data.rs` | t3 Dataset, JSON + CSV ingest (hand-rolled, no extra deps) |
| `src/config.rs` | t1 Config, t2 ConfigError |
| `src/detect/labor.rs` | t13 LaborDetector — E4 LABOR_VARIANCE, E5 LABOR_QUAL_BELOW, E6 LABOR_RATE_OVERBILL |
| `src/detect/ghost.rs` | t14 GhostDetector — E7 GHOST_NO_EMPLOYEE, E8 GHOST_NOT_VERIFIED, E9 GHOST_BILLED_NOT_PERFORMED |
| `src/detect/time.rs` | t15 TimeDetector — E10 TIME_OVERCHARGE |
| `src/detect/duplicate.rs` | t16 DuplicateDetector — E11 DUPLICATE_BILLING |
| `src/detect/subcontractor.rs` | t22 SubcontractorDetector — E16 SUB_BILLED_AS_PRIME |
| `src/detect/rate_escalation.rs` | t23 RateEscalationDetector — E17 RATE_ESCALATION_TREND |
| `src/export/mod.rs` | f18 referral_package (GAGAS), f19 fbi_case_opening, f27 FNV-1a chain-of-custody hash |
| `src/demo.rs` | Baked-in fraud scenarios for federal investigator demos |
| `src/main.rs` | clap CLI, output formatting (json/csv) |
| `src/util.rs` | f20 timestamp helper |
| `src/tests.rs` | f30 e2e harness, f49-f62 cases |
| `src/bin/whyyoulying-test.rs` | e2e binary entry point |
| `src/lib.rs` | Public re-exports (Alert, Config, Dataset, *Detector, FraudType, RuleId) |
| `src/android_jni.rs` | JNI bindings for Android AAB |

## Detection rules — current (10 total)

The first 8 implement DoDI 5505.02 Enclosure 3 fraud indicators. Rules 9-10 extend to subcontractor and rate-trend fraud per DCAM.

| # | Token | Rule ID | Type | Source |
|---|-------|---------|------|--------|
| 1 | E4 | LABOR_VARIANCE | Labor | DoDI 5505.02 Encl 3 §1 |
| 2 | E5 | LABOR_QUAL_BELOW | Labor | DoDI 5505.02 Encl 3 §1 |
| 3 | E6 | LABOR_RATE_OVERBILL | Labor | DoDI 5505.02 Encl 3 §1 |
| 4 | E7 | GHOST_NO_EMPLOYEE | Ghost | DoDI 5505.02 Encl 3 §2 |
| 5 | E8 | GHOST_NOT_VERIFIED | Ghost | DoDI 5505.02 Encl 3 §2 |
| 6 | E9 | GHOST_BILLED_NOT_PERFORMED | Ghost | DoDI 5505.02 Encl 3 §2 |
| 7 | E10 | TIME_OVERCHARGE | Ghost | DoDI 5505.02 Encl 3 §2 |
| 8 | E11 | DUPLICATE_BILLING | Labor | DoDI 5505.02 Encl 3 §1 |
| 9 | E16 | SUB_BILLED_AS_PRIME | Subcontractor | DCAM 6-414 |
| 10 | E17 | RATE_ESCALATION_TREND | Trend | DCAM 6-606 |

## Code state

| Metric | Value |
|--------|-------|
| Lines of Rust | 3,468 |
| Source files | 18 |
| Unit tests | 111 |
| E2E tests | 14 (f49-f62) |
| Direct deps | 6 (anyhow, clap, serde, serde_json, tempfile, thiserror) |
| Test deps | +2 (exopack, tokio, both feature-gated on `tests`) |
| Release binary (macOS ARM) | 637 KB |
| Rust edition | 2024 |

## Conventions

- P13 compression: all public symbols are tokenized (`f1`-`f30`, `t1`-`t23`, `s1`-`s72`, `E1`-`E17`). See `docs/compression_map.md`.
- Serde rename layer keeps the wire format human-readable: `t10::E2 → "labor_category"`, `t11::E4 → "LABOR_VARIANCE"`, etc.
- Chain-of-custody hashes use FNV-1a (`f27`) — deterministic across platforms and Rust versions. `DefaultHasher` is forbidden here for legal reasons.
- Estimated loss (`s66`) is computed by detectors when contract rate is known and attached to the alert.
- DoD nexus filters (`--agency`, `--cage-code`) work across cross-entity alerts (TIME_OVERCHARGE, DUPLICATE_BILLING) by joining through related contracts.
- Output formats: pretty JSON (default), CSV (`--output csv`).
- Exit code: 0 = no alerts, 1 = alerts found, 2 = error.

## Testing model

- **Unit tests** live next to the code in `#[cfg(test)] mod tests`. 111 cases as of last commit. Run with `cargo test`.
- **E2E tests** are in `src/tests.rs`, exposed via `whyyoulying-test` binary. They build the release binary and shell out to it. 14 cases (f49-f62). Run with `cargo run --bin whyyoulying-test --features tests`.
- **No external test framework.** No `tokio::test`, no `rstest`. The test binary is the CI pipeline (P16).
- **No mocks for the database** — fixtures are real files in `tempfile::TempDir`.

## Don'ts

- Don't add new direct deps without weighing binary size impact (zero-dep posture is a value).
- Don't reintroduce `DefaultHasher` for chain-of-custody hashing — use `f27` (FNV-1a).
- Don't break the tokenized public API (`f`/`t`/`s`/`E` symbols). Re-exports in `src/lib.rs` are stable surface.
- Don't add `#[allow(...)]` without justification (P12).

## Federal compliance

All compliance docs live in `govdocs/` and are baked into the binary. List at runtime:
```
whyyoulying govdocs <doc>     # SBOM, FIPS, CMMC, ITAR_EAR, etc.
whyyoulying --sbom            # Print SPDX 2.3 SBOM and exit
```
