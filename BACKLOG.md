# Backlog — whyyoulying

Prioritized. Most important at top. Max 20 items.
Tagged: [build] [test] [docs] [feature] [fix] [research]
Last reorganized: 2026-04-03.

---

1. ~~[fix] E5 silent failure~~ DONE (014ae97→this) — normalize_category() with alias matching, category_level() returns Option
2. ~~[fix] E9 split-billing bypass~~ DONE — aggregate billed hours before comparing, deduplicate E7/E8 alerts
3. ~~[fix] E4 case-sensitive match~~ DONE — eq_ignore_ascii_case for category lookup + rate lookup
4. ~~[feature] Add estimated_loss (s66) to core Alert~~ DONE — s66 on t5, s67 on t1, --min-loss CLI filter, E6/E7/E9 compute loss inline
5. [feature] CSV ingest — hand-rolled parser in data.rs, fall back to JSON when both exist, no new deps (v0.3.0 Phase 1)
6. [feature] SUB_BILLED_AS_PRIME rule (E16) — detect subcontractor billed at prime rate, add is_subcontractor field to Employee (v0.3.0 Phase 2). Dep: exopack (cochranblock/exopack) for triple sims gate
7. [feature] RATE_ESCALATION_TREND rule (E17) — detect gradual rate creep across billing periods, add period field to LaborCharge (v0.3.0 Phase 2)
8. [feature] POP_OUTSIDE_DATES rule (E18) — detect billing outside contract Period of Performance, add pop_start/pop_end to Contract (v0.3.0 Phase 2)
9. [feature] Alert fingerprinting + dedup — deterministic alert_id from (rule, contract, employee), deduplicate across runs (v0.3.0 Phase 3)
10. [feature] Alert state file — .whyyoulying-state.json for --exclude-reviewed and mark-reviewed subcommand (v0.3.0 Phase 3)
11. [test] Add tests for new rules and features — target 131 unit tests from current 67 (v0.3.0 Phase 4). Dep: exopack (cochranblock/exopack) for integration test harness
12. [build] Update compression_map.md — register t22-t24, f21-f29, s66-s76, E15-E18 after v0.3.0 implementation
13. [feature] P23 Model 1: Labor Cat Normalizer (15K params) — train via kova micro forge spark, embed in binary behind --features ai. Dep: kova (candle_train.rs, TurboQuant)
14. [feature] P23 Model 4: Period Anomaly Detector (8K params) — new BILLING_PATTERN_ANOMALY rule, catches sub-threshold escalation. Dep: kova (candle_train.rs)
15. [feature] P23 Model 5: Entity Resolution (10K params) — new GHOST_ID_VARIANT rule, fuzzy employee ID matching. Dep: kova (candle_train.rs)
16. [research] Training data collection for P23 — GSA Schedule 70/OASIS labor catalogs, DCIS case outcomes (FOIA), DOJ prosecution database, Census Bureau surname list
17. [feature] IRONHIVE narrative enrichment — post-detection --ai flag calls kova cluster gen for investigator-ready prose on each alert. Dep: kova (IRONHIVE cluster, ollama)
18. [build] Binary size audit after v0.3.0 — verify still under 1 MB with new rules + CSV parser, no new deps in release profile
19. [docs] Investigator onboarding guide — "Getting Started for Special Agents", non-developer audience, explain what the output means and how to act on it
20. [research] Confidence calibration methodology — document how hardcoded confidence scores (70-95) were chosen, prepare whitepaper for court defensibility

---

## Cross-Project Dependencies

| Item | Depends On | Project | Why |
|------|-----------|---------|-----|
| 6, 11 | exopack | [cochranblock/exopack](https://github.com/cochranblock/exopack) | Triple sims test gate for new rules and integration tests |
| 13-15 | kova micro forge | [kova](~/dev/kova) | Candle Spark training pipeline, TurboQuant quantization |
| 17 | kova IRONHIVE | [kova](~/dev/kova) | Cluster inference for narrative generation (ollama on n0/n1) |
| 16 | none | external (FOIA requests, public datasets) | Training data for P23 models |
