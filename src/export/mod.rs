// Unlicense — cochranblock.org
//! Referral export (GAGAS) and FBI case-opening. P13 compressed.

use crate::types::t5;
use serde::Serialize;

/// t17=ReferralPackage. GAGAS-compliant referral for DoD IG.
#[derive(Debug, Serialize)]
pub struct t17 {
    #[serde(rename = "document_type")]
    pub s43: String,
    #[serde(rename = "generated_at")]
    pub s44: String,
    #[serde(rename = "chain_of_custody")]
    pub s45: t18,
    #[serde(rename = "alert_count")]
    pub s46: usize,
    #[serde(rename = "alerts")]
    pub s47: Vec<t5>,
    #[serde(rename = "audit_entries")]
    pub s48: Vec<t19>,
}

/// t18=ChainOfCustody
#[derive(Debug, Serialize)]
pub struct t18 {
    #[serde(rename = "tool")]
    pub s49: String,
    #[serde(rename = "version")]
    pub s50: String,
    #[serde(rename = "each_alert_traced_to_rule")]
    pub s51: bool,
}

/// t19=AuditEntry
#[derive(Debug, Serialize)]
pub struct t19 {
    #[serde(rename = "rule_id")]
    pub s52: String,
    #[serde(rename = "alert_index")]
    pub s53: usize,
    #[serde(rename = "input_hash")]
    pub s54: String,
}

/// t20=FbiCaseOpening. FBI case-opening per AG Guidelines (F5).
#[derive(Debug, Serialize)]
pub struct t20 {
    #[serde(rename = "document_type")]
    pub s55: String,
    #[serde(rename = "generated_at")]
    pub s56: String,
    #[serde(rename = "factual_basis")]
    pub s57: Vec<t21>,
    #[serde(rename = "predicate_acts_summary")]
    pub s58: std::collections::HashMap<String, usize>,
}

/// t21=FactualBasis
#[derive(Debug, Serialize)]
pub struct t21 {
    #[serde(rename = "alert_index")]
    pub s59: usize,
    #[serde(rename = "fraud_type")]
    pub s60: String,
    #[serde(rename = "summary")]
    pub s61: String,
    #[serde(rename = "confidence")]
    pub s62: u8,
    #[serde(rename = "contract_id")]
    pub s63: Option<String>,
    #[serde(rename = "employee_id")]
    pub s64: Option<String>,
    #[serde(rename = "predicate_acts")]
    pub s65: Vec<String>,
}

/// f19=fbi_case_opening
pub fn f19(alerts: &[t5]) -> t20 {
    let mut predicate_summary: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let factual_basis: Vec<t21> = alerts.iter().enumerate().map(|(i, a)| {
        let acts: Vec<String> = a.s20.as_ref()
            .map(|v| v.iter().map(|p| format!("{:?}", p)).collect())
            .unwrap_or_default();
        for act in &acts { *predicate_summary.entry(act.clone()).or_insert(0) += 1; }
        t21 { s59: i, s60: format!("{:?}", a.s11), s61: a.s15.clone(), s62: a.s14, s63: a.s16.clone(), s64: a.s17.clone(), s65: acts }
    }).collect();

    t20 { s55: "FBI Case Opening - Factual Basis".to_string(), s56: crate::util::f20(), s57: factual_basis, s58: predicate_summary }
}

/// f27=fnv1a. FNV-1a 64-bit hash — deterministic across all platforms and Rust versions.
/// Used for chain-of-custody audit hashes where reproducibility is a legal requirement.
fn f27(data: &[&str]) -> String {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    let mut hash = FNV_OFFSET;
    for s in data {
        for &byte in s.as_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // Separator byte so ("ab","c") != ("a","bc")
        hash ^= 0x1e;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

/// f18=referral_package
pub fn f18(alerts: &[t5]) -> t17 {
    let generated_at = crate::util::f20();
    let audit_entries: Vec<t19> = alerts.iter().enumerate().map(|(i, a)| {
        let rule = format!("{:?}", a.s12);
        let hash = f27(&[
            rule.as_str(),
            a.s16.as_deref().unwrap_or(""),
            a.s17.as_deref().unwrap_or(""),
            a.s15.as_str(),
        ]);
        t19 { s52: rule, s53: i, s54: hash }
    }).collect();

    t17 {
        s43: "DoD IG Fraud Referral Package".to_string(),
        s44: generated_at,
        s45: t18 { s49: "whyyoulying".to_string(), s50: env!("CARGO_PKG_VERSION", "?").to_string(), s51: true },
        s46: alerts.len(),
        s47: alerts.to_vec(),
        s48: audit_entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{t5, t10, t11, t12};

    fn sample_alert() -> t5 {
        t5 {
            s11: t10::E2, s12: t11::E5, s13: 7, s14: 90,
            s15: "test".into(), s16: Some("C1".into()), s17: Some("E1".into()),
            s18: None, s19: None, s20: Some(vec![t12::E12]), s21: None, s66: None,
        }
    }

    #[test]
    fn referral_package_structure() {
        let pkg = f18(&[sample_alert()]);
        assert_eq!(pkg.s46, 1);
        assert_eq!(pkg.s47.len(), 1);
        assert_eq!(pkg.s48.len(), 1);
        assert!(pkg.s43.contains("DoD"));
        assert!(pkg.s45.s51);
        assert_eq!(pkg.s45.s49, "whyyoulying");
    }

    #[test]
    fn referral_package_audit_entry_has_hash() {
        let pkg = f18(&[sample_alert()]);
        assert!(!pkg.s48[0].s54.is_empty());
        assert!(pkg.s48[0].s54.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn fbi_case_opening_structure() {
        let fbi = f19(&[sample_alert()]);
        assert!(fbi.s55.contains("FBI"));
        assert_eq!(fbi.s57.len(), 1);
        assert!(!fbi.s58.is_empty());
        assert_eq!(fbi.s57[0].s65.len(), 1);
    }

    #[test]
    fn fbi_case_opening_empty() {
        let fbi = f19(&[]);
        assert!(fbi.s57.is_empty());
        assert!(fbi.s58.is_empty());
    }

    #[test]
    fn referral_package_multiple_alerts() {
        let pkg = f18(&[sample_alert(), sample_alert()]);
        assert_eq!(pkg.s46, 2);
        assert_eq!(pkg.s48.len(), 2);
    }

    #[test]
    fn referral_package_audit_index_matches() {
        let pkg = f18(&[sample_alert()]);
        assert_eq!(pkg.s48[0].s53, 0);
    }

    #[test]
    fn referral_package_json_serializable() {
        let j = serde_json::to_string(&f18(&[sample_alert()])).unwrap();
        assert!(j.contains("DoD"));
    }

    // --- FNV-1a chain-of-custody hash tests ---

    #[test]
    fn fnv1a_deterministic_same_inputs_same_hash() {
        let h1 = f27(&["E5", "C1", "E1", "some summary"]);
        let h2 = f27(&["E5", "C1", "E1", "some summary"]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn fnv1a_different_inputs_different_hash() {
        let h1 = f27(&["E5", "C1", "E1", "summary A"]);
        let h2 = f27(&["E5", "C1", "E1", "summary B"]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn fnv1a_different_rule_different_hash() {
        let h1 = f27(&["E4", "C1", "E1", "x"]);
        let h2 = f27(&["E7", "C1", "E1", "x"]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn fnv1a_separator_prevents_collision() {
        // ("ab","c") must differ from ("a","bc")
        let h1 = f27(&["ab", "c"]);
        let h2 = f27(&["a", "bc"]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn fnv1a_output_is_16_hex_chars() {
        let h = f27(&["E5", "C1", "E1", "test"]);
        assert_eq!(h.len(), 16);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn fnv1a_known_value() {
        // FNV-1a of empty input: offset basis unchanged
        let h = f27(&[]);
        assert_eq!(h, "cbf29ce484222325"); // FNV offset basis
    }

    #[test]
    fn referral_package_audit_hash_is_deterministic() {
        // Same alert → same hash on every call (not timestamp-dependent)
        let a = sample_alert();
        let pkg1 = f18(&[a.clone()]);
        let pkg2 = f18(&[a]);
        assert_eq!(pkg1.s48[0].s54, pkg2.s48[0].s54);
    }

    #[test]
    fn referral_package_audit_hash_changes_with_different_alert() {
        let mut a2 = sample_alert();
        a2.s15 = "different summary".into();
        let pkg1 = f18(&[sample_alert()]);
        let pkg2 = f18(&[a2]);
        assert_ne!(pkg1.s48[0].s54, pkg2.s48[0].s54);
    }

    #[test]
    fn referral_package_audit_hash_changes_with_different_contract() {
        let mut a2 = sample_alert();
        a2.s16 = Some("C2".into());
        let pkg1 = f18(&[sample_alert()]);
        let pkg2 = f18(&[a2]);
        assert_ne!(pkg1.s48[0].s54, pkg2.s48[0].s54);
    }
}
