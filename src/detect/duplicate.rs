//! Duplicate/cross-contract billing detection.
//!
//! Red flag: same employee billed on 2+ contracts in the same period.
//! Indicates labor substitution or double-dipping across contracts.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, PredicateAct, RuleId};
use crate::util::now_rfc3339;
use std::collections::HashMap;

pub struct DuplicateDetector;

impl Default for DuplicateDetector {
    fn default() -> Self {
        Self
    }
}

impl DuplicateDetector {
    pub fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();

        // Group billing records by (employee_id, period) → list of (contract_id, hours).
        let mut by_emp_period: HashMap<(&str, &str), Vec<(&str, f64)>> = HashMap::new();
        for br in &ds.billing_records {
            if let Some(ref period) = br.period {
                by_emp_period
                    .entry((br.employee_id.as_str(), period.as_str()))
                    .or_default()
                    .push((br.contract_id.as_str(), br.billed_hours));
            }
        }

        for ((employee_id, period), entries) in &by_emp_period {
            // Deduplicate contract IDs — multiple line items on same contract is normal.
            let mut contract_ids: Vec<&str> = entries.iter().map(|(c, _)| *c).collect();
            contract_ids.sort();
            contract_ids.dedup();
            if contract_ids.len() < 2 {
                continue;
            }

            let total_hours: f64 = entries.iter().map(|(_, h)| h).sum();
            let contract_list = contract_ids.join(", ");

            // Look up cage_code/agency from first contract for alert metadata.
            let first_contract = ds.contract_by_id(contract_ids[0]);
            let (cage_code, agency) = first_contract
                .map(|c| (c.cage_code.as_deref(), c.agency.as_deref()))
                .unwrap_or((None, None));

            alerts.push(Alert {
                fraud_type: FraudType::LaborCategory,
                rule_id: RuleId::DuplicateBilling,
                severity: 7,
                confidence: 75,
                summary: format!(
                    "Employee '{}' billed on {} contracts ({}) in period {} totaling {:.1} hrs",
                    employee_id,
                    contract_ids.len(),
                    contract_list,
                    period,
                    total_hours
                ),
                contract_id: None,
                employee_id: Some(employee_id.to_string()),
                cage_code: cage_code.map(String::from),
                agency: agency.map(String::from),
                predicate_acts: Some(vec![PredicateAct::FalseClaims, PredicateAct::WireFraud]),
                timestamp: Some(now_rfc3339()),
            });
        }

        alerts
    }
}
