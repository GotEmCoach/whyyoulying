//! Ghost billing detection. P13 compressed.
//!
//! Red flags: unexplained employee ID gaps, billed-but-not-performed.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;
use std::collections::HashSet;

/// t14=GhostDetector
pub struct t14;

impl Default for t14 {
    fn default() -> Self { Self }
}

impl t14 {
    /// f12=new
    pub fn f12() -> Self { Self }

    /// f13=run
    #[must_use]
    pub fn f13(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();
        let employee_ids: HashSet<&str> = ds.f8();

        let performed_hours: std::collections::HashMap<(String, String, String), f64> = ds.s9
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, lc| {
                let key = (lc.s31.clone(), lc.s32.clone(), lc.s33.clone());
                *acc.entry(key).or_insert(0.0) += lc.s34;
                acc
            });

        for br in &ds.s10 {
            let contract = ds.f6(&br.s36);
            let (cage_code, agency) = contract
                .map(|c| (c.s23.as_deref(), c.s24.as_deref()))
                .unwrap_or((None, None));

            if !employee_ids.contains(br.s37.as_str()) {
                alerts.push(alert(t11::E7, 95, 8,
                    &format!("Billed employee '{}' not in employee roster", br.s37),
                    Some(&br.s36), Some(&br.s37), cage_code, agency,
                    vec![t12::E12, t12::E14],
                ));
            }

            if let Some(emp) = ds.f7(&br.s37) {
                if !emp.s30 {
                    alerts.push(alert(t11::E8, 70, 5,
                        &format!("Billed employee '{}' has no floorcheck verification", br.s37),
                        Some(&br.s36), Some(&br.s37), cage_code, agency,
                        vec![t12::E12],
                    ));
                }
            }

            let key = (br.s36.clone(), br.s37.clone(), br.s39.clone());
            let performed = performed_hours.get(&key).copied().unwrap_or(0.0);
            if performed < br.s38 - 0.01 {
                let (conf, sev) = if performed == 0.0 { (90, 8) } else { (80, 7) };
                alerts.push(alert(t11::E9, conf, sev,
                    &format!("Billed {} hrs for {}/{}/{} but only {} hrs performed",
                        br.s38, br.s36, br.s37, br.s39, performed),
                    Some(&br.s36), Some(&br.s37), cage_code, agency,
                    vec![t12::E12, t12::E13],
                ));
            }
        }
        alerts
    }
}

#[allow(clippy::too_many_arguments)]
fn alert(
    rule_id: t11, confidence: u8, severity: u8, summary: &str,
    contract_id: Option<&str>, employee_id: Option<&str>,
    cage_code: Option<&str>, agency: Option<&str>,
    predicate_acts: Vec<t12>,
) -> t5 {
    t5 {
        s11: t10::E3, s12: rule_id, s13: severity, s14: confidence,
        s15: summary.to_string(),
        s16: contract_id.map(String::from), s17: employee_id.map(String::from),
        s18: cage_code.map(String::from), s19: agency.map(String::from),
        s20: Some(predicate_acts), s21: Some(f20()),
    }
}
