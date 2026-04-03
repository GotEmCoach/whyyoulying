// Unlicense — cochranblock.org
//! Fraud detection modules. P13 compressed.

pub mod duplicate;
pub mod ghost;
pub mod labor;
pub mod time;

#[cfg(test)]
mod tests {
    use super::labor::t13;
    use super::ghost::t14;
    use super::time::t15;
    use super::duplicate::t16;
    use crate::data::t3;
    use crate::types::{t6, t7, t8, t9};

    fn contract(id: &str, agency: Option<&str>, cage: Option<&str>) -> t6 {
        t6 { s22: id.into(), s24: agency.map(String::from), s23: cage.map(String::from), ..Default::default() }
    }

    #[test]
    fn labor_detector_empty_ds_no_alerts() {
        let ds = t3::default();
        let det = t13::f10(15.0);
        assert!(det.f11(&ds).is_empty());
    }

    #[test]
    fn labor_detector_qual_below() {
        let mut ds = t3::default();
        let c = contract("C1", Some("DoD"), None);
        ds.s7.insert(c.s22.clone(), c);
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s28: vec!["BA".into()], s29: Some("Junior".into()), s30: false });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Principal".into(), s34: 40.0, s35: Some(150.0) });
        let det = t13::f10(15.0);
        let alerts = det.f11(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.s12).contains("E5")));
    }

    #[test]
    fn labor_detector_variance_unapproved_cat() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s25: [("Senior".to_string(), "BA".to_string())].into_iter().collect(), ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "UnapprovedCat".into(), s34: 10.0, s35: None });
        let det = t13::f10(15.0);
        let alerts = det.f11(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.s12).contains("E4")));
    }

    #[test]
    fn labor_detector_rate_overbill() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 {
            s22: "C1".into(),
            s25: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
            s26: [("Senior".to_string(), 100.0)].into_iter().collect(),
            ..Default::default()
        });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: Some(120.0) });
        let det = t13::f10(15.0);
        let alerts = det.f11(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.s12).contains("E6")));
    }

    #[test]
    fn labor_detector_rate_under_threshold_no_alert() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 {
            s22: "C1".into(),
            s25: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
            s26: [("Senior".to_string(), 100.0)].into_iter().collect(),
            ..Default::default()
        });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: Some(110.0) });
        let det = t13::f10(15.0);
        assert!(!det.f11(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E6")));
    }

    #[test]
    fn labor_detector_rate_no_contract_rate_no_alert() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s25: [("Senior".to_string(), "BA".to_string())].into_iter().collect(), ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: Some(999.0) });
        let det = t13::f10(15.0);
        assert!(!det.f11(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E6")));
    }

    #[test]
    fn labor_detector_rate_no_charge_rate_no_alert() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 {
            s22: "C1".into(),
            s25: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
            s26: [("Senior".to_string(), 100.0)].into_iter().collect(),
            ..Default::default()
        });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: None });
        let det = t13::f10(15.0);
        assert!(!det.f11(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E6")));
    }

    #[test]
    fn labor_detector_qual_ok_no_alert() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s25: [("Junior".to_string(), "Assoc".to_string())].into_iter().collect(), ..Default::default() });
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s29: Some("Senior".into()), ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Junior".into(), s34: 40.0, s35: None });
        let det = t13::f10(15.0);
        assert!(det.f11(&ds).is_empty());
    }

    #[test]
    fn labor_detector_both_unknown_category_no_qual_alert() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s25: [("CustomCat".to_string(), "X".to_string())].into_iter().collect(), ..Default::default() });
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s29: Some("OtherCustom".into()), ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "CustomCat".into(), s34: 40.0, s35: None });
        let det = t13::f10(15.0);
        assert!(!det.f11(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E5")));
    }

    // --- Ghost ---

    #[test]
    fn ghost_detector_empty_ds_no_alerts() {
        let ds = t3::default();
        let det = t14::f12();
        assert!(det.f13(&ds).is_empty());
    }

    #[test]
    fn ghost_detector_no_employee() {
        let mut ds = t3::default();
        let c = contract("C1", None, None);
        ds.s7.insert(c.s22.clone(), c);
        ds.s10.push(t9 { s36: "C1".into(), s37: "E99".into(), s38: 10.0, s39: "Junior".into(), s40: None });
        let det = t14::f12();
        assert!(det.f13(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E7")));
    }

    #[test]
    fn ghost_detector_billed_not_performed() {
        let mut ds = t3::default();
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s30: true, ..Default::default() });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: None });
        let det = t14::f12();
        assert!(det.f13(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E9")));
    }

    #[test]
    fn ghost_detector_not_verified() {
        let mut ds = t3::default();
        let c = contract("C1", None, None);
        ds.s7.insert(c.s22.clone(), c);
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s30: false, ..Default::default() });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 10.0, s39: "Junior".into(), s40: None });
        let det = t14::f12();
        assert!(det.f13(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E8")));
    }

    #[test]
    fn ghost_detector_partial_performed() {
        let mut ds = t3::default();
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s30: true, ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 20.0, s35: None });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: None });
        let det = t14::f12();
        assert!(det.f13(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E9")));
    }

    #[test]
    fn ghost_detector_verified_no_alert() {
        let mut ds = t3::default();
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s30: true, ..Default::default() });
        ds.s9.push(t8 { s31: "C1".into(), s32: "E1".into(), s33: "Senior".into(), s34: 40.0, s35: None });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: None });
        let det = t14::f12();
        let alerts = det.f13(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.s12).contains("E8")));
        assert!(!alerts.iter().any(|a| format!("{:?}", a.s12).contains("E9")));
    }

    // --- Time ---

    #[test]
    fn time_detector_empty_ds_no_alerts() { assert!(t15::f14(176.0).f15(&t3::default()).is_empty()); }

    #[test]
    fn time_detector_overcharge() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 100.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        ds.s10.push(t9 { s36: "C2".into(), s37: "E1".into(), s38: 100.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        assert!(t15::f14(176.0).f15(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E10")));
    }

    #[test]
    fn time_detector_under_max_no_alert() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 160.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        assert!(!t15::f14(176.0).f15(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E10")));
    }

    #[test]
    fn time_detector_no_period_skipped() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 999.0, s39: "Senior".into(), s40: None });
        assert!(t15::f14(176.0).f15(&ds).is_empty());
    }

    #[test]
    fn time_detector_separate_periods_no_cross_contamination() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 160.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 160.0, s39: "Senior".into(), s40: Some("2026-02".into()) });
        assert!(t15::f14(176.0).f15(&ds).is_empty());
    }

    #[test]
    fn time_detector_high_excess_severity() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 250.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        let a = t15::f14(176.0).f15(&ds).into_iter().find(|a| format!("{:?}", a.s12).contains("E10")).unwrap();
        assert_eq!(a.s13, 8);
    }

    // --- Duplicate ---

    #[test]
    fn duplicate_detector_empty_ds_no_alerts() { assert!(t16::f16().f17(&t3::default()).is_empty()); }

    #[test]
    fn duplicate_detector_cross_contract() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s23: Some("1X".into()), s24: Some("DoD".into()), ..Default::default() });
        ds.s7.insert("C2".into(), t6 { s22: "C2".into(), ..Default::default() });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        ds.s10.push(t9 { s36: "C2".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        assert!(t16::f16().f17(&ds).iter().any(|a| format!("{:?}", a.s12).contains("E11")));
    }

    #[test]
    fn duplicate_detector_single_contract_no_alert() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Lead".into(), s40: Some("2026-01".into()) });
        assert!(t16::f16().f17(&ds).is_empty());
    }

    #[test]
    fn duplicate_detector_different_periods_no_alert() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: Some("2026-01".into()) });
        ds.s10.push(t9 { s36: "C2".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: Some("2026-02".into()) });
        assert!(t16::f16().f17(&ds).is_empty());
    }

    #[test]
    fn duplicate_detector_no_period_skipped() {
        let mut ds = t3::default();
        ds.s10.push(t9 { s36: "C1".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: None });
        ds.s10.push(t9 { s36: "C2".into(), s37: "E1".into(), s38: 40.0, s39: "Senior".into(), s40: None });
        assert!(t16::f16().f17(&ds).is_empty());
    }
}
