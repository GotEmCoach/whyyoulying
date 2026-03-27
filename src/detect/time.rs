//! Time overcharging detection.
//!
//! Red flag: employee total billed hours in a period exceed max (default 176 hrs/month).
//! DoD IG fraud scenario: Time Overcharging.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, PredicateAct, RuleId};
use chrono::Utc;
use std::collections::HashMap;

pub struct TimeDetector {
    pub max_hours_per_period: f64,
}

impl TimeDetector {
    pub fn new(max_hours_per_period: f64) -> Self {
        Self { max_hours_per_period }
    }

    #[must_use]
    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();

        // Aggregate billed hours per (employee_id, period).
        // Track contract IDs per key for agency/cage lookup.
        // Skip records without a period — can't detect overcharge without time boundary.
        let mut totals: HashMap<(&str, &str), f64> = HashMap::new();
        let mut contracts_per_key: HashMap<(&str, &str), Vec<&str>> = HashMap::new();
        for br in &ds.billing_records {
            if let Some(ref period) = br.period {
                let key = (br.employee_id.as_str(), period.as_str());
                *totals.entry(key).or_insert(0.0) += br.billed_hours;
                contracts_per_key.entry(key).or_default().push(br.contract_id.as_str());
            }
        }

        for ((employee_id, period), total_hours) in &totals {
            if *total_hours > self.max_hours_per_period {
                let excess = total_hours - self.max_hours_per_period;

                // Pull agency/cage from first known contract for nexus filtering.
                let (cage_code, agency) = contracts_per_key
                    .get(&(*employee_id, *period))
                    .and_then(|cids| cids.iter().find_map(|cid| ds.contract_by_id(cid)))
                    .map(|c| (c.cage_code.clone(), c.agency.clone()))
                    .unwrap_or((None, None));

                alerts.push(Alert {
                    fraud_type: FraudType::GhostBilling,
                    rule_id: RuleId::TimeOvercharge,
                    severity: if excess > 40.0 { 8 } else { 6 },
                    confidence: 80,
                    summary: format!(
                        "Employee '{}' billed {:.1} hrs in period {} (max {:.0}, excess {:.1})",
                        employee_id, total_hours, period, self.max_hours_per_period, excess
                    ),
                    contract_id: None,
                    employee_id: Some(employee_id.to_string()),
                    cage_code,
                    agency,
                    predicate_acts: Some(vec![PredicateAct::FalseClaims]),
                    timestamp: Some(Utc::now().to_rfc3339()),
                });
            }
        }

        alerts
    }
}
