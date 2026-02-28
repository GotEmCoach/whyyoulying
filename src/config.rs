//! Configuration for data sources and detection thresholds.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub labor_variance_threshold_pct: f64,
    pub data_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            labor_variance_threshold_pct: 15.0,
            data_path: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)
            .with_context(|| format!("read config: {}", path.display()))?;
        let cfg: Self = serde_json::from_str(&s)
            .with_context(|| format!("parse config: {}", path.display()))?;
        if cfg.labor_variance_threshold_pct <= 0.0 || cfg.labor_variance_threshold_pct > 100.0 {
            anyhow::bail!("labor_variance_threshold_pct must be in (0, 100]");
        }
        Ok(cfg)
    }

    pub fn apply_cli_overrides(&mut self, data_path: Option<String>, threshold: Option<f64>) {
        if let Some(p) = data_path {
            self.data_path = Some(p);
        }
        if let Some(t) = threshold {
            self.labor_variance_threshold_pct = t;
        }
    }
}
