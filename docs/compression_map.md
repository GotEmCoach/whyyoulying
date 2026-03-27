# Compression Map — whyyoulying

Kova P13 tokenization. All public symbols compressed.

## Functions (f)

| Token | Name | Module | Description |
|-------|------|--------|-------------|
| f1 | load | config | Load default config |
| f2 | load_from_path | config | Load config from file |
| f3 | apply_cli_overrides | config | Apply CLI overrides |
| f4 | load | data::Ingest | Load data from config |
| f5 | load_from_path | data::Ingest | Load data from path |
| f6 | contract_by_id | data::Dataset | Contract lookup |
| f7 | employee_by_id | data::Dataset | Employee lookup |
| f8 | employee_ids | data::Dataset | All employee IDs |
| f9 | nexus_contract_ids | data::Dataset | DoD nexus filter |
| f10 | new | detect::LaborDetector | Create labor detector |
| f11 | run | detect::LaborDetector | Run labor detection |
| f12 | new | detect::GhostDetector | Create ghost detector |
| f13 | run | detect::GhostDetector | Run ghost detection |
| f14 | new | detect::TimeDetector | Create time detector |
| f15 | run | detect::TimeDetector | Run time detection |
| f16 | new | detect::DuplicateDetector | Create duplicate detector |
| f17 | run | detect::DuplicateDetector | Run duplicate detection |
| f18 | referral_package | export | GAGAS referral package |
| f19 | fbi_case_opening | export | FBI case-opening docs |
| f20 | now_rfc3339 | util | UTC RFC3339 timestamp |
| f30 | run_tests | tests | Test harness entry |

## Types (t)

| Token | Name | Module | Kind |
|-------|------|--------|------|
| t1 | Config | config | struct |
| t2 | ConfigError | config | enum |
| t3 | Dataset | data | struct |
| t4 | Ingest | data | struct |
| t5 | Alert | types | struct |
| t6 | Contract | types | struct |
| t7 | Employee | types | struct |
| t8 | LaborCharge | types | struct |
| t9 | BillingRecord | types | struct |
| t10 | FraudType | types | enum |
| t11 | RuleId | types | enum |
| t12 | PredicateAct | types | enum |
| t13 | LaborDetector | detect/labor | struct |
| t14 | GhostDetector | detect/ghost | struct |
| t15 | TimeDetector | detect/time | struct |
| t16 | DuplicateDetector | detect/duplicate | struct |
| t17 | ReferralPackage | export | struct |
| t18 | ChainOfCustody | export | struct |
| t19 | AuditEntry | export | struct |
| t20 | FbiCaseOpening | export | struct |
| t21 | FactualBasis | export | struct |

## Fields (s)

### t1=Config
| Token | Name | Type |
|-------|------|------|
| s1 | labor_variance_threshold_pct | f64 |
| s2 | data_path | Option\<String\> |
| s3 | min_confidence | u8 |
| s4 | filter_agency | Option\<String\> |
| s5 | filter_cage_code | Option\<String\> |
| s6 | max_hours_per_period | f64 |

### t3=Dataset
| Token | Name | Type |
|-------|------|------|
| s7 | contracts | HashMap |
| s8 | employees | HashMap |
| s9 | labor_charges | Vec |
| s10 | billing_records | Vec |

### t5=Alert
| Token | Name | Type |
|-------|------|------|
| s11 | fraud_type | t10 |
| s12 | rule_id | t11 |
| s13 | severity | u8 |
| s14 | confidence | u8 |
| s15 | summary | String |
| s16 | contract_id | Option\<String\> |
| s17 | employee_id | Option\<String\> |
| s18 | cage_code | Option\<String\> |
| s19 | agency | Option\<String\> |
| s20 | predicate_acts | Option\<Vec\<t12\>\> |
| s21 | timestamp | Option\<String\> |

### t6=Contract
| Token | Name | Type |
|-------|------|------|
| s22 | id | String |
| s23 | cage_code | Option\<String\> |
| s24 | agency | Option\<String\> |
| s25 | labor_cats | HashMap |
| s26 | labor_rates | HashMap |

### t7=Employee
| Token | Name | Type |
|-------|------|------|
| s27 | id | String |
| s28 | quals | Vec\<String\> |
| s29 | labor_cat_min | Option\<String\> |
| s30 | verified | bool |

### t8=LaborCharge
| Token | Name | Type |
|-------|------|------|
| s31 | contract_id | String |
| s32 | employee_id | String |
| s33 | labor_cat | String |
| s34 | hours | f64 |
| s35 | rate | Option\<f64\> |

### t9=BillingRecord
| Token | Name | Type |
|-------|------|------|
| s36 | contract_id | String |
| s37 | employee_id | String |
| s38 | billed_hours | f64 |
| s39 | billed_cat | String |
| s40 | period | Option\<String\> |

### t13=LaborDetector
| Token | Name | Type |
|-------|------|------|
| s41 | threshold_pct | f64 |

### t15=TimeDetector
| Token | Name | Type |
|-------|------|------|
| s42 | max_hours_per_period | f64 |

### t17=ReferralPackage
| Token | Name | Type |
|-------|------|------|
| s43 | document_type | String |
| s44 | generated_at | String |
| s45 | chain_of_custody | t18 |
| s46 | alert_count | usize |
| s47 | alerts | Vec\<t5\> |
| s48 | audit_entries | Vec\<t19\> |

### t18=ChainOfCustody
| Token | Name | Type |
|-------|------|------|
| s49 | tool | String |
| s50 | version | String |
| s51 | each_alert_traced_to_rule | bool |

### t19=AuditEntry
| Token | Name | Type |
|-------|------|------|
| s52 | rule_id | String |
| s53 | alert_index | usize |
| s54 | input_hash | String |

### t20=FbiCaseOpening
| Token | Name | Type |
|-------|------|------|
| s55 | document_type | String |
| s56 | generated_at | String |
| s57 | factual_basis | Vec\<t21\> |
| s58 | predicate_acts_summary | HashMap |

### t21=FactualBasis
| Token | Name | Type |
|-------|------|------|
| s59 | alert_index | usize |
| s60 | fraud_type | String |
| s61 | summary | String |
| s62 | confidence | u8 |
| s63 | contract_id | Option\<String\> |
| s64 | employee_id | Option\<String\> |
| s65 | predicate_acts | Vec\<String\> |

## Error / Enum Variants (E)

### t2=ConfigError
| Token | Name |
|-------|------|
| E1 | InvalidThreshold |

### t10=FraudType
| Token | Name | Serde |
|-------|------|-------|
| E2 | LaborCategory | labor_category |
| E3 | GhostBilling | ghost_billing |

### t11=RuleId
| Token | Name | Serde |
|-------|------|-------|
| E4 | LaborVariance | LABOR_VARIANCE |
| E5 | LaborQualBelow | LABOR_QUAL_BELOW |
| E6 | LaborRateOverbill | LABOR_RATE_OVERBILL |
| E7 | GhostNoEmployee | GHOST_NO_EMPLOYEE |
| E8 | GhostNotVerified | GHOST_NOT_VERIFIED |
| E9 | GhostBilledNotPerformed | GHOST_BILLED_NOT_PERFORMED |
| E10 | TimeOvercharge | TIME_OVERCHARGE |
| E11 | DuplicateBilling | DUPLICATE_BILLING |

### t12=PredicateAct
| Token | Name | Serde |
|-------|------|-------|
| E12 | FalseClaims | false_claims |
| E13 | WireFraud | wire_fraud |
| E14 | IdentityFraud | identity_fraud |

## CLI Commands (c)

| Token | Name | Description |
|-------|------|-------------|
| c1 | run | Run detection, output alerts |
| c2 | ingest | Load and validate data |
| c3 | export-referral | Export referral/case docs |
