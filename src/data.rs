//! Data ingestion and normalization.

use crate::config::Config;
use crate::types::{BillingRecord, Contract, Employee, LaborCharge};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::Path;

/// Normalized dataset for detection pipeline.
#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub contracts: Vec<Contract>,
    pub employees: Vec<Employee>,
    pub labor_charges: Vec<LaborCharge>,
    pub billing_records: Vec<BillingRecord>,
}

impl Dataset {
    pub fn contract_by_id(&self, id: &str) -> Option<&Contract> {
        self.contracts.iter().find(|c| c.id == id)
    }

    pub fn employee_by_id(&self, id: &str) -> Option<&Employee> {
        self.employees.iter().find(|e| e.id == id)
    }

    pub fn employee_ids(&self) -> HashSet<&str> {
        self.employees.iter().map(|e| e.id.as_str()).collect()
    }
}

pub struct Ingest;

impl Ingest {
    pub fn new(_config: &Config) -> Result<Self> {
        Ok(Self)
    }

    /// Load and normalize data from data_path.

    pub fn load(config: &Config) -> Result<Dataset> {
        let path = config
            .data_path
            .as_deref()
            .context("data_path required for ingest")?;
        Self::load_from_path(Path::new(path))
    }

    pub fn load_from_path(path: &Path) -> Result<Dataset> {
        let mut ds = Dataset::default();

        let contracts_path = path.join("contracts.json");
        if contracts_path.exists() {
            let s = std::fs::read_to_string(&contracts_path)
                .with_context(|| format!("read {}", contracts_path.display()))?;
            let raw: Vec<Contract> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", contracts_path.display()))?;
            ds.contracts = raw;
        }

        let employees_path = path.join("employees.json");
        if employees_path.exists() {
            let s = std::fs::read_to_string(&employees_path)
                .with_context(|| format!("read {}", employees_path.display()))?;
            let raw: Vec<Employee> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", employees_path.display()))?;
            ds.employees = raw;
        }

        let labor_path = path.join("labor_charges.json");
        if labor_path.exists() {
            let s = std::fs::read_to_string(&labor_path)
                .with_context(|| format!("read {}", labor_path.display()))?;
            let raw: Vec<LaborCharge> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", labor_path.display()))?;
            ds.labor_charges = raw;
        }

        let billing_path = path.join("billing_records.json");
        if billing_path.exists() {
            let s = std::fs::read_to_string(&billing_path)
                .with_context(|| format!("read {}", billing_path.display()))?;
            let raw: Vec<BillingRecord> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", billing_path.display()))?;
            ds.billing_records = raw;
        }

        Ok(ds)
    }
}
