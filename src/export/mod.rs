//! Referral export (GAGAS) and FBI case-opening. P13 compressed.

use crate::types::t5;
use serde::Serialize;

/// t17=ReferralPackage. GAGAS-compliant referral for DoD IG.
#[derive(Debug, Serialize)]
pub struct t17 {
    pub s43: String,
    pub s44: String,
    pub s45: t18,
    pub s46: usize,
    pub s47: Vec<t5>,
    pub s48: Vec<t19>,
}

/// t18=ChainOfCustody
#[derive(Debug, Serialize)]
pub struct t18 {
    pub s49: String,
    pub s50: String,
    pub s51: bool,
}

/// t19=AuditEntry
#[derive(Debug, Serialize)]
pub struct t19 {
    pub s52: String,
    pub s53: usize,
    pub s54: String,
}

/// t20=FbiCaseOpening. FBI case-opening per AG Guidelines (F5).
#[derive(Debug, Serialize)]
pub struct t20 {
    pub s55: String,
    pub s56: String,
    pub s57: Vec<t21>,
    pub s58: std::collections::HashMap<String, usize>,
}

/// t21=FactualBasis
#[derive(Debug, Serialize)]
pub struct t21 {
    pub s59: usize,
    pub s60: String,
    pub s61: String,
    pub s62: u8,
    pub s63: Option<String>,
    pub s64: Option<String>,
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

/// f18=referral_package
pub fn f18(alerts: &[t5]) -> t17 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let generated_at = crate::util::f20();
    let audit_entries: Vec<t19> = alerts.iter().enumerate().map(|(i, a)| {
        let mut hasher = DefaultHasher::new();
        a.s16.hash(&mut hasher);
        a.s17.hash(&mut hasher);
        a.s15.hash(&mut hasher);
        format!("{:?}", a.s12).hash(&mut hasher);
        t19 { s52: format!("{:?}", a.s12), s53: i, s54: format!("{:x}", hasher.finish()) }
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
            s18: None, s19: None, s20: Some(vec![t12::E12]), s21: None,
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
}
