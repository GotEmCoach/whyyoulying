<p align="center">
  <img src="https://raw.githubusercontent.com/GotEmCoach/whyyoulying/main/assets/logo.svg" alt="whyyoulying" width="64">
</p>

# whyyoulying

Proactive detection of **Labor Category Fraud** and **Ghost Billing** for DoD IG and FBI fraud investigators.

Per DoDI 5505.02/03, DoD OIG Fraud Scenarios, and Attorney General Guidelines.

Single binary. 6 dependencies. 637 KB. 10 detection rules. 111 unit tests + 14 e2e tests. Zero external services.

---

## Architecture

```mermaid
flowchart LR
    Data[fixtures/] --> Ingest[ingest]
    Ingest --> Run[run]
    Run --> LD[LaborDetector]
    Run --> GD[GhostDetector]
    Run --> TD[TimeDetector]
    Run --> DD[DuplicateDetector]
    Run --> SD[SubcontractorDetector]
    Run --> RE[RateEscalationDetector]
    LD --> Alerts[Alerts]
    GD --> Alerts
    TD --> Alerts
    DD --> Alerts
    SD --> Alerts
    RE --> Alerts
    Alerts --> Export[export-referral]
    Alerts --> FBI[FBI case-opening]
```

---

## Supported Platforms

| Target | Arch | Status | Size |
|--------|------|--------|------|
| macOS ARM | aarch64-apple-darwin | Release binary | 637 KB |
| macOS Intel | x86_64-apple-darwin | Release binary | 671 KB |
| Linux x86_64 | x86_64-unknown-linux-gnu | Release binary | 764 KB |
| Android | aarch64-linux-android | AAB (JNI + WebView) | 213 KB |
| Linux ARM64 | aarch64-unknown-linux-gnu | Cross (needs `cross`) | — |
| Linux ARM32 | armv7-unknown-linux-gnueabihf | Cross (needs `cross`) | — |
| Windows x64 | x86_64-pc-windows-gnu | Cross (needs `cross`) | — |
| FreeBSD x64 | x86_64-unknown-freebsd | Cross (needs `cross`) | — |
| RISC-V 64 | riscv64gc-unknown-linux-gnu | Cross (needs `cross`) | — |
| IBM POWER | powerpc64le-unknown-linux-gnu | Cross (needs `cross`) | — |
| iOS | aarch64-apple-ios | Library only | — |
| WebAssembly | wasm32-unknown-unknown | Library only | — |

Build all: `./scripts/build-all-targets.sh`

---

## Quick Start

```bash
# Build
cargo build --release

# Run fraud detection demo (baked-in sample contracts)
cargo run --release -- demo

# Run detection on your own data
cargo run --release -- --data-path fixtures run

# Export FBI case-opening document
cargo run --release -- --data-path fixtures export-referral --fbi

# Print SPDX SBOM
cargo run --release -- --sbom

# Run unit tests (111 tests)
cargo test

# Run integration tests (f49-f62, 14 cases)
cargo run --bin whyyoulying-test --features tests
```

---

## Usage

| Command | Description |
|---------|-------------|
| `run` | Load data, run all detectors, output alerts (default) |
| `ingest` | Load and validate data only |
| `export-referral` | Export GAGAS referral package for DoD IG |
| `export-referral --fbi` | Export FBI case-opening per AG Guidelines |
| `demo` | Run detection on baked-in sample contracts (text, json, or html) |
| `govdocs` | Print federal compliance docs (sbom, fips, cmmc, etc.) |

### Options

| Flag | Description |
|------|-------------|
| `--data-path PATH` | Directory with contracts.json, employees.json, labor_charges.json, billing_records.json |
| `--config PATH` | Config file (labor_variance_threshold_pct, min_confidence) |
| `--threshold PCT` | Labor variance threshold 0-100 (default 15) |
| `--min-confidence 0-100` | Filter alerts below confidence (S4 false-positive control) |
| `--min-loss N` | Filter alerts below estimated loss (USD) |
| `--agency AGENCY` | DoD nexus: filter by agency (e.g. DoD, Army) |
| `--cage-code CODE` | DoD nexus: filter by CAGE code |
| `--output json\|csv` | Output format |
| `--sbom` | Print SPDX 2.3 SBOM and exit |

### Exit Codes

- `0` — No alerts
- `1` — Alerts found
- `2` — Error

---

## Data Format

Place JSON or CSV files in `--data-path` (JSON preferred; CSV ingest is hand-rolled, no extra deps):

- `contracts.json` — id, cage_code, agency, labor_cats, labor_rates
- `employees.json` — id, quals, labor_cat_min, verified, is_subcontractor
- `labor_charges.json` — contract_id, employee_id, labor_cat, hours, rate, period
- `billing_records.json` — contract_id, employee_id, billed_hours, billed_cat, period

See `fixtures/` for examples.

---

## Detection Rules

The first 8 rules implement DoDI 5505.02 Enclosure 3 fraud indicators. Rules 9-10 extend to subcontractor and rate-trend fraud per DCAA Contract Audit Manual guidance.

| # | Rule ID (E#) | Type | Description | DoDI 5505.02 |
|---|--------------|------|-------------|--------------|
| 1 | LABOR_VARIANCE (E4) | Labor | Labor category billed not in contract | Encl 3 §1 |
| 2 | LABOR_QUAL_BELOW (E5) | Labor | Employee charged above their qualification | Encl 3 §1 |
| 3 | LABOR_RATE_OVERBILL (E6) | Labor | Charged rate exceeds contract rate by > threshold | Encl 3 §1 |
| 4 | GHOST_NO_EMPLOYEE (E7) | Ghost | Billed employee not in roster | Encl 3 §2 |
| 5 | GHOST_NOT_VERIFIED (E8) | Ghost | Billed employee has no floorcheck verification | Encl 3 §2 |
| 6 | GHOST_BILLED_NOT_PERFORMED (E9) | Ghost | Billed hours exceed performed (split-billing aware) | Encl 3 §2 |
| 7 | TIME_OVERCHARGE (E10) | Ghost | Employee total billed hours exceed max per period | Encl 3 §2 |
| 8 | DUPLICATE_BILLING (E11) | Labor | Same employee billed on 2+ contracts in same period | Encl 3 §1 |
| 9 | SUB_BILLED_AS_PRIME (E16) | Subcontractor | Subcontractor billed at prime contractor rates | DCAM 6-414 |
| 10 | RATE_ESCALATION_TREND (E17) | Trend | Rate creep across consecutive billing periods | DCAM 6-606 |

See [PROOF_OF_ARTIFACTS](PROOF_OF_ARTIFACTS.md) for example outputs of all 8 DoDI rules.

---

## Code Metrics

| Metric | Value |
|--------|-------|
| Lines of Rust | 3,468 |
| Source files | 18 |
| Detection rules | 10 |
| Unit tests | 111 |
| Integration tests | 14 (f49-f62) |
| Direct dependencies | 6 (anyhow, clap, serde, serde_json, tempfile, thiserror) |
| Release binary (macOS ARM) | 637 KB |
| Rust edition | 2024 |

All public symbols are P13 compressed per [compression_map](docs/compression_map.md).
Chain-of-custody hashing uses FNV-1a for cross-platform reproducibility (legal defensibility).

---

## Docs

- [USER_STORY_ANALYSIS](USER_STORY_ANALYSIS.md) — DoD IG / FBI personas and gap analysis
- [TIMELINE_OF_INVENTION](TIMELINE_OF_INVENTION.md) — Chronological commit record
- [PROOF_OF_ARTIFACTS](PROOF_OF_ARTIFACTS.md) — Verifiable build and test metrics
- [TRIPLE_SIMS_WHYYOULYING](docs/TRIPLE_SIMS_WHYYOULYING.md) — Sim 1-4
- [TRIPLE_SIMS_ARCH](docs/TRIPLE_SIMS_ARCH.md) — Domain model, pipeline
- [TRIPLE_SIMS_STAT](docs/TRIPLE_SIMS_STAT.md) — Test coverage stats
- [protocol_map](docs/protocol_map.md) — Protocol abbreviations
- [compression_map](docs/compression_map.md) — P13 tokenization map
- [TINY_AI_OPPORTUNITIES](docs/TINY_AI_OPPORTUNITIES.md) — P23: 7 sub-100K param models for on-device fraud detection

### Federal Compliance (govdocs/)

All compliance docs are baked into the binary and available at runtime via `whyyoulying govdocs <doc>`.

- [SBOM](govdocs/SBOM.md) — Software Bill of Materials (EO 14028)
- [SSDF](govdocs/SSDF.md) — NIST SP 800-218 compliance
- [SECURITY](govdocs/SECURITY.md) — Security posture
- [PRIVACY](govdocs/PRIVACY.md) — Privacy impact assessment
- [FIPS](govdocs/FIPS.md) — FIPS 140-2/3 status
- [CMMC](govdocs/CMMC.md) — CMMC Level 1-2 practices
- [SUPPLY_CHAIN](govdocs/SUPPLY_CHAIN.md) — Supply chain integrity
- [FedRAMP_NOTES](govdocs/FedRAMP_NOTES.md) — FedRAMP applicability
- [ITAR_EAR](govdocs/ITAR_EAR.md) — Export control classification
- [ACCESSIBILITY](govdocs/ACCESSIBILITY.md) — Section 508 compliance
- [FEDERAL_USE_CASES](govdocs/FEDERAL_USE_CASES.md) — Agency use cases

---

Built by [cochranblock.org](https://cochranblock.org) — The Cochran Block. Unlicense (public domain).
