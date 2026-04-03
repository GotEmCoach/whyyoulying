// Unlicense — cochranblock.org
//! Labor category fraud detection. P13 compressed.
//!
//! Red flags: unapproved categories, employees below min quals, rate overbilling.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;

const CAT_ORDER: &[&str] = &["Junior", "Mid", "Senior", "Lead", "Principal"];

/// Map common aliases to canonical category names (case-insensitive).
fn normalize_category(cat: &str) -> Option<&'static str> {
    let lower = cat.to_ascii_lowercase();
    // Exact matches first (most common path)
    for &canonical in CAT_ORDER {
        if lower == canonical.to_ascii_lowercase() {
            return Some(canonical);
        }
    }
    // Alias matching — order matters, check longer/more specific patterns first
    if lower.contains("principal") || lower.contains("director") || lower.contains("fellow") {
        return Some("Principal");
    }
    if lower.contains("lead") || lower.contains("manager") || lower.contains("supervisor") {
        return Some("Lead");
    }
    if lower.starts_with("sr") || lower.contains("senior") || lower.contains("iii") {
        return Some("Senior");
    }
    if lower.starts_with("jr") || lower.contains("junior") || lower.contains("entry")
        || lower.contains("intern") || lower.contains("apprentice") || lower.contains(" i") && !lower.contains("ii")
    {
        return Some("Junior");
    }
    if lower.contains("mid") || lower.contains("ii") || lower.contains("associate")
        || lower.contains("journeyman") || lower.contains("analyst")
    {
        return Some("Mid");
    }
    None
}

fn category_level(cat: &str) -> Option<usize> {
    normalize_category(cat).and_then(|norm| CAT_ORDER.iter().position(|&c| c == norm))
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
                if !c.s25.keys().any(|k| k.eq_ignore_ascii_case(&lc.s33)) {
                    alerts.push(alert(
                        t11::E4, 85, 6,
                        &format!("Labor category '{}' not in contract {}", lc.s33, lc.s31),
                        Some(&lc.s31), Some(&lc.s32),
                        c.s23.as_deref(), c.s24.as_deref(),
                        vec![t12::E12], None,
                    ));
                }
            }

            // Rate overbilling: charged rate exceeds contract rate by > threshold_pct
            if let (Some(c), Some(charged_rate)) = (contract, lc.s35) {
                if let Some(&contract_rate) = c.s26.iter().find(|(k, _)| k.eq_ignore_ascii_case(&lc.s33)).map(|(_, v)| v) {
                    if contract_rate > 0.0 {
                        let variance_pct = ((charged_rate - contract_rate) / contract_rate) * 100.0;
                        if variance_pct > self.s41 {
                            let loss = (charged_rate - contract_rate) * lc.s34;
                            alerts.push(alert(
                                t11::E6, 85, 7,
                                &format!(
                                    "Rate ${:.2}/hr exceeds contract ${:.2}/hr by {:.1}% (threshold {:.1}%) for {}/{}",
                                    charged_rate, contract_rate, variance_pct, self.s41, lc.s31, lc.s33
                                ),
                                Some(&lc.s31), Some(&lc.s32),
                                c.s23.as_deref(), c.s24.as_deref(),
                                vec![t12::E12, t12::E13], Some(loss),
                            ));
                        }
                    }
                }
            }

            if let Some(emp) = ds.f7(&lc.s32) {
                if let Some(ref min_cat) = emp.s29 {
                    if let (Some(charged_lvl), Some(qual_lvl)) = (category_level(&lc.s33), category_level(min_cat)) {
                        if charged_lvl > qual_lvl {
                            let c = contract;
                            alerts.push(alert(
                                t11::E5, 90, 7,
                                &format!("Employee {} charged as '{}' but qualifies only for '{}'", lc.s32, lc.s33, min_cat),
                                Some(&lc.s31), Some(&lc.s32),
                                c.and_then(|x| x.s23.as_ref()).map(|s| s.as_str()),
                                c.and_then(|x| x.s24.as_ref()).map(|s| s.as_str()),
                                vec![t12::E12, t12::E13], None,
                            ));
                        }
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
    predicate_acts: Vec<t12>, estimated_loss: Option<f64>,
) -> t5 {
    t5 {
        s11: t10::E2, s12: rule_id, s13: severity, s14: confidence,
        s15: summary.to_string(),
        s16: contract_id.map(String::from), s17: employee_id.map(String::from),
        s18: cage_code.map(String::from), s19: agency.map(String::from),
        s20: Some(predicate_acts), s21: Some(f20()), s66: estimated_loss,
    }
}
