# TRIPLE SIMS: whyyoulying

**Target:** Proactive Labor Category Fraud and Ghost Billing detection  
**Method:** Sim1→2→3→4. Implement=default. @t @b @go §1.  
**Date:** 2026-02-27

**Architecture:** [TRIPLE_SIMS_ARCH.md](TRIPLE_SIMS_ARCH.md) — domain model, pipeline, phases.

---

## Sim 1: User Story Analysis

**Done.** See [USER_STORY_ANALYSIS.md](USER_STORY_ANALYSIS.md).

| Persona | User Stories |
|---------|--------------|
| DoD IG / DCIS | D1–D6: proactive alerts, labor vs quals, ghost detection, DoD nexus, referral export |
| FBI | F1–F5: preliminary inquiry signals, predicate strength, data at scale, fraud-type routing |
| Shared | S1–S4: data ingestion, configurable thresholds, audit trail, false-positive control |

---

## Sim 2: Feature Gap Analysis

**Method:** Acceptance criteria vs current scaffold (lib.rs, config, data, detect, types)

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Data ingestion | Contract, labor, billing feeds | Ingest stub | Full impl |
| Labor category detection | Variance; quals vs charged | LaborDetector stub | Logic |
| Ghost billing detection | Employee existence; billed-not-performed | GhostDetector stub | Logic |
| Configurable thresholds | labor_variance_threshold_pct | Config has default | Add more |
| Alert output | Alert struct | types::Alert | ✓ |
| Fraud referral export | GAGAS / predicate docs | — | Missing |
| Audit trail | Chain of custody | — | Missing |
| --test flag | f49 f50 f51 same binary | — | Missing |

### Prioritized Gaps

| # | Gap | Priority |
|---|-----|----------|
| 1 | Data ingestion (S1) | High |
| 2 | Labor detector logic (D1, D2) | High |
| 3 | Ghost detector logic (D3) | High |
| 4 | --test binary (P14) | High |
| 5 | Referral export (D6, F5) | Medium |
| 6 | Audit trail (S3) | Medium |
| 7 | Config thresholds (S2) | Medium |

---

## Sim 3: CLI / API UX

**Context:** Library + CLI. No web UI. Fraud officers run locally or integrate into agency pipelines.

### Current

- `main.rs`: Config::load, Ingest::new, prints "whyyoulying ready"
- No CLI args, no subcommands, no output format

### Recommendations

| # | Item | Recommendation |
|---|------|----------------|
| 1 | CLI args | `--config`, `--data-path`, `--threshold`, `--output` (json/csv) |
| 2 | Subcommands | `run`, `ingest`, `export-referral` |
| 3 | Exit codes | 0=ok, 1=alerts found, 2=error |
| 4 | Logging | stderr for progress; stdout for structured output only |
| 5 | --test | f49 f50 f51; colored PASS/FAIL |

---

## Sim 4: Output Schema / Artifacts

**Method:** Audit output formats for DoD IG and FBI referral compatibility.

### Artifacts

| Artifact | Purpose | Format |
|----------|---------|--------|
| Alert | Single anomaly | JSON: fraud_type, severity, summary, contract_id, employee_id |
| Referral package | DoD IG fraud referral | Structured export (GAGAS) |
| Case opening docs | FBI predicate | Structured export (AG Guidelines) |
| Audit log | Chain of custody | Timestamped, immutable |

### Schema Requirements

- Alert: fraud_type, severity, summary, contract_id, employee_id, timestamp, rule_id
- Export: configurable; support JSON, CSV for integration
- Audit: every alert links to rule_id + input hash

---

## Implementation Summary

**Status:** Full implementation complete.

| # | Item | Done |
|---|------|------|
| 1 | Sim 1 User Story | ✓ USER_STORY_ANALYSIS.md |
| 2 | Sim 2 Feature Gap | ✓ This doc |
| 3 | Sim 3 CLI/API UX | ✓ run, ingest, export-referral; --config, --data-path, --threshold, --output |
| 4 | Sim 4 Output Schema | ✓ Alert (rule_id, timestamp); ReferralPackage + AuditEntry |
| 5 | Architecture (TRIPLE_SIMS_ARCH.md) | ✓ Domain model, pipeline, phases |
| 6 | Domain types | ✓ Contract, Employee, LaborCharge, BillingRecord |
| 7 | --test binary | ✓ f49 f50 f51; colored PASS/FAIL |
| 8 | Data ingestion | ✓ JSON from data_path (contracts, employees, labor_charges, billing_records) |
| 9 | Labor/Ghost detectors | ✓ LABOR_VARIANCE, LABOR_QUAL_BELOW, GHOST_* |
| 10 | Referral export | ✓ GAGAS structure with audit entries |

**Commands:** `@t` `@b` `@go` §1. **Fixtures:** `fixtures/` for sample data.
