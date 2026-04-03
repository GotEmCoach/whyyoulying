# P23: Tiny AI Opportunities — whyyoulying

Sub-100K parameter models that augment (never replace) the deterministic rule engine.
All models are optional (`--ai` flag), feature-gated, and fall back to current logic on failure.
Audit trail preserved: deterministic rules always run first, AI layer enriches after.

## Why Tiny AI

The 8 detection rules use hardcoded thresholds, exact string matching, and fixed confidence scores.
This works for clear-cut fraud but misses edge cases:

- "Sr. Developer" vs "Senior Developer" fails case-sensitive E4 match silently
- "Project Lead" not in `["Junior", "Mid", "Senior", "Lead", "Principal"]` returns level 0, bypasses E5
- Confidence 85% means nothing — not calibrated against real case outcomes
- No dollar estimates when contract rates are missing ($100/hr arbitrary fallback)
- No trend detection — 175 hrs/month for 12 months (just under 176 threshold) is invisible
- Predicate act routing is one-size-fits-all per rule, ignores context

## 7 Models

### 1. Labor Category Normalizer (15K params)

Maps free-text labor category strings to canonical levels.
Fixes silent failures in E4 (LABOR_VARIANCE) and E5 (LABOR_QUAL_BELOW).

| Field | Value |
|-------|-------|
| Input | Raw labor category string, 1-50 chars |
| Output | Canonical level index (0-4: Junior/Mid/Senior/Lead/Principal) + confidence |
| Architecture | Character-level embedding (32-dim) + 2-layer MLP |
| Params | ~15K |
| Latency | <10 us |
| Training data | GSA Schedule 70/OASIS labor categories (~2K titles), FPDS labor cat fields, OPM GS grade mappings |
| Fallback | Exact match (current behavior) if confidence < 0.8 |

### 2. Confidence Calibrator (5K params)

Adjusts confidence scores based on alert context instead of hardcoded values.

| Field | Value |
|-------|-------|
| Input | 8-dim feature vector: rule_id (one-hot), excess_hours_norm, rate_variance_pct, contract_value, employee_tenure, period_count |
| Output | Calibrated confidence 0-100 |
| Architecture | 2-layer MLP, sigmoid output scaled to 100 |
| Params | ~5K |
| Latency | <5 us |
| Training data | Historical DoD IG case outcomes (DCIS, ~10K adjudicated referrals). Alternative: DCAA audit reports with known outcomes |
| Fallback | Hardcoded confidence per rule |

### 3. Materiality Estimator (3K params)

Estimates hourly rate when contract rate data is missing.
Enables `--min-loss` filtering even with incomplete data.

| Field | Value |
|-------|-------|
| Input | 6-dim vector: labor_cat_level (0-4), agency (one-hot, 3), hours_billed, hours_performed, region_index, contract_year |
| Output | Estimated hourly rate (single float) |
| Architecture | 2-layer MLP |
| Params | ~3K |
| Latency | <3 us |
| Training data | GSA Alliant 2/OASIS rate catalogs (~50K entries by category/region/year), SAM.gov historical rates |
| Fallback | $100/hr default (current demo.rs behavior) |

### 4. Period Anomaly Detector (8K params)

Time-series anomaly detection on per-employee billing patterns.
Catches gradual escalation that stays just under the 176-hour threshold.

| Field | Value |
|-------|-------|
| Input | Sequence of (period, total_hours) tuples per employee, max 24 periods |
| Output | Anomaly score per period (0.0-1.0) |
| Architecture | 1D convolution (kernel=3) + global pool + linear |
| Params | ~8K |
| Latency | <15 us per employee |
| Training data | Synthetic: normal billing patterns (140-170 hrs/month with variance) + injected anomalies (gradual escalation, sudden spikes, perfectly consistent hours). ~100K sequences |
| New rule | BILLING_PATTERN_ANOMALY |
| Fallback | No alert (purely additive) |

### 5. Entity Resolution (10K params)

Fuzzy matching between employee identifiers to detect ghost variants.
"John Smith" as E101 on Contract A and JS-201 on Contract B.

| Field | Value |
|-------|-------|
| Input | Two employee ID/name strings |
| Output | Match probability (0.0-1.0) |
| Architecture | Siamese character-level embedding (16-dim) + cosine similarity |
| Params | ~10K |
| Latency | <8 us per pair |
| Training data | Census Bureau surname list, OPM fedscope data, synthetic ID variations. ~50K pos/neg pairs |
| New rule | GHOST_ID_VARIANT |
| Fallback | Exact match (current E7 behavior) |

### 6. Predicate Act Router (4K params)

Context-aware assignment of predicate acts (False Claims, Wire Fraud, Identity Fraud).
Current mapping is hardcoded per rule and ignores alert context.

| Field | Value |
|-------|-------|
| Input | 12-dim vector: rule_id (one-hot, 8), has_ssn, has_address, dollar_amount_log, employee_exists_in_other_contracts |
| Output | 3-dim sigmoid: [false_claims_prob, wire_fraud_prob, identity_fraud_prob] |
| Architecture | 2-layer MLP, multi-label sigmoid |
| Params | ~4K |
| Latency | <3 us |
| Training data | DOJ fraud prosecution database (~5K cases with charged predicates), FBI UCR white-collar crime data |
| Fallback | Hardcoded mapping per rule |

### 7. Narrative Generator (50K params)

Generates investigator-ready prose from structured alert data.
Template strings become court-ready summaries.

| Field | Value |
|-------|-------|
| Input | Structured alert JSON (rule_id, amounts, employee, contract, predicate acts) |
| Output | 1-3 sentence investigator narrative (~50 tokens) |
| Architecture | Spark transformer (kova candle_train.rs architecture) |
| Params | ~50K |
| Latency | <500 us (autoregressive) |
| Training data | DCIS investigation summaries (FOIA), DoD IG semiannual reports, GAO fraud audit findings (~2K examples) |
| Fallback | Template string (current behavior) |

## Totals

| Metric | Value |
|--------|-------|
| Total models | 7 |
| Total params | ~95K |
| Total size (quantized ~2.5 bits/weight) | ~32 KB |
| Binary impact | 621 KB + 32 KB = ~653 KB (under 1 MB) |
| New detection rules | 2 (BILLING_PATTERN_ANOMALY, GHOST_ID_VARIANT) |

## Integration

```
whyyoulying run --data-path fixtures --ai

  Deterministic Rules (E4-E11) ──── always run, audit trail preserved
  │
  Tiny AI Layer (optional --ai flag, feature-gated)
  ├── Pre-detection:  Model 1 (normalize categories), Model 5 (entity resolution)
  ├── Post-detection: Model 2 (calibrate confidence), Model 3 (estimate loss),
  │                   Model 4 (period anomaly), Model 6 (route predicates)
  └── Export-time:    Model 7 (generate narratives)
```

## Constraints

- Feature-gated: `[features] ai = ["candle-core", "candle-nn"]`. Without `--features ai`, binary stays at 621 KB
- Air-gapped: no network dependencies, all models embedded via `include_bytes!`
- Deterministic rules always run first — AI never gates or blocks rule engine output
- Fallback: every model has a deterministic fallback if unavailable or low-confidence
- Training: kova `candle_train.rs` Spark tier, quantized via TurboQuant

## Training Pipeline

Train via kova's existing infrastructure:

```bash
kova micro forge spark --data training/labor_cat_normalizer.jsonl
kova micro quantize spark --output models/labor_cat_normalizer.safetensors
```

Pack all 7 models into single binary at build time. No runtime model loading.

## What Tiny AI Does NOT Do

- Replace deterministic rules (breaks audit trail, GAGAS compliance)
- Adjust `--min-confidence` or `--threshold` (must remain deterministic)
- Require network access (must work on classified networks)
- Add release dependencies (candle behind feature flag)
