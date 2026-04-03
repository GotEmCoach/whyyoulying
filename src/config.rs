// Unlicense — cochranblock.org
//! Configuration for data sources and detection thresholds. P13 compressed.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// t2=ConfigError
#[derive(Debug, Error)]
pub enum t2 {
    #[error("labor_variance_threshold_pct must be in (0, 100], got {0}")]
    E1(f64),
}

/// t1=Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct t1 {
    /// s1=labor_variance_threshold_pct
    #[serde(rename = "labor_variance_threshold_pct")]
    pub s1: f64,
    /// s2=data_path
    #[serde(rename = "data_path")]
    pub s2: Option<String>,
    /// s3=min_confidence. 0-100 (S4 false-positive control).
    #[serde(rename = "min_confidence", default = "default_s3")]
    pub s3: u8,
    /// s4=filter_agency. DoD nexus.
    #[serde(rename = "filter_agency")]
    pub s4: Option<String>,
    /// s5=filter_cage_code. DoD nexus.
    #[serde(rename = "filter_cage_code")]
    pub s5: Option<String>,
    /// s6=max_hours_per_period. TIME_OVERCHARGE threshold.
    #[serde(rename = "max_hours_per_period", default = "default_s6")]
    pub s6: f64,
    /// s67=min_loss. Filter alerts below estimated dollar loss.
    #[serde(rename = "min_loss", default)]
    pub s67: Option<f64>,
}

fn default_s3() -> u8 { 50 }
fn default_s6() -> f64 { 176.0 }

impl Default for t1 {
    fn default() -> Self {
        Self { s1: 15.0, s2: None, s3: 50, s4: None, s5: None, s6: 176.0, s67: None }
    }
}

impl t1 {
    /// f1=load
    pub fn f1() -> Result<Self> { Ok(Self::default()) }

    /// f2=load_from_path
    pub fn f2(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)
            .with_context(|| format!("read config: {}", path.display()))?;
        let cfg: Self = serde_json::from_str(&s)
            .with_context(|| format!("parse config: {}", path.display()))?;
        if cfg.s1 <= 0.0 || cfg.s1 > 100.0 {
            return Err(t2::E1(cfg.s1).into());
        }
        Ok(cfg)
    }

    /// f3=apply_cli_overrides
    pub fn f3(
        &mut self,
        data_path: Option<String>,
        threshold: Option<f64>,
        min_confidence: Option<u8>,
        filter_agency: Option<String>,
        filter_cage_code: Option<String>,
    ) -> Result<(), t2> {
        if let Some(p) = data_path { self.s2 = Some(p); }
        if let Some(t) = threshold {
            if t <= 0.0 || t > 100.0 { return Err(t2::E1(t)); }
            self.s1 = t;
        }
        if let Some(c) = min_confidence { self.s3 = c; }
        if filter_agency.is_some() { self.s4 = filter_agency; }
        if filter_cage_code.is_some() { self.s5 = filter_cage_code; }
        Ok(())
    }

    /// f21=apply_min_loss
    pub fn f21(&mut self, min_loss: Option<f64>) {
        if min_loss.is_some() { self.s67 = min_loss; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn default_has_valid_threshold() {
        let c = t1::default();
        assert!(c.s1 > 0.0);
        assert!(c.s1 <= 100.0);
        assert!(c.s3 <= 100);
    }

    #[test]
    fn load_succeeds_with_valid_config() {
        let c = t1::f1().unwrap();
        assert!(c.s1 > 0.0 && c.s1 <= 100.0);
        assert!(c.s3 <= 100);
    }

    #[test]
    fn load_from_path_valid() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp.as_file(), r#"{{"labor_variance_threshold_pct":20.0,"data_path":"/x"}}"#).unwrap();
        let c = t1::f2(tmp.path()).unwrap();
        assert_eq!(c.s1, 20.0);
        assert_eq!(c.s2.as_deref(), Some("/x"));
    }

    #[test]
    fn load_from_path_rejects_zero_threshold() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp.as_file(), r#"{{"labor_variance_threshold_pct":0}}"#).unwrap();
        assert!(t1::f2(tmp.path()).is_err());
    }

    #[test]
    fn load_from_path_rejects_over_100_threshold() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp.as_file(), r#"{{"labor_variance_threshold_pct":101}}"#).unwrap();
        assert!(t1::f2(tmp.path()).is_err());
    }

    #[test]
    fn apply_cli_overrides() {
        let mut c = t1::default();
        c.f3(Some("x".into()), Some(25.0), Some(80), Some("DoD".into()), None).unwrap();
        assert_eq!(c.s2.as_deref(), Some("x"));
        assert_eq!(c.s1, 25.0);
        assert_eq!(c.s3, 80);
        assert_eq!(c.s4.as_deref(), Some("DoD"));
    }

    #[test]
    fn apply_cli_overrides_cage_code() {
        let mut c = t1::default();
        c.f3(None, None, None, None, Some("1ABC2".into())).unwrap();
        assert_eq!(c.s5.as_deref(), Some("1ABC2"));
    }

    #[test]
    fn apply_cli_overrides_rejects_invalid_threshold() {
        let mut c = t1::default();
        assert!(c.f3(None, Some(0.0), None, None, None).is_err());
        assert!(c.f3(None, Some(101.0), None, None, None).is_err());
    }

    #[test]
    fn load_from_path_missing_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        let p = tmp.path().join("nonexistent.json");
        assert!(t1::f2(&p).is_err());
    }

    #[test]
    fn load_from_path_invalid_json() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp.as_file(), "not json").unwrap();
        assert!(t1::f2(tmp.path()).is_err());
    }

    #[test]
    fn load_from_path_uses_default_min_confidence() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp.as_file(), r#"{{"labor_variance_threshold_pct":10}}"#).unwrap();
        let c = t1::f2(tmp.path()).unwrap();
        assert_eq!(c.s3, 50);
    }
}
