# TRIPLE SIMS Test Coverage Stat — whyyoulying

**Method:** Sim1→2→3→4. f49 f50 f51. Same binary.  
**Command:** `cargo run -p whyyoulying -- --test`  
**Date:** 2026-02-27

---

## Test Counts

| Phase | Count | Description |
|-------|-------|-------------|
| Unit (f49) | 4 | Config, LaborDetector, Alert serialization |
| Integration (f50) | 1 | Ingest + LaborDetector with TempDir fixtures |
| f51 | 1 | E2E scaffold (pass) |
| — | **6** | |

---

## TRIPLE SIMS Mapping

### Sim 1: User Story → Tests (TBD)

| User Story | Test(s) |
|------------|---------|
| D1: Proactive labor alerts | TBD |
| D3: Ghost detection | TBD |
| S1: Data ingestion | TBD |
| S3: Audit trail | TBD |

### Sim 2: Feature Gap → Tests (TBD)

| Criterion | Test(s) |
|-----------|---------|
| Labor detector | TBD |
| Ghost detector | TBD |
| Config thresholds | TBD |

### Sim 3: CLI/API → Tests (TBD)

| Criterion | Test(s) |
|-----------|---------|
| --test flag | TBD |
| Exit codes | TBD |

### Sim 4: Output Schema

| Criterion | Test(s) |
|-----------|---------|
| Alert serialization | TBD |
| Export format | TBD |

---

## Run

```bash
cargo run -p whyyoulying -- --test
```

Exit 0 = all pass. (Tests not yet implemented.)
