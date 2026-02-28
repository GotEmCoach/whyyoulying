//! Referral export (GAGAS structure) and audit log.

use crate::types::Alert;
use serde::Serialize;

/// GAGAS-compliant referral package for DoD IG / FBI.
#[derive(Debug, Serialize)]
pub struct ReferralPackage {
    pub generated_at: String,
    pub alert_count: usize,
    pub alerts: Vec<Alert>,
    pub audit_entries: Vec<AuditEntry>,
}

#[derive(Debug, Serialize)]
pub struct AuditEntry {
    pub rule_id: String,
    pub alert_index: usize,
    pub input_hash: String,
}

pub fn referral_package(alerts: &[Alert]) -> ReferralPackage {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let generated_at = chrono::Utc::now().to_rfc3339();
    let audit_entries: Vec<AuditEntry> = alerts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let mut hasher = DefaultHasher::new();
            a.contract_id.hash(&mut hasher);
            a.employee_id.hash(&mut hasher);
            a.summary.hash(&mut hasher);
            format!("{:?}", a.rule_id).hash(&mut hasher);
            AuditEntry {
                rule_id: format!("{:?}", a.rule_id),
                alert_index: i,
                input_hash: format!("{:x}", hasher.finish()),
            }
        })
        .collect();

    ReferralPackage {
        generated_at,
        alert_count: alerts.len(),
        alerts: alerts.to_vec(),
        audit_entries,
    }
}
