// Unlicense — cochranblock.org
//! Core types for fraud detection. P13 compressed.
//!
//! Domain model per TRIPLE_SIMS_ARCH.md. See docs/compression_map.md.

use std::fmt;
use serde::{Deserialize, Serialize};

/// t10=FraudType. Fraud classification per DoD IG scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum t10 {
    #[serde(rename = "labor_category")]
    E2,
    #[serde(rename = "ghost_billing")]
    E3,
    #[serde(rename = "subcontractor_fraud")]
    E15,
}

impl fmt::Display for t10 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            t10::E2 => "labor_category",
            t10::E3 => "ghost_billing",
            t10::E15 => "subcontractor_fraud",
        })
    }
}

/// t11=RuleId. Rule ID for audit trail and chain of custody (Sim 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum t11 {
    #[serde(rename = "LABOR_VARIANCE")]
    E4,
    #[serde(rename = "LABOR_QUAL_BELOW")]
    E5,
    #[serde(rename = "LABOR_RATE_OVERBILL")]
    E6,
    #[serde(rename = "GHOST_NO_EMPLOYEE")]
    E7,
    #[serde(rename = "GHOST_NOT_VERIFIED")]
    E8,
    #[serde(rename = "GHOST_BILLED_NOT_PERFORMED")]
    E9,
    #[serde(rename = "TIME_OVERCHARGE")]
    E10,
    #[serde(rename = "DUPLICATE_BILLING")]
    E11,
    #[serde(rename = "SUB_BILLED_AS_PRIME")]
    E16,
}

impl fmt::Display for t11 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            t11::E4 => "LABOR_VARIANCE",
            t11::E5 => "LABOR_QUAL_BELOW",
            t11::E6 => "LABOR_RATE_OVERBILL",
            t11::E7 => "GHOST_NO_EMPLOYEE",
            t11::E8 => "GHOST_NOT_VERIFIED",
            t11::E9 => "GHOST_BILLED_NOT_PERFORMED",
            t11::E10 => "TIME_OVERCHARGE",
            t11::E11 => "DUPLICATE_BILLING",
            t11::E16 => "SUB_BILLED_AS_PRIME",
        })
    }
}

/// t12=PredicateAct. Predicate act for FBI case routing (F4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum t12 {
    #[serde(rename = "false_claims")]
    E12,
    #[serde(rename = "wire_fraud")]
    E13,
    #[serde(rename = "identity_fraud")]
    E14,
}

/// t5=Alert. Alert produced by a detector for fraud referral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct t5 {
    #[serde(rename = "fraud_type")]
    pub s11: t10,
    #[serde(rename = "rule_id")]
    pub s12: t11,
    #[serde(rename = "severity")]
    pub s13: u8,
    /// 0-100; higher = stronger indicator (S4 false-positive control).
    #[serde(rename = "confidence")]
    pub s14: u8,
    #[serde(rename = "summary")]
    pub s15: String,
    #[serde(rename = "contract_id")]
    pub s16: Option<String>,
    #[serde(rename = "employee_id")]
    pub s17: Option<String>,
    #[serde(rename = "cage_code", skip_serializing_if = "Option::is_none")]
    pub s18: Option<String>,
    #[serde(rename = "agency", skip_serializing_if = "Option::is_none")]
    pub s19: Option<String>,
    /// FBI predicate routing (F4).
    #[serde(rename = "predicate_acts", skip_serializing_if = "Option::is_none")]
    pub s20: Option<Vec<t12>>,
    #[serde(rename = "timestamp")]
    pub s21: Option<String>,
    /// s66=estimated_loss. Dollar amount of suspected fraud.
    #[serde(rename = "estimated_loss", skip_serializing_if = "Option::is_none")]
    pub s66: Option<f64>,
}

// --- Domain entities (TRIPLE_SIMS_ARCH §1) ---

/// t6=Contract. Contract proposal/requirements: labor categories and min quals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct t6 {
    #[serde(rename = "id")]
    pub s22: String,
    #[serde(rename = "cage_code")]
    pub s23: Option<String>,
    #[serde(rename = "agency")]
    pub s24: Option<String>,
    /// Map labor_cat → min qualification level.
    #[serde(rename = "labor_cats")]
    pub s25: std::collections::HashMap<String, String>,
    /// Contracted rate per labor category ($/hr).
    #[serde(rename = "labor_rates", default)]
    pub s26: std::collections::HashMap<String, f64>,
}

/// t7=Employee. Employee qualifications vs charged category.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct t7 {
    #[serde(rename = "id")]
    pub s27: String,
    /// Qualification levels (e.g. ["Senior", "BA"]).
    #[serde(rename = "quals")]
    pub s28: Vec<String>,
    /// Minimum labor category this employee qualifies for.
    #[serde(rename = "labor_cat_min")]
    pub s29: Option<String>,
    /// Floorcheck verified (DCAA 13500).
    #[serde(rename = "verified")]
    pub s30: bool,
    /// s70=is_subcontractor. True if employee is subcontractor, not prime.
    #[serde(rename = "is_subcontractor", default)]
    pub s70: Option<bool>,
}

/// t8=LaborCharge. Actual labor charged (timesheet/DCAA).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct t8 {
    #[serde(rename = "contract_id")]
    pub s31: String,
    #[serde(rename = "employee_id")]
    pub s32: String,
    #[serde(rename = "labor_cat")]
    pub s33: String,
    #[serde(rename = "hours")]
    pub s34: f64,
    #[serde(rename = "rate")]
    pub s35: Option<f64>,
}

/// t9=BillingRecord. What was billed to gov.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct t9 {
    #[serde(rename = "contract_id")]
    pub s36: String,
    #[serde(rename = "employee_id")]
    pub s37: String,
    #[serde(rename = "billed_hours")]
    pub s38: f64,
    #[serde(rename = "billed_cat")]
    pub s39: String,
    #[serde(rename = "period")]
    pub s40: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_serialize_fraud_type_snake_case() {
        let a = t5 {
            s11: t10::E2, s12: t11::E4, s13: 5, s14: 85,
            s15: "x".into(), s16: None, s17: None, s18: None,
            s19: None, s20: None, s21: None, s66: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("labor_category"));
    }

    #[test]
    fn alert_serialize_rule_id_screaming_snake() {
        let a = t5 {
            s11: t10::E3, s12: t11::E7, s13: 8, s14: 95,
            s15: "x".into(), s16: None, s17: None, s18: None,
            s19: None, s20: None, s21: None, s66: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("GHOST_NO_EMPLOYEE"));
    }

    #[test]
    fn alert_roundtrip() {
        let a = t5 {
            s11: t10::E2, s12: t11::E5, s13: 7, s14: 90,
            s15: "test".into(), s16: Some("C1".into()), s17: Some("E1".into()),
            s18: Some("1ABC".into()), s19: Some("DoD".into()),
            s20: Some(vec![t12::E12]), s21: Some("2026-01-01T00:00:00Z".into()), s66: Some(5000.0),
        };
        let j = serde_json::to_string(&a).unwrap();
        let b: t5 = serde_json::from_str(&j).unwrap();
        assert_eq!(a.s11, b.s11);
        assert_eq!(a.s12, b.s12);
        assert_eq!(a.s16, b.s16);
    }

    #[test]
    fn contract_default() {
        let c = t6::default();
        assert!(c.s22.is_empty());
        assert!(c.s25.is_empty());
    }

    #[test]
    fn employee_default() {
        let e = t7::default();
        assert!(e.s27.is_empty());
        assert!(e.s28.is_empty());
        assert!(!e.s30);
    }

    #[test]
    fn labor_charge_default() {
        let lc = t8::default();
        assert!(lc.s31.is_empty());
        assert_eq!(lc.s34, 0.0);
    }

    #[test]
    fn billing_record_default() {
        let br = t9::default();
        assert!(br.s37.is_empty());
        assert_eq!(br.s38, 0.0);
    }

    #[test]
    fn fraud_type_ghost_serialize() {
        let a = t5 {
            s11: t10::E3, s12: t11::E7, s13: 8, s14: 95,
            s15: "x".into(), s16: None, s17: None, s18: None,
            s19: None, s20: None, s21: None, s66: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("ghost_billing"));
    }

    #[test]
    fn predicate_act_serialize() {
        let a = t5 {
            s11: t10::E2, s12: t11::E4, s13: 5, s14: 85,
            s15: "x".into(), s16: None, s17: None, s18: None,
            s19: None, s20: Some(vec![t12::E13]), s21: None, s66: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("wire_fraud"));
    }
}
