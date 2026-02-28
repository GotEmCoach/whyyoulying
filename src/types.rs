//! Core types for fraud detection.
//!
//! Domain model per TRIPLE_SIMS_ARCH.md: Contract, Employee, LaborCharge, BillingRecord.

use serde::{Deserialize, Serialize};

/// Fraud classification per DoD IG scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FraudType {
    LaborCategory,
    GhostBilling,
}

/// Rule ID for audit trail and chain of custody (Sim 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleId {
    LaborVariance,
    LaborQualBelow,
    GhostNoEmployee,
    GhostNotVerified,
    GhostBilledNotPerformed,
}

/// Alert produced by a detector for fraud referral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub fraud_type: FraudType,
    pub rule_id: RuleId,
    pub severity: u8,
    pub summary: String,
    pub contract_id: Option<String>,
    pub employee_id: Option<String>,
    pub timestamp: Option<String>,
}

// --- Domain entities (TRIPLE_SIMS_ARCH §1) ---

/// Contract proposal/requirements: labor categories and min quals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub cage_code: Option<String>,
    pub agency: Option<String>,
    /// Map labor_cat → min qualification level.
    pub labor_cats: std::collections::HashMap<String, String>,
}

/// Employee qualifications vs charged category.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    /// Qualification levels (e.g. ["Senior", "BA"]).
    pub quals: Vec<String>,
    /// Minimum labor category this employee qualifies for.
    pub labor_cat_min: Option<String>,
    /// Floorcheck verified (DCAA 13500).
    pub verified: bool,
}

/// Actual labor charged (timesheet/DCAA).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaborCharge {
    pub contract_id: String,
    pub employee_id: String,
    pub labor_cat: String,
    pub hours: f64,
    pub rate: Option<f64>,
}

/// What was billed to gov.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingRecord {
    pub contract_id: String,
    pub employee_id: String,
    pub billed_hours: f64,
    pub billed_cat: String,
    pub period: Option<String>,
}
