// Unlicense — cochranblock.org
//! Rate escalation trend detection. P13 compressed.
//!
//! Red flag: employee rate for a given labor category creeps upward across
//! billing periods beyond the configured threshold (E17).

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;
use std::collections::HashMap;

/// t23=RateEscalationDetector
pub struct t23 {
    /// s72=threshold_pct. Min % rate increase per consecutive period to alert.
    pub s72: f64,
}

impl t23 {
    /// f25=new
    pub fn f25(threshold_pct: f64) -> Self { Self { s72: threshold_pct } }

    /// f26=run. Detect rate creep across billing periods for the same
    /// (contract, employee, labor_cat) tuple.
    #[must_use]
    pub fn f26(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();

        // Group: (contract_id, employee_id, labor_cat) → Vec<(period, rate)>
        let mut groups: HashMap<(String, String, String), Vec<(String, f64)>> = HashMap::new();
        for lc in &ds.s9 {
            if let (Some(ref period), Some(rate)) = (&lc.s71, lc.s35) {
                groups
                    .entry((lc.s31.clone(), lc.s32.clone(), lc.s33.clone()))
                    .or_default()
                    .push((period.clone(), rate));
            }
        }

        for ((contract_id, employee_id, labor_cat), mut entries) in groups {
            if entries.len() < 2 { continue; }
            // Sort by period string — ISO "YYYY-MM" sorts correctly lexicographically
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            for window in entries.windows(2) {
                let (prev_period, prev_rate) = &window[0];
                let (curr_period, curr_rate) = &window[1];
                if prev_rate <= &0.0 { continue; }
                let increase_pct = ((curr_rate - prev_rate) / prev_rate) * 100.0;
                if increase_pct > self.s72 {
                    let contract = ds.f6(&contract_id);
                    let (cage_code, agency) = contract
                        .map(|c| (c.s23.as_deref(), c.s24.as_deref()))
                        .unwrap_or((None, None));
                    alerts.push(t5 {
                        s11: t10::E2,
                        s12: t11::E17,
                        s13: if increase_pct > 25.0 { 8 } else { 6 },
                        s14: 75,
                        s15: format!(
                            "Rate for {}/{}/{} increased {:.1}% from {} (${:.2}) to {} (${:.2})",
                            contract_id, employee_id, labor_cat,
                            increase_pct, prev_period, prev_rate, curr_period, curr_rate
                        ),
                        s16: Some(contract_id.clone()),
                        s17: Some(employee_id.clone()),
                        s18: cage_code.map(String::from),
                        s19: agency.map(String::from),
                        s20: Some(vec![t12::E12, t12::E13]),
                        s21: Some(f20()),
                        s66: None,
                    });
                }
            }
        }
        alerts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::t3;
    use crate::types::t8;

    fn lc(contract_id: &str, employee_id: &str, cat: &str, rate: f64, period: &str) -> t8 {
        t8 { s31: contract_id.into(), s32: employee_id.into(), s33: cat.into(), s34: 40.0, s35: Some(rate), s71: Some(period.into()) }
    }

    #[test]
    fn empty_ds_no_alerts() {
        assert!(t23::f25(10.0).f26(&t3::default()).is_empty());
    }

    #[test]
    fn single_period_no_alert() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn flat_rate_no_alert() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-02"));
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn slight_increase_under_threshold_no_alert() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 109.0, "2026-02")); // 9% < 10% threshold
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn rate_escalation_triggers_e17() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 115.0, "2026-02")); // 15% > 10%
        let alerts = t23::f25(10.0).f26(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.s12).contains("E17")));
    }

    #[test]
    fn rate_escalation_high_increase_severity_8() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 130.0, "2026-02")); // 30% > 25%
        let alert = t23::f25(10.0).f26(&ds).into_iter().find(|a| format!("{:?}", a.s12).contains("E17")).unwrap();
        assert_eq!(alert.s13, 8);
    }

    #[test]
    fn rate_escalation_moderate_severity_6() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 120.0, "2026-02")); // 20%, between 10-25%
        let alert = t23::f25(10.0).f26(&ds).into_iter().find(|a| format!("{:?}", a.s12).contains("E17")).unwrap();
        assert_eq!(alert.s13, 6);
    }

    #[test]
    fn different_employees_independent() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E2", "Senior", 200.0, "2026-01")); // different employee, no window
        // Only one entry per employee, no alert
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn different_contracts_independent() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C2", "E1", "Senior", 150.0, "2026-02")); // different contract
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn no_rate_field_skipped() {
        let mut ds = t3::default();
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: None, s71: Some("2026-01".into()) });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: None, s71: Some("2026-02".into()) });
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn no_period_field_skipped() {
        let mut ds = t3::default();
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: Some(100.0), s71: None });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: Some(150.0), s71: None });
        assert!(t23::f25(10.0).f26(&ds).is_empty());
    }

    #[test]
    fn multi_period_escalation_fires_per_window() {
        let mut ds = t3::default();
        ds.s9.push(lc("C1", "E1", "Senior", 100.0, "2026-01"));
        ds.s9.push(lc("C1", "E1", "Senior", 120.0, "2026-02")); // +20%
        ds.s9.push(lc("C1", "E1", "Senior", 150.0, "2026-03")); // +25%
        let alerts = t23::f25(10.0).f26(&ds);
        assert_eq!(alerts.iter().filter(|a| format!("{:?}", a.s12).contains("E17")).count(), 2);
    }
}
