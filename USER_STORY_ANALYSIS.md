# User Story Analysis — whyyoulying

Full end-to-end user walkthrough. 2026-03-27.

---

## 1. Discovery

**First impression (10-second test):** PASS. README opens with "Proactive detection of Labor Category Fraud and Ghost Billing for DoD IG and FBI fraud investigators." Clear what it does, who it's for, and why. The mermaid diagram gives a pipeline overview. Detection rules table shows exactly what it catches. Logo is present for social preview.

**What's missing:** No sample output screenshot in the README. The "Screenshots" table mentions terminal output but has no actual image. A user can't see what the output looks like before building.

---

## 2. Installation

```bash
cargo build --release   # 43s clean build, 586KB binary
cargo run --release -- --help
```

**Result:** Works. Help text is clear. All flags have descriptions. Subcommands listed. Version prints `0.2.0`.

**Friction:** Requires Rust toolchain. No pre-built binaries. No `brew install` or `cargo install whyyoulying`. A fraud investigator (non-developer) can't use this without developer help.

---

## 3. First Use (Happy Path)

```bash
cargo run --release -- --data-path fixtures
```

**Steps:**
1. Read README Quick Start
2. Copy the command
3. Run it
4. Get 10 JSON alerts with fraud_type, rule_id, severity, confidence, summary

**Result:** Works perfectly. Output is structured, actionable. Each alert names the employee, contract, fraud type, and legal predicates. Severity and confidence scores enable triage.

**Friction:** Exit code is 1 (alerts found). User might think it errored. README documents exit codes, but a new user's instinct is "exit 1 = bad." No way to suppress the exit code behavior.

---

## 4. Second Use Case (Export)

```bash
cargo run --release -- --data-path fixtures export-referral
cargo run --release -- --data-path fixtures export-referral --fbi
```

**Result:** Both work. GAGAS referral includes chain_of_custody, audit_entries with input hashes. FBI case-opening includes factual_basis and predicate_acts_summary.

**Friction:** Output to file requires `--path`. Not obvious from help. `export-referral --help` would help but wasn't tested by most users.

---

## 5. Edge Cases

| # | Input | Expected | Actual | Verdict |
|---|-------|----------|--------|---------|
| 1 | No args | Error with guidance | "error: --data-path or config data_path required" exit 2 | PASS |
| 2 | `run` without `--data-path` | Same error | Same | PASS |
| 3 | Nonexistent `--data-path` | Error or warning | Silently outputs `[]` exit 0 | FAIL — should warn |
| 4 | `--threshold 999` | Reject | "must be in (0, 100], got 999" exit 2 | PASS |
| 5 | `--min-confidence 101` | Reject | "101 is not in 0..=100" exit 2 | PASS |
| 6 | Empty JSON files | No alerts | `[]` exit 0 | PASS |
| 7 | `--output csv` | CSV format | Headers + data rows | PASS |
| 8 | `--agency DoD` filter | Only DoD alerts | Correct (4 alerts vs 10 unfiltered) | PASS |

**Score: 7/8.** One silent failure (nonexistent path).

---

## 6. Feature Gap Analysis

What a real user would ask for:

1. **No database/API ingestion** — only reads local JSON files. Real DoD systems use SAP, DCAA feeds, FPDS. No connector layer.
2. **No real-time monitoring** — batch-only. User stories mention "continuous monitoring" but this is run-once.
3. **No deduplication across runs** — running twice on same data produces duplicate alerts with different timestamps.
4. **No alert suppression/triage** — can't mark an alert as "reviewed" or "false positive" and have it excluded from future runs.
5. **No multi-period trend analysis** — detects per-period issues but can't spot gradual escalation patterns across periods.
6. **No report generation** — export-referral outputs JSON, not a formatted PDF/Word doc that an investigator would attach to a referral.
7. **No CAGE code lookup** — user must know the code. No integration with SAM.gov for contractor validation.
8. **No user authentication** — anyone with the binary can run it. No audit of who ran what query.
9. **No sensitivity marking** — output should be marked FOUO/CUI per DoD policy. Currently unmarked.
10. **No threshold auto-tuning** — user must guess what threshold is right. No baseline recommendation per industry/contract type.

---

## 7. Documentation Gaps

1. **No data schema documentation** — README lists field names but doesn't explain what each field means in a DoD context (e.g., what exactly is "verified" and how does a user set it?).
2. **No sample config file** — `fixtures/config.json` exists but README doesn't mention it. A user wouldn't know config files exist without reading source.
3. **No explanation of confidence scoring** — what does confidence 85 vs 90 mean? How is it calculated? No methodology doc.
4. **No onboarding guide** — a fraud investigator getting this tool has no "Getting Started for Investigators" doc.
5. **compression_map.md** is developer-facing but exposed in docs/ alongside user-facing docs. Confusing for non-developers.

---

## 8. Competitor Check

| Tool | What it does | How whyyoulying compares |
|------|-------------|------------------------|
| **DCAA DCAM** | Government audit manual, not software | whyyoulying automates what DCAM describes manually |
| **SAP GRC** | Enterprise fraud detection | Full platform vs CLI tool. SAP costs $500K+. whyyoulying is free, focused, portable |
| **Palantir Gotham** | Intelligence analysis platform | Palantir is a data lake + viz. whyyoulying is a focused detector |
| **Oversight.gov** | Public IG report search | Reactive (past reports). whyyoulying is proactive detection |
| **Custom Python scripts** | What most IG offices actually use | whyyoulying is more structured, tested, and documented than most ad-hoc scripts |

**Honest assessment:** No direct competitor exists for this specific niche (automated LCAT + ghost billing detection per DoD IG procedures). The closest is custom agency tooling that isn't shared. whyyoulying fills a real gap. Its weakness is the local-files-only data model.

---

## 9. Verdict

| Dimension | Score | Notes |
|-----------|-------|-------|
| Usability | 7/10 | CLI is clean, help is good, but requires Rust. No pre-built binary |
| Completeness | 6/10 | 8 detection rules cover the core scenarios. Missing real-time, DB, trend analysis |
| Error Handling | 7/10 | Good validation on thresholds. Nonexistent data-path should warn |
| Documentation | 6/10 | README is solid. Missing onboarding guide, confidence methodology, sample config |
| Would Pay For? | 5/10 | Useful for PoC/demo. Not production-ready without DB connectors and report generation |

**Overall: 6.2/10.** Strong foundation, clear niche, well-tested. Needs data connectors and investigator UX to be shippable.

---

## 10. Top 3 Fixes (Implemented)

1. **Export JSON used compressed field names** (s43, s55, s48). External consumers couldn't read the output. Fixed: added `#[serde(rename)]` to all export struct fields.
2. **Help text leaked P13 tokens** (c1=run, c2=ingest). Users shouldn't see internal compression tokens. Fixed: removed tokens from doc comments.
3. **`--config`, `--threshold`, `--data-path` had no help text**. `--help` showed blank descriptions. Fixed: added descriptive help strings.
