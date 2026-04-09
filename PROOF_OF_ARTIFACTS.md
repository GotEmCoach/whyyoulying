# Proof of Artifacts — whyyoulying

Verifiable metrics from the current build, and example outputs for the 8 DoDI 5505.02 fraud rules.
Updated 2026-04-09.

---

## 1. The 8 DoDI 5505.02 rules — what fires, what comes out

Each rule below is exercised either by `fixtures/` or by a single-file fixture noted in the example.
Outputs are pretty-printed JSON taken from `whyyoulying --data-path <path> run`.
All fields are stable across builds; the `timestamp` field is wall-clock at run time.

### Rule 1 — LABOR_VARIANCE (E4)

**DoDI 5505.02, Encl 3 §1.** Labor category billed against the contract is not in the contract's approved category list.

**Predicate acts:** False Claims Act, 31 USC 3729.

**How it fires:** Charge `Architect` against a contract whose approved categories are only `Senior` and `Junior`.

```json
{
  "fraud_type": "labor_category",
  "rule_id": "LABOR_VARIANCE",
  "severity": 6,
  "confidence": 85,
  "summary": "Labor category 'Architect' not in contract C1",
  "contract_id": "C1",
  "employee_id": "E1",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims"],
  "timestamp": "2026-04-09T17:37:27Z"
}
```

---

### Rule 2 — LABOR_QUAL_BELOW (E5)

**DoDI 5505.02, Encl 3 §1.** Employee billed at a category that exceeds their qualification floor (`labor_cat_min`).

**Predicate acts:** False Claims Act, 31 USC 3729; Wire Fraud, 18 USC 1343.

**How it fires:** Employee `E2` qualifies as `Junior`, billed as `Lead`. Category aliases (`Project Lead`, `Sr. Developer`) are normalized so DCAA-style misspellings still trigger the rule.

```json
{
  "fraud_type": "labor_category",
  "rule_id": "LABOR_QUAL_BELOW",
  "severity": 7,
  "confidence": 90,
  "summary": "Employee E2 charged as 'Lead' but qualifies only for 'Junior'",
  "contract_id": "C1",
  "employee_id": "E2",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims", "wire_fraud"],
  "timestamp": "2026-04-09T16:39:35Z"
}
```

---

### Rule 3 — LABOR_RATE_OVERBILL (E6)

**DoDI 5505.02, Encl 3 §1.** Charged hourly rate exceeds the contract rate beyond the configured threshold (default 15%).

**Predicate acts:** False Claims Act; Wire Fraud.

**How it fires:** Contract rate for `Senior` is `$125/hr`; charge submitted at `$150/hr` (+20%). Includes computed `estimated_loss` so `--min-loss` filtering is meaningful.

```json
{
  "fraud_type": "labor_category",
  "rule_id": "LABOR_RATE_OVERBILL",
  "severity": 7,
  "confidence": 85,
  "summary": "Rate $150.00/hr exceeds contract $125.00/hr by 20.0% (threshold 15.0%) for C1/Senior",
  "contract_id": "C1",
  "employee_id": "E1",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims", "wire_fraud"],
  "timestamp": "2026-04-09T16:39:35Z",
  "estimated_loss": 1000.0
}
```

---

### Rule 4 — GHOST_NO_EMPLOYEE (E7)

**DoDI 5505.02, Encl 3 §2.** Billing record references an employee not present in the contractor's roster.

**Predicate acts:** False Claims Act; **Identity Fraud, 18 USC 1028** (key for FBI predicate routing).

**How it fires:** Billing line for `E3`, but `employees.json` only contains `E1` and `E2`.

```json
{
  "fraud_type": "ghost_billing",
  "rule_id": "GHOST_NO_EMPLOYEE",
  "severity": 8,
  "confidence": 95,
  "summary": "Billed employee 'E3' not in employee roster",
  "contract_id": "C1",
  "employee_id": "E3",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims", "identity_fraud"],
  "timestamp": "2026-04-09T16:39:35Z",
  "estimated_loss": 750.0
}
```

---

### Rule 5 — GHOST_NOT_VERIFIED (E8)

**DoDI 5505.02, Encl 3 §2.** Billed employee has no DCAA floorcheck verification (DCAM 13500). Single alert per `(contract, employee)` even when split-billed across multiple line items.

**Predicate acts:** False Claims Act.

**How it fires:** `E2.verified = false` and a billing record exists for them.

```json
{
  "fraud_type": "ghost_billing",
  "rule_id": "GHOST_NOT_VERIFIED",
  "severity": 5,
  "confidence": 70,
  "summary": "Billed employee 'E2' has no floorcheck verification",
  "contract_id": "C1",
  "employee_id": "E2",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims"],
  "timestamp": "2026-04-09T16:39:35Z"
}
```

---

### Rule 6 — GHOST_BILLED_NOT_PERFORMED (E9)

**DoDI 5505.02, Encl 3 §2.** Billed hours exceed performed (timesheet) hours. Aggregates billing records per `(contract, employee, category)` to catch split-billing fraud.

**Predicate acts:** False Claims Act; Wire Fraud.

**How it fires:** `E1` has zero performed hours on contract `C2` but is billed for 80 hrs. Contract `C2` has a `$130/hr` rate, so `estimated_loss = 80 × $130 = $10,400`.

```json
{
  "fraud_type": "ghost_billing",
  "rule_id": "GHOST_BILLED_NOT_PERFORMED",
  "severity": 8,
  "confidence": 90,
  "summary": "Billed 80 hrs for C2/E1/Senior but only 0 hrs performed",
  "contract_id": "C2",
  "employee_id": "E1",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims", "wire_fraud"],
  "timestamp": "2026-04-09T16:39:35Z",
  "estimated_loss": 10400.0
}
```

---

### Rule 7 — TIME_OVERCHARGE (E10)

**DoDI 5505.02, Encl 3 §2.** Employee's total billed hours in a single period exceed a physically plausible maximum (default 176 hrs/month). Cross-contract aggregation per `(employee, period)`.

**Predicate acts:** False Claims Act.

**How it fires:** `E1` is billed 80 hrs on `C2` and 160 hrs on `C1` in `2026-01` — total 240 hrs against 176-hr cap. Excess: 64 hrs.

```json
{
  "fraud_type": "ghost_billing",
  "rule_id": "TIME_OVERCHARGE",
  "severity": 8,
  "confidence": 80,
  "summary": "Employee 'E1' billed 240.0 hrs in period 2026-01 (max 176, excess 64.0)",
  "contract_id": null,
  "employee_id": "E1",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims"],
  "timestamp": "2026-04-09T16:39:35Z"
}
```

`contract_id: null` because the alert is cross-contract; `cage_code` and `agency` propagate from any related contract so DoD nexus filters still work.

---

### Rule 8 — DUPLICATE_BILLING (E11)

**DoDI 5505.02, Encl 3 §1.** Same employee billed against two or more distinct contracts in the same period — classic double-dipping pattern.

**Predicate acts:** False Claims Act; Wire Fraud.

**How it fires:** `E1` is billed in `2026-01` against both `C1` and `C2`.

```json
{
  "fraud_type": "labor_category",
  "rule_id": "DUPLICATE_BILLING",
  "severity": 7,
  "confidence": 75,
  "summary": "Employee 'E1' billed on 2 contracts (C1, C2) in period 2026-01 totaling 240.0 hrs",
  "contract_id": null,
  "employee_id": "E1",
  "cage_code": "1ABC2",
  "agency": "DoD",
  "predicate_acts": ["false_claims", "wire_fraud"],
  "timestamp": "2026-04-09T16:39:35Z"
}
```

---

## 2. Test results (current build)

| Suite | Count | Result | Command |
|-------|-------|--------|---------|
| `cargo test` (unit) | 111 | All pass | `cargo test` |
| `whyyoulying-test` (e2e) | 14 (f49-f62) | All pass | `cargo run --bin whyyoulying-test --features tests` |
| TRIPLE SIMS | 3/3 | All pass | (embedded in e2e binary) |
| **Total** | **125** | **0 failures** | |

### Polish pass — 2026-04-09 (commit `93b9153`)

| Check | Result |
|-------|--------|
| Documentation rewrite | README, CLAUDE.md, PROOF_OF_ARTIFACTS — 10 rules, accurate metrics |
| 8 DoDI 5505.02 rules — live JSON examples | All 8 captured from running binary, included in §1 |
| Build state pre-pass | **Broken** — `cargo test` did not compile |
| Root cause | 16 `t8` literals in `src/detect/mod.rs` missing `s71` (added by rate-escalation commit but never propagated to existing tests); `src/tests.rs:71` `Alert` literal missing `s66` (added by estimated_loss commit but never propagated to e2e harness); `src/detect/rate_escalation.rs:31` used an explicit `ref` binding incompatible with edition 2024 |
| Build state post-pass | `cargo test` 111/111, `whyyoulying-test` 14/14, TRIPLE SIMS 3/3 |
| Edition | Bumped 2021 → 2024 (already in working tree, now actually compiling) |
| Verdict | PASS |

E2E case map:

| Token | Coverage |
|-------|----------|
| f49 | Library API smoke test |
| f50 | JSON ingest + labor + ghost detectors |
| f51 | Release binary on `fixtures/`, JSON output validity |
| f52 | Exit code 1 when alerts found |
| f53 | `--min-confidence` filter |
| f54 | `ingest` subcommand |
| f55 | Missing `--data-path` errors gracefully |
| f56 | `--agency DoD` filter |
| f57 | `--output csv` |
| f58 | `export-referral` (DoD IG) |
| f59 | `export-referral --fbi` |
| f60 | Empty dataset → exit 0, `[]` output |
| f61 | E16 SUB_BILLED_AS_PRIME end-to-end |
| f62 | E17 RATE_ESCALATION_TREND end-to-end |

---

## 3. Binaries

| Target | Size | Notes |
|--------|------|-------|
| aarch64-apple-darwin (macOS ARM) | 652,624 bytes (637 KB) | Primary |
| x86_64-apple-darwin (macOS Intel) | 687,264 bytes (671 KB) | Cross-compiled |
| x86_64-unknown-linux-gnu | 782,528 bytes (764 KB) | Built on st |
| Android AAB | 218,413 bytes (213 KB) | Rust JNI + WebView |

Release profile: `opt-level=z`, `lto=true`, `codegen-units=1`, `panic=abort`, `strip=true`.

---

## 4. Code metrics

| Metric | Value |
|--------|-------|
| Lines of Rust | 3,468 |
| Source files | 18 (.rs) |
| Detection rules | 10 (8 DoDI + 2 DCAM) |
| Direct dependencies | 6 (release) |
| Test dependencies | +2 (exopack, tokio — feature-gated) |
| Rust edition | 2024 |

### P13 Tokenization (current)

| Category | Count |
|----------|-------|
| Functions (`f`) | 30 |
| Types (`t`) | 23 |
| Fields (`s`) | 72 |
| Enum variants (`E`) | 17 |

---

## 5. Chain-of-custody hash

`f27 = fnv1a` — FNV-1a 64-bit hash, deterministic across all platforms and Rust versions.

```rust
const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME:  u64 = 1099511628211;
```

`std::collections::hash_map::DefaultHasher` is forbidden in this codebase: it is not stable across Rust versions or platforms, and would break courtroom reproducibility of audit hashes. Six unit tests (`fnv1a_*`) lock the algorithm:

- `fnv1a_deterministic_same_inputs_same_hash`
- `fnv1a_different_inputs_different_hash`
- `fnv1a_different_rule_different_hash`
- `fnv1a_separator_prevents_collision`
- `fnv1a_output_is_16_hex_chars`
- `fnv1a_known_value` (asserts FNV offset basis: `cbf29ce484222325`)

Three additional referral-level tests (`referral_package_audit_hash_*`) verify that the hash propagates to the audit entry and changes when the alert payload changes.

---

## 6. Federal compliance docs (baked into the binary)

| Document | Standard | CLI |
|----------|----------|-----|
| SBOM.md | EO 14028, SPDX 2.3 | `whyyoulying govdocs sbom` or `whyyoulying --sbom` |
| SSDF.md | NIST SP 800-218 | `whyyoulying govdocs ssdf` |
| SECURITY.md | General posture | `whyyoulying govdocs security` |
| PRIVACY.md | PIA | `whyyoulying govdocs privacy` |
| FIPS.md | FIPS 140-2/3 | `whyyoulying govdocs fips` |
| CMMC.md | CMMC L1-2 | `whyyoulying govdocs cmmc` |
| SUPPLY_CHAIN.md | Supply chain integrity | `whyyoulying govdocs supply_chain` |
| FedRAMP_NOTES.md | FedRAMP | `whyyoulying govdocs fedramp` |
| ITAR_EAR.md | EAR/ITAR | `whyyoulying govdocs itar_ear` |
| ACCESSIBILITY.md | Section 508 | `whyyoulying govdocs accessibility` |
| FEDERAL_USE_CASES.md | Agency mapping | `whyyoulying govdocs use_cases` |

---

## 7. Dependencies (release binary)

| Crate | Version | License |
|-------|---------|---------|
| anyhow | 1.0.102 | MIT/Apache-2.0 |
| clap | 4.5.60 | MIT/Apache-2.0 |
| serde | 1.0.228 | MIT/Apache-2.0 |
| serde_json | 1.0.149 | MIT/Apache-2.0 |
| tempfile | 3.26.0 | MIT/Apache-2.0 |
| thiserror | 1.0.69 | MIT/Apache-2.0 |

No network deps. No async runtime in the release binary. No database. No telemetry.

---

## 8. Reproducing the rule examples in section 1

Rules 2-8 fire against the in-tree `fixtures/` directory:

```bash
cargo run --release -- --data-path fixtures run
```

Rule 1 (LABOR_VARIANCE) requires a labor charge with an unapproved category. Reproduce with:

```bash
mkdir -p /tmp/wyl_e4
printf '%s' '[{"id":"C1","cage_code":"1ABC2","agency":"DoD","labor_cats":{"Senior":"BA","Junior":"Assoc"}}]' > /tmp/wyl_e4/contracts.json
printf '%s' '[{"id":"E1","quals":["BA"],"labor_cat_min":"Senior","verified":true}]' > /tmp/wyl_e4/employees.json
printf '%s' '[{"contract_id":"C1","employee_id":"E1","labor_cat":"Architect","hours":40.0,"rate":175.0}]' > /tmp/wyl_e4/labor_charges.json
printf '%s' '[]' > /tmp/wyl_e4/billing_records.json
cargo run --release -- --data-path /tmp/wyl_e4 run
```

---

## 9. Out-of-scope: rules 9-10 (E16, E17)

Documented for completeness; not part of DoDI 5505.02 Encl 3.

| # | Rule | Source | Triggered by |
|---|------|--------|--------------|
| 9 | SUB_BILLED_AS_PRIME (E16) | DCAM 6-414 | `employees[*].is_subcontractor = true` billed at prime rate |
| 10 | RATE_ESCALATION_TREND (E17) | DCAM 6-606 | Same `(contract, employee, category)` rate creeps > threshold % between consecutive `period`s |

E2E coverage: `f61` and `f62`. Unit coverage: `subcontractor_*` (5 cases) and `rate_escalation_*` (10 cases).
