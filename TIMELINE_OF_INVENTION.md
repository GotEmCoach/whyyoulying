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
