//! Labor category fraud detection (Labor Mischarging, Labor Substitution).
//!
//! Red flags: budget vs actual variance, employees below min quals.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, RuleId};
use chrono::Utc;

/// Category ordering for qual-below check (higher index = higher category).
const CAT_ORDER: &[&str] = &["Junior", "Mid", "Senior", "Lead", "Principal"];

fn category_level(cat: &str) -> usize {
    CAT_ORDER
        .iter()
        .position(|&c| c.eq_ignore_ascii_case(cat))
        .unwrap_or(0)
}

pub struct LaborDetector {
    pub threshold_pct: f64,
}

impl LaborDetector {
    pub fn new(threshold_pct: f64) -> Self {
        Self { threshold_pct }
    }

    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();

        for lc in &ds.labor_charges {
            if let Some(contract) = ds.contract_by_id(&lc.contract_id) {
                if !contract.labor_cats.contains_key(&lc.labor_cat) {
                    alerts.push(alert(
                        RuleId::LaborVariance,
                        &format!(
                            "Labor category '{}' not in contract {}",
                            lc.labor_cat, lc.contract_id
                        ),
                        Some(&lc.contract_id),
                        Some(&lc.employee_id),
                    ));
                }
            }

            if let Some(emp) = ds.employee_by_id(&lc.employee_id) {
                if let Some(ref min_cat) = emp.labor_cat_min {
                    if category_level(&lc.labor_cat) > category_level(min_cat) {
                        alerts.push(alert(
                            RuleId::LaborQualBelow,
                            &format!(
                                "Employee {} charged as '{}' but qualifies only for '{}'",
                                lc.employee_id, lc.labor_cat, min_cat
                            ),
                            Some(&lc.contract_id),
                            Some(&lc.employee_id),
                        ));
                    }
                }
            }
        }

        alerts
    }
}

fn alert(
    rule_id: RuleId,
    summary: &str,
    contract_id: Option<&str>,
    employee_id: Option<&str>,
) -> Alert {
    Alert {
        fraud_type: FraudType::LaborCategory,
        rule_id,
        severity: 5,
        summary: summary.to_string(),
        contract_id: contract_id.map(String::from),
        employee_id: employee_id.map(String::from),
        timestamp: Some(Utc::now().to_rfc3339()),
    }
}
