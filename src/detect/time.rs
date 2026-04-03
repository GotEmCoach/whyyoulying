// Unlicense — cochranblock.org
//! Time overcharging detection. P13 compressed.
//!
//! Red flag: employee total billed hours in a period exceed max.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;
use std::collections::HashMap;

/// t15=TimeDetector
pub struct t15 {
    /// s42=max_hours_per_period
    pub s42: f64,
}

impl t15 {
    /// f14=new
    pub fn f14(max_hours_per_period: f64) -> Self { Self { s42: max_hours_per_period } }

    /// f15=run
    #[must_use]
    pub fn f15(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();
        let mut totals: HashMap<(&str, &str), f64> = HashMap::new();
        let mut contracts_per_key: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

        for br in &ds.s10 {
            if let Some(ref period) = br.s40 {
                let key = (br.s37.as_str(), period.as_str());
                *totals.entry(key).or_insert(0.0) += br.s38;
                contracts_per_key.entry(key).or_default().push(br.s36.as_str());
            }
        }

        for ((employee_id, period), total_hours) in &totals {
            if *total_hours > self.s42 {
                let excess = total_hours - self.s42;
                let (cage_code, agency) = contracts_per_key
                    .get(&(*employee_id, *period))
                    .and_then(|cids| cids.iter().find_map(|cid| ds.f6(cid)))
                    .map(|c| (c.s23.clone(), c.s24.clone()))
                    .unwrap_or((None, None));

                alerts.push(t5 {
                    s11: t10::E3, s12: t11::E10,
                    s13: if excess > 40.0 { 8 } else { 6 }, s14: 80,
                    s15: format!("Employee '{}' billed {:.1} hrs in period {} (max {:.0}, excess {:.1})",
                        employee_id, total_hours, period, self.s42, excess),
                    s16: None, s17: Some(employee_id.to_string()),
                    s18: cage_code, s19: agency,
                    s20: Some(vec![t12::E12]), s21: Some(f20()), s66: None,
                });
            }
        }
        alerts
    }
}
