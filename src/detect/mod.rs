//! Fraud detection modules.

pub mod duplicate;
pub mod ghost;
pub mod labor;
pub mod time;

#[cfg(test)]
mod tests {
    use super::duplicate::DuplicateDetector;
    use super::labor::LaborDetector;
    use super::ghost::GhostDetector;
    use super::time::TimeDetector;
    use crate::data::Dataset;
    use crate::types::{Contract, Employee, LaborCharge, BillingRecord};

    fn contract(id: &str, agency: Option<&str>, cage: Option<&str>) -> Contract {
        Contract {
            id: id.into(),
            agency: agency.map(String::from),
            cage_code: cage.map(String::from),
            ..Default::default()
        }
    }

    #[test]
    fn labor_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = LaborDetector::new(15.0);
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn labor_detector_qual_below() {
        let mut ds = Dataset::default();
        let c = contract("C1", Some("DoD"), None);
        ds.contracts.insert(c.id.clone(), c);
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                quals: vec!["BA".into()],
                labor_cat_min: Some("Junior".into()),
                verified: false,
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Principal".into(),
            hours: 40.0,
            rate: Some(150.0),
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborQualBelow")));
    }

    #[test]
    fn labor_detector_variance_unapproved_cat() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "UnapprovedCat".into(),
            hours: 10.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborVariance")));
    }

    #[test]
    fn ghost_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = GhostDetector::new();
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn ghost_detector_no_employee() {
        let mut ds = Dataset::default();
        let c = contract("C1", None, None);
        ds.contracts.insert(c.id.clone(), c);
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E99".into(),
            billed_hours: 10.0,
            billed_cat: "Junior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNoEmployee")));
    }

    #[test]
    fn ghost_detector_billed_not_performed() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }

    #[test]
    fn labor_detector_qual_ok_no_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Junior".to_string(), "Assoc".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                labor_cat_min: Some("Senior".into()),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Junior".into(),
            hours: 40.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(alerts.is_empty());
    }

    #[test]
    fn labor_detector_both_unknown_category_no_qual_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("CustomCat".to_string(), "X".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                labor_cat_min: Some("OtherCustom".into()),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "CustomCat".into(),
            hours: 40.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborQualBelow")));
    }

    #[test]
    fn labor_detector_rate_overbill() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                labor_rates: [("Senior".to_string(), 100.0)].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: Some(120.0), // 20% over contract rate of 100
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborRateOverbill")));
    }

    #[test]
    fn labor_detector_rate_under_threshold_no_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                labor_rates: [("Senior".to_string(), 100.0)].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: Some(110.0), // 10% over — under 15% threshold
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborRateOverbill")));
    }

    #[test]
    fn labor_detector_rate_no_contract_rate_no_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                ..Default::default() // no labor_rates
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: Some(999.0),
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborRateOverbill")));
    }

    #[test]
    fn labor_detector_rate_no_charge_rate_no_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                labor_rates: [("Senior".to_string(), 100.0)].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: None, // no rate on charge
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborRateOverbill")));
    }

    #[test]
    fn ghost_detector_not_verified() {
        let mut ds = Dataset::default();
        let c = contract("C1", None, None);
        ds.contracts.insert(c.id.clone(), c);
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: false,
                ..Default::default()
            },
        );
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 10.0,
            billed_cat: "Junior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNotVerified")));
    }

    #[test]
    fn ghost_detector_partial_performed() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 20.0,
            rate: None,
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }

    #[test]
    fn ghost_detector_verified_no_alert() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: None,
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNotVerified")));
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }

    // --- TimeDetector tests ---

    #[test]
    fn time_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = TimeDetector::new(176.0);
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn time_detector_overcharge() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 100.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C2".into(),
            employee_id: "E1".into(),
            billed_hours: 100.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        let det = TimeDetector::new(176.0);
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("TimeOvercharge")));
    }

    #[test]
    fn time_detector_under_max_no_alert() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 160.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        let det = TimeDetector::new(176.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("TimeOvercharge")));
    }

    #[test]
    fn time_detector_no_period_skipped() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 999.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = TimeDetector::new(176.0);
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn time_detector_separate_periods_no_cross_contamination() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 160.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 160.0,
            billed_cat: "Senior".into(),
            period: Some("2026-02".into()),
        });
        let det = TimeDetector::new(176.0);
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn time_detector_high_excess_severity() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 250.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        let det = TimeDetector::new(176.0);
        let alerts = det.run(&ds);
        let a = alerts.iter().find(|a| format!("{:?}", a.rule_id).contains("TimeOvercharge")).unwrap();
        assert_eq!(a.severity, 8); // excess > 40 hrs
    }

    // --- DuplicateDetector tests ---

    #[test]
    fn duplicate_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = DuplicateDetector::new();
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn duplicate_detector_cross_contract() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: Some("1X".into()),
                agency: Some("DoD".into()),
                ..Default::default()
            },
        );
        ds.contracts.insert(
            "C2".into(),
            Contract {
                id: "C2".into(),
                ..Default::default()
            },
        );
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C2".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        let det = DuplicateDetector::new();
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("DuplicateBilling")));
    }

    #[test]
    fn duplicate_detector_single_contract_no_alert() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Lead".into(),
            period: Some("2026-01".into()),
        });
        let det = DuplicateDetector::new();
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn duplicate_detector_different_periods_no_alert() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: Some("2026-01".into()),
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C2".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: Some("2026-02".into()),
        });
        let det = DuplicateDetector::new();
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn duplicate_detector_no_period_skipped() {
        let mut ds = Dataset::default();
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C2".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = DuplicateDetector::new();
        assert!(det.run(&ds).is_empty());
    }
}
