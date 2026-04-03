// Unlicense — cochranblock.org
//! Proactive detection of Labor Category Fraud and Ghost Billing. P13 compressed.
#![allow(non_camel_case_types)] // P13 compression: t+num type names
//!
//! Supports DoD IG and FBI fraud investigator workflows per DoDI 5505.02/03
//! and Attorney General Guidelines. See docs/compression_map.md.

#[cfg(feature = "tests")]
pub mod tests;
pub mod config;
pub mod data;
pub mod demo;
pub mod detect;
pub mod export;
pub mod types;
pub mod util;
#[cfg(target_os = "android")]
pub mod android_jni;

pub use config::t1 as Config;
pub use data::{t3 as Dataset, t4 as Ingest};
pub use detect::{
    duplicate::t16 as DuplicateDetector,
    ghost::t14 as GhostDetector,
    labor::t13 as LaborDetector,
    rate_escalation::t23 as RateEscalationDetector,
    subcontractor::t22 as SubcontractorDetector,
    time::t15 as TimeDetector,
};
pub use types::{
    t5 as Alert, t9 as BillingRecord, t6 as Contract, t7 as Employee,
    t10 as FraudType, t8 as LaborCharge, t12 as PredicateAct, t11 as RuleId,
};
