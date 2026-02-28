//! Ghost billing detection (Ghost Employees, Employee Existence).
//!
//! Red flags: unexplained employee ID gaps, billed-but-not-performed.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, RuleId};
use chrono::Utc;
use std::collections::HashSet;

pub struct GhostDetector;

impl GhostDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();
        let employee_ids: HashSet<&str> = ds.employee_ids();

        let performed_hours: std::collections::HashMap<(String, String, String), f64> = ds
            .labor_charges
            .iter()
            .fold(
                std::collections::HashMap::new(),
                |mut acc, lc| {
                    let key = (
                        lc.contract_id.clone(),
                        lc.employee_id.clone(),
                        lc.labor_cat.clone(),
                    );
                    *acc.entry(key).or_insert(0.0) += lc.hours;
                    acc
                },
            );

        for br in &ds.billing_records {
            if !employee_ids.contains(br.employee_id.as_str()) {
                alerts.push(alert(
                    RuleId::GhostNoEmployee,
                    &format!(
                        "Billed employee '{}' not in employee roster",
                        br.employee_id
                    ),
                    Some(&br.contract_id),
                    Some(&br.employee_id),
                ));
            }

            if let Some(emp) = ds.employee_by_id(&br.employee_id) {
                if !emp.verified {
                    alerts.push(alert(
                        RuleId::GhostNotVerified,
                        &format!(
                            "Billed employee '{}' has no floorcheck verification",
                            br.employee_id
                        ),
                        Some(&br.contract_id),
                        Some(&br.employee_id),
                    ));
                }
            }

            let key = (
                br.contract_id.clone(),
                br.employee_id.clone(),
                br.billed_cat.clone(),
            );
            let performed = performed_hours.get(&key).copied().unwrap_or(0.0);
            if performed < br.billed_hours - 0.01 {
                alerts.push(alert(
                    RuleId::GhostBilledNotPerformed,
                    &format!(
                        "Billed {} hrs for {}/{}/{} but only {} hrs performed",
                        br.billed_hours, br.contract_id, br.employee_id, br.billed_cat, performed
                    ),
                    Some(&br.contract_id),
                    Some(&br.employee_id),
                ));
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
        fraud_type: FraudType::GhostBilling,
        rule_id,
        severity: 5,
        summary: summary.to_string(),
        contract_id: contract_id.map(String::from),
        employee_id: employee_id.map(String::from),
        timestamp: Some(Utc::now().to_rfc3339()),
    }
}
