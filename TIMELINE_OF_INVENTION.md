# Timeline of Invention — whyyoulying

Chronological record of every commit. Dates and hashes from git log.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### DoDI 5505.02 Rule Engine — 8 Fraud Detection Rules (March 2026)

**Invention:** An 8-rule fraud detection engine that encodes specific DoDI 5505.02 (Criminal Investigations of Fraud Offenses) patterns into executable Rust code — each rule maps a regulation paragraph to a detection algorithm with legal predicate output suitable for IG referral.

**The Problem:** DoD Inspector General fraud investigations rely on human analysts manually reviewing contracts for patterns like ghost billing, labor category overbilling, and duplicate invoicing. Each pattern is documented in DoDI 5505.02 and GAGAS (Generally Accepted Government Auditing Standards), but no tool encodes these patterns into automated detection. Analysts spend weeks on what pattern matching could do in seconds.

**The Insight:** Every fraud pattern in DoDI 5505.02 is a computable rule. Ghost employees = billing for labor with no corresponding deliverables. Labor category overbilling = paying Senior Engineer rates for Junior Engineer work. Duplicate billing = same invoice amount to the same entity within a time window. These aren't AI problems — they're pattern matching problems that a 586KB Rust binary can solve.

**The Technique:**
1. 8 detection rules: GHOST_EMPLOYEE, UNPERFORMED_WORK, COST_MISCHARGE, INFLATED_COST, FALSE_CERTIFICATION, LABOR_RATE_OVERBILL, TIME_OVERCHARGE, DUPLICATE_BILLING
2. Each rule maps to a specific DoDI 5505.02 paragraph and produces a legal predicate (e.g., "Pattern consistent with 18 U.S.C. 1001 — False Statements")
3. Cross-entity nexus filter: alerts that span multiple entities are flagged for broader investigation
4. GAGAS-format referral export: output matches the format expected by IG offices
5. Demo mode with 3 baked-in contracts for instant validation

**Result:** A 586KB binary that detects 8 categories of federal contract fraud with legal predicates. No cloud, no database, no Python — just pattern matching against contract data. Output is formatted for direct IG referral per GAGAS standards.

**Named:** DoDI 5505.02 Rule Engine
**Commit:** `a90714b` (initial 5 rules), `beded33` (rule 6), `bf4f32f` (rules 7-8)
**Origin:** Michael Cochran's experience as a federal whistleblower and defense contractor. The fraud patterns in this tool are documented in real federal investigations. The insight: if you can describe the fraud pattern in English, you can encode it in Rust.

### GAGAS Referral Export (March 2026)

**Invention:** Automated export of fraud detection results in the exact format expected by DoD Inspector General offices per Generally Accepted Government Auditing Standards — including legal predicates, estimated dollar impact, and cross-entity nexus analysis.

**The Problem:** Fraud detection tools produce reports for analysts. Analysts then manually reformat findings into GAGAS-compliant referral packages for the IG. This reformatting step introduces errors, takes days, and requires specialized knowledge of GAGAS formatting requirements.

**The Insight:** If the detection tool already knows the legal predicate (18 U.S.C. 1001, 31 U.S.C. 3729), the estimated dollar impact, and the entities involved — it can produce the referral package directly. The IG referral format is standardized. Automate the last mile.

**The Technique:**
1. `export-referral` subcommand: generates full GAGAS-format referral from detection results
2. Each finding includes: rule name, legal predicate, dollar estimate, affected entities, supporting evidence
3. Text, JSON, and HTML output formats
4. Cross-entity nexus analysis: findings that span multiple contractors are grouped for conspiracy investigation

**Result:** Detection to referral in one command. No analyst reformatting step. Output is directly submittable to IG offices.

**Named:** GAGAS Referral Export
**Commit:** `a90714b` (initial implementation), `57eef26` (demo mode)
**Origin:** Frustration with the gap between "we found fraud" and "the IG accepted the referral." The formatting step shouldn't be a human bottleneck.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with DoDI 5505.02 Rule Engine and GAGAS Referral Export.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## 2026-03-10

| Hash | Description |
|------|-------------|
| bb0d6c4 | Initial commit — project scaffold, domain types, detection rules, CLI |

## 2026-03-11

| Hash | Description |
|------|-------------|
| fe002ca | Add .claude/worktrees/ to gitignore for Claude Code |
| 1a47c6c | Add CLAUDE.md for Claude Code project instructions |
| ef58b14 | Switch to Unlicense, attribute cochranblock.org in all headers |
| ee2f7c1 | Add contributor attribution to all file headers |
| a90714b | Foundational Founders — v0.2.0. Full implementation: 5 detection rules, data ingestion, CLI (run/ingest/export-referral), config, test binary (f49-f60) |
| d73f875 | Add Proof of Artifacts section with wire diagram (mermaid) |
| b482a30 | Add logo to README for LinkedIn/social preview |

## 2026-03-27

| Hash | Description |
|------|-------------|
| 02a8f02 | Fix stale docs: remove --test flag refs, correct test counts (52 not 50), update file structure. QA Round 1 |
| beded33 | Add LABOR_RATE_OVERBILL detection (rule 6). Wire up dead threshold_pct field. Contract.labor_rates added. 4 new tests |
| bf4f32f | Add TIME_OVERCHARGE and DUPLICATE_BILLING detection (rules 7-8). TimeDetector, DuplicateDetector structs. Fix nexus filter for cross-entity alerts. 11 new tests |
| 62a1d5f | Strip binary: remove chrono dep, add release profile (opt-z, lto, panic=abort, strip). 586KB from 1.3MB (56% reduction). util::now_rfc3339 replaces chrono |
| 070cbdc | Kova P13 compression: tokenize all public symbols. 20 functions, 21 types, 65 fields, 14 enum variants. compression_map.md. Serde wire format preserved |
| ac7e648 | User story analysis (10-section walkthrough, score 6.2/10). Fix 3 bugs: export serde field names, help text P13 leaks, missing flag descriptions |
| 5843124 | Federal compliance: 11 govdocs (SBOM, SSDF, SECURITY, PRIVACY, FIPS, FedRAMP, CMMC, ITAR/EAR, ACCESSIBILITY, SUPPLY_CHAIN, FEDERAL_USE_CASES) |
| 1ef93e7 | Add TIMELINE_OF_INVENTION and PROOF_OF_ARTIFACTS |

## 2026-03-28

| Hash | Description |
|------|-------------|
| 5f471cb | Add govdocs subcommand + --sbom flag. Binary serves its own compliance docs at runtime via include_str!. Live SPDX 2.3 SBOM from baked Cargo.toml |

## 2026-03-29

| Hash | Description |
|------|-------------|
| 57eef26 | Add demo subcommand. 3 baked-in contracts (Acme Defense, Pinnacle Systems, Ironclad Construction). Text/JSON/HTML reports with fraud $ estimates and legal predicates |
| 4803098 | Android project + multi-arch. JNI bridge, WebView wrapper, org.cochranblock.whyyoulying. macOS ARM + Linux x86_64 release binaries |
| 4c343d3 | Multi-arch build script. 12 targets: native (macOS ARM/Intel, iOS, WASM), remote (Linux x86_64 via st), cross (6 targets via docker). scripts/build-all-targets.sh |
| 46d9f2c | Build real AAB (213KB). Rust .so via cargo-ndk + Gradle bundleRelease. API 35, JDK 17. Uploaded to GitHub release v0.2.0 |

## 2026-03-30

| Hash | Description |
|------|-------------|
| 7a7a8cc | Polish pass. Updated TOI/POA with correct metrics (binary 636KB, LOC 2603, 16 src files, 28 pub fns). Gitignore hardened (.DS_Store, .env, *.jks, *.log). Removed empty tests/ dir and duplicate android/.gitignore |
| 57f799e | TOI: add 2026-03-30 polish pass commit (7a7a8cc) |

## 2026-04-02

| Hash | Description |
|------|-------------|
| 7ad6a82 | README: accurate metrics, full subcommand docs (demo, govdocs, --sbom), fix architecture diagram (4 detectors), add code metrics section, fix doc links |
| fddbd1a | Docs: cochranblock.org cross-linking, attribution headers in all 16 .rs files, fix stale binary size in SUPPLY_CHAIN.md |

## 2026-04-03

| Hash | Description |
|------|-------------|
| (this) | P23 Tiny AI Opportunities: 7 sub-100K param models (95K total, ~32 KB quantized). Labor cat normalizer, confidence calibrator, materiality estimator, period anomaly detector, entity resolution, predicate act router, narrative generator |
