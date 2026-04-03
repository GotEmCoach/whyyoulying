// Unlicense — cochranblock.org
//! Labor category fraud detection. P13 compressed.
//!
//! Red flags: unapproved categories, employees below min quals, rate overbilling.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;

const CAT_ORDER: &[&str] = &["Junior", "Mid", "Senior", "Lead", "Principal"];

fn category_level(cat: &str) -> usize {
    CAT_ORDER.iter().position(|&c| c.eq_ignore_ascii_case(cat)).unwrap_or(0)
}

/// t13=LaborDetector
pub struct t13 {
    /// s41=threshold_pct
    pub s41: f64,
}

impl t13 {
    /// f10=new
    pub fn f10(threshold_pct: f64) -> Self { Self { s41: threshold_pct } }

    /// f11=run
    #[must_use]
    pub fn f11(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();

        for lc in &ds.s9 {
            let contract = ds.f6(&lc.s31);
            if let Some(c) = contract {
                if !c.s25.contains_key(&lc.s33) {
                    alerts.push(alert(
                        t11::E4, 85, 6,
                        &format!("Labor category '{}' not in contract {}", lc.s33, lc.s31),
                        Some(&lc.s31), Some(&lc.s32),
                        c.s23.as_deref(), c.s24.as_deref(),
                        vec![t12::E12],
                    ));
                }
            }

            // Rate overbilling: charged rate exceeds contract rate by > threshold_pct
            if let (Some(c), Some(charged_rate)) = (contract, lc.s35) {
                if let Some(&contract_rate) = c.s26.get(&lc.s33) {
                    if contract_rate > 0.0 {
                        let variance_pct = ((charged_rate - contract_rate) / contract_rate) * 100.0;
                        if variance_pct > self.s41 {
                            alerts.push(alert(
                                t11::E6, 85, 7,
                                &format!(
                                    "Rate ${:.2}/hr exceeds contract ${:.2}/hr by {:.1}% (threshold {:.1}%) for {}/{}",
                                    charged_rate, contract_rate, variance_pct, self.s41, lc.s31, lc.s33
                                ),
                                Some(&lc.s31), Some(&lc.s32),
                                c.s23.as_deref(), c.s24.as_deref(),
                                vec![t12::E12, t12::E13],
                            ));
                        }
                    }
                }
            }

            if let Some(emp) = ds.f7(&lc.s32) {
                if let Some(ref min_cat) = emp.s29 {
                    if category_level(&lc.s33) > category_level(min_cat) {
                        let c = contract;
                        alerts.push(alert(
                            t11::E5, 90, 7,
                            &format!("Employee {} charged as '{}' but qualifies only for '{}'", lc.s32, lc.s33, min_cat),
                            Some(&lc.s31), Some(&lc.s32),
                            c.and_then(|x| x.s23.as_ref()).map(|s| s.as_str()),
                            c.and_then(|x| x.s24.as_ref()).map(|s| s.as_str()),
                            vec![t12::E12, t12::E13],
                        ));
                    }
                }
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
        s11: t10::E2, s12: rule_id, s13: severity, s14: confidence,
        s15: summary.to_string(),
        s16: contract_id.map(String::from), s17: employee_id.map(String::from),
        s18: cage_code.map(String::from), s19: agency.map(String::from),
        s20: Some(predicate_acts), s21: Some(f20()),
    }
}
