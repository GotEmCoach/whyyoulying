# TRIPLE SIMS Test Coverage Stat — whyyoulying

**Method:** Sim1→2→3→4. f49 f50 f51. Same binary.  
**Command:** `cargo run --bin whyyoulying-test --features tests`
**Date:** 2026-02-27

---

## Test Counts

| Phase | Count | Description |
|-------|-------|-------------|
| cargo test (unit) | 67 | config(11), types(9), data(14), detect(26), export(7) |
| whyyoulying-test f49–f60 | 12 | Unit, TempDir, e2e, self-integration (run, agency, csv, export, empty) |
| — | **79** | |

---

## TRIPLE SIMS Mapping

### Sim 1: User Story → Tests

| User Story | Test(s) |
|------------|---------|
| D1: Proactive labor alerts | f50 LaborQualBelow |
| D3: Ghost detection | f50 GHOST_NO_EMPLOYEE; run fixtures |
| S1: Data ingestion | f50 Ingest::load_from_path |
| S3: Audit trail | ReferralPackage.audit_entries |

### Sim 2: Feature Gap → Tests

| Criterion | Test(s) |
|-----------|---------|
| Labor detector | f49, f50 |
| Ghost detector | run fixtures |
| Config thresholds | f49 Config::default |

### Sim 3: CLI/API → Tests

| Criterion | Test(s) |
|-----------|---------|
| Test binary | f49–f60 via whyyoulying-test |
| Exit codes | f51 e2e |

### Sim 4: Output Schema

| Criterion | Test(s) |
|-----------|---------|
| Alert serialization | f49 |
| Export format | export-referral |

---

## Run

```bash
cargo build --release && cargo run --bin whyyoulying-test --features tests
# or: @b && @t
```

Exit 0 = all pass. Separate test binary (P14). E2E tests require release build.
