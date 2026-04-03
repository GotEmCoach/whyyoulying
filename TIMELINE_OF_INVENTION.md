# Timeline of Invention — whyyoulying

Chronological record of every commit. Dates and hashes from git log.

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
