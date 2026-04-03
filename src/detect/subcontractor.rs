// Unlicense — cochranblock.org
//! Subcontractor fraud detection. P13 compressed.
//!
//! Red flag: employee marked as subcontractor but billed at prime rate/category.

use crate::data::t3;
use crate::types::{t5, t10, t11, t12};
use crate::util::f20;

/// t22=SubcontractorDetector
pub struct t22;

impl t22 {
    /// f23=new
    pub fn f23() -> Self { Self }

    /// f24=run
    #[must_use]
    pub fn f24(&self, ds: &t3) -> Vec<t5> {
        let mut alerts = Vec::new();

        for br in &ds.s10 {
            if let Some(emp) = ds.f7(&br.s37) {
                if emp.s70 == Some(true) {
                    let contract = ds.f6(&br.s36);
                    let (cage_code, agency) = contract
                        .map(|c| (c.s23.as_deref(), c.s24.as_deref()))
                        .unwrap_or((None, None));
                    let loss = contract
                        .and_then(|c| c.s26.iter().find(|(k, _)| k.eq_ignore_ascii_case(&br.s39)).map(|(_, v)| *v))
                        .map(|rate| br.s38 * rate);
                    alerts.push(t5 {
                        s11: t10::E15, s12: t11::E16, s13: 7, s14: 85,
                        s15: format!("Subcontractor '{}' billed as prime on {} at '{}' for {} hrs",
                            br.s37, br.s36, br.s39, br.s38),
                        s16: Some(br.s36.clone()), s17: Some(br.s37.clone()),
                        s18: cage_code.map(String::from), s19: agency.map(String::from),
                        s20: Some(vec![t12::E12, t12::E13]), s21: Some(f20()), s66: loss,
                    });
                }
            }
        }
        alerts
    }
}
