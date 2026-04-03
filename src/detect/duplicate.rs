// Unlicense — cochranblock.org
//! Duplicate/cross-contract billing detection. P13 compressed.
//!
//! Red flag: same employee billed on 2+ contracts in the same period.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;
use std::collections::HashMap;

/// t16=DuplicateDetector
pub struct t16;

impl Default for t16 {
    fn default() -> Self { Self }
}

impl t16 {
    /// f16=new
    pub fn f16() -> Self { Self }

    /// f17=run
    #[must_use]
    pub fn f17(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();
        let mut by_emp_period: HashMap<(&str, &str), Vec<(&str, f64)>> = HashMap::new();

        for br in &ds.s10 {
            if let Some(ref period) = br.s40 {
                by_emp_period
                    .entry((br.s37.as_str(), period.as_str()))
                    .or_default()
                    .push((br.s36.as_str(), br.s38));
            }
        }

        for ((employee_id, period), entries) in &by_emp_period {
            let mut contract_ids: Vec<&str> = entries.iter().map(|(c, _)| *c).collect();
            contract_ids.sort();
            contract_ids.dedup();
            if contract_ids.len() < 2 { continue; }

            let total_hours: f64 = entries.iter().map(|(_, h)| h).sum();
            let contract_list = contract_ids.join(", ");
            let first_contract = ds.f6(contract_ids[0]);
            let (cage_code, agency) = first_contract
                .map(|c| (c.s23.as_deref(), c.s24.as_deref()))
                .unwrap_or((None, None));

            alerts.push(t5 {
                s11: t10::E2, s12: t11::E11, s13: 7, s14: 75,
                s15: format!("Employee '{}' billed on {} contracts ({}) in period {} totaling {:.1} hrs",
                    employee_id, contract_ids.len(), contract_list, period, total_hours),
                s16: None, s17: Some(employee_id.to_string()),
                s18: cage_code.map(String::from), s19: agency.map(String::from),
                s20: Some(vec![t12::E12, t12::E13]), s21: Some(f20()), s66: None,
            });
        }
        alerts
    }
}
