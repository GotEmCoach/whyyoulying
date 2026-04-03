// Unlicense — cochranblock.org
//! Data ingestion and normalization. P13 compressed.

use crate::config::t1;
use crate::types::{t6, t7, t8, t9};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// t3=Dataset. Normalized dataset for detection pipeline.
#[derive(Debug, Clone, Default)]
pub struct t3 {
    /// s7=contracts
    pub s7: HashMap<String, t6>,
    /// s8=employees
    pub s8: HashMap<String, t7>,
    /// s9=labor_charges
    pub s9: Vec<t8>,
    /// s10=billing_records
    pub s10: Vec<t9>,
}

impl t3 {
    /// f6=contract_by_id
    pub fn f6(&self, id: &str) -> Option<&t6> { self.s7.get(id) }

    /// f7=employee_by_id
    pub fn f7(&self, id: &str) -> Option<&t7> { self.s8.get(id) }

    /// f8=employee_ids
    pub fn f8(&self) -> HashSet<&str> {
        self.s8.keys().map(|s| s.as_str()).collect()
    }

    /// f9=nexus_contract_ids. DoD nexus filter (D5).
    pub fn f9(&self, filter_agency: Option<&str>, filter_cage_code: Option<&str>) -> HashSet<&str> {
        if filter_agency.is_none() && filter_cage_code.is_none() {
            return self.s7.keys().map(|s| s.as_str()).collect();
        }
        self.s7.values().filter(|c| {
            let agency_ok = filter_agency
                .map(|a| c.s24.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(a)))
                .unwrap_or(true);
            let cage_ok = filter_cage_code
                .map(|g| c.s23.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(g)))
                .unwrap_or(true);
            agency_ok && cage_ok
        }).map(|c| c.s22.as_str()).collect()
    }
}

/// t4=Ingest
pub struct t4;

impl t4 {
    /// f4=load. Load and normalize data from config.s2.
    pub fn f4(config: &t1) -> Result<t3> {
        let path = config.s2.as_deref().context("data_path required for ingest")?;
        Self::f5(Path::new(path))
    }

    /// f5=load_from_path. JSON preferred; falls back to CSV.
    pub fn f5(path: &Path) -> Result<t3> {
        let mut ds = t3::default();

        // Contracts
        let json = path.join("contracts.json");
        let csv = path.join("contracts.csv");
        if json.exists() {
            let s = std::fs::read_to_string(&json).with_context(|| format!("read {}", json.display()))?;
            let raw: Vec<t6> = serde_json::from_str(&s).with_context(|| format!("parse {}", json.display()))?;
            ds.s7 = raw.into_iter().map(|c| (c.s22.clone(), c)).collect();
        } else if csv.exists() {
            for row in f22(&csv)? {
                let c = t6 {
                    s22: row.get("id").cloned().unwrap_or_default(),
                    s23: row.get("cage_code").cloned().filter(|s| !s.is_empty()),
                    s24: row.get("agency").cloned().filter(|s| !s.is_empty()),
                    s25: HashMap::new(), s26: HashMap::new(),
                };
                ds.s7.insert(c.s22.clone(), c);
            }
        }

        // Employees
        let json = path.join("employees.json");
        let csv = path.join("employees.csv");
        if json.exists() {
            let s = std::fs::read_to_string(&json).with_context(|| format!("read {}", json.display()))?;
            let raw: Vec<t7> = serde_json::from_str(&s).with_context(|| format!("parse {}", json.display()))?;
            ds.s8 = raw.into_iter().map(|e| (e.s27.clone(), e)).collect();
        } else if csv.exists() {
            for row in f22(&csv)? {
                let e = t7 {
                    s27: row.get("id").cloned().unwrap_or_default(),
                    s28: row.get("quals").map(|q| q.split(';').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                    s29: row.get("labor_cat_min").cloned().filter(|s| !s.is_empty()),
                    s30: row.get("verified").is_some_and(|v| v == "true" || v == "1"),
                };
                ds.s8.insert(e.s27.clone(), e);
            }
        }

        // Labor charges
        let json = path.join("labor_charges.json");
        let csv = path.join("labor_charges.csv");
        if json.exists() {
            let s = std::fs::read_to_string(&json).with_context(|| format!("read {}", json.display()))?;
            ds.s9 = serde_json::from_str(&s).with_context(|| format!("parse {}", json.display()))?;
        } else if csv.exists() {
            for row in f22(&csv)? {
                ds.s9.push(t8 {
                    s31: row.get("contract_id").cloned().unwrap_or_default(),
                    s32: row.get("employee_id").cloned().unwrap_or_default(),
                    s33: row.get("labor_cat").cloned().unwrap_or_default(),
                    s34: row.get("hours").and_then(|h| h.parse().ok()).unwrap_or(0.0),
                    s35: row.get("rate").and_then(|r| r.parse().ok()),
                });
            }
        }

        // Billing records
        let json = path.join("billing_records.json");
        let csv = path.join("billing_records.csv");
        if json.exists() {
            let s = std::fs::read_to_string(&json).with_context(|| format!("read {}", json.display()))?;
            ds.s10 = serde_json::from_str(&s).with_context(|| format!("parse {}", json.display()))?;
        } else if csv.exists() {
            for row in f22(&csv)? {
                ds.s10.push(t9 {
                    s36: row.get("contract_id").cloned().unwrap_or_default(),
                    s37: row.get("employee_id").cloned().unwrap_or_default(),
                    s38: row.get("billed_hours").and_then(|h| h.parse().ok()).unwrap_or(0.0),
                    s39: row.get("billed_cat").cloned().unwrap_or_default(),
                    s40: row.get("period").cloned().filter(|s| !s.is_empty()),
                });
            }
        }

        Ok(ds)
    }
}

/// f22=parse_csv. Minimal CSV parser: header row + data rows. Handles quoted fields.
fn f22(path: &Path) -> Result<Vec<HashMap<String, String>>> {
    let content = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut lines = content.lines();
    let header_line = lines.next().context("CSV file is empty")?;
    let headers: Vec<&str> = parse_csv_row(header_line);
    let mut rows = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() { continue; }
        let fields = parse_csv_row(line);
        let mut row = HashMap::new();
        for (i, h) in headers.iter().enumerate() {
            if let Some(val) = fields.get(i) {
                row.insert(h.to_string(), val.to_string());
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

/// Parse a single CSV row, handling quoted fields with embedded commas.
fn parse_csv_row(line: &str) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let bytes = line.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'"' => in_quotes = !in_quotes,
            b',' if !in_quotes => {
                fields.push(unquote(&line[start..i]));
                start = i + 1;
            }
            _ => {}
        }
    }
    fields.push(unquote(&line[start..]));
    fields
}

fn unquote(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        &s[1..s.len()-1]
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{t6, t7};
    use std::collections::HashMap;

    #[test]
    fn load_from_path_empty_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert!(ds.s7.is_empty());
        assert!(ds.s8.is_empty());
        assert!(ds.s9.is_empty());
        assert!(ds.s10.is_empty());
    }

    #[test]
    fn load_from_path_partial() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("contracts.json"),
            r#"[{"id":"C1","cage_code":"1X","agency":"DoD","labor_cats":{}}]"#,
        ).unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s7.len(), 1);
        assert_eq!(ds.f6("C1").unwrap().s22, "C1");
        assert!(ds.s8.is_empty());
    }

    #[test]
    fn contract_by_id() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 {
            s22: "C1".into(), s23: Some("1X".into()), s24: Some("DoD".into()),
            s25: HashMap::new(), ..Default::default()
        });
        assert!(ds.f6("C1").is_some());
        assert!(ds.f6("C2").is_none());
    }

    #[test]
    fn nexus_contract_ids_no_filter_returns_all() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), ..Default::default() });
        let ids = ds.f9(None, None);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_filter_agency() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s24: Some("DoD".into()), ..Default::default() });
        ds.s7.insert("C2".into(), t6 { s22: "C2".into(), s24: Some("GSA".into()), ..Default::default() });
        let ids = ds.f9(Some("DoD"), None);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_filter_cage() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s23: Some("1ABC".into()), ..Default::default() });
        let ids = ds.f9(None, Some("1ABC"));
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn nexus_contract_ids_case_insensitive() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s24: Some("DoD".into()), ..Default::default() });
        let ids = ds.f9(Some("dod"), None);
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn nexus_contract_ids_both_filters() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), s23: Some("1X".into()), s24: Some("DoD".into()), ..Default::default() });
        ds.s7.insert("C2".into(), t6 { s22: "C2".into(), s23: Some("2Y".into()), s24: Some("DoD".into()), ..Default::default() });
        let ids = ds.f9(Some("DoD"), Some("1X"));
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_empty_ds() {
        let ds = t3::default();
        assert!(ds.f9(None, None).is_empty());
    }

    #[test]
    fn nexus_contract_ids_filter_excludes_missing_agency() {
        let mut ds = t3::default();
        ds.s7.insert("C1".into(), t6 { s22: "C1".into(), ..Default::default() });
        let ids = ds.f9(Some("DoD"), None);
        assert!(ids.is_empty());
    }

    #[test]
    fn employee_by_id() {
        let mut ds = t3::default();
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), s28: vec!["BA".into()], ..Default::default() });
        assert!(ds.f7("E1").is_some());
        assert!(ds.f7("E2").is_none());
    }

    #[test]
    fn employee_ids() {
        let mut ds = t3::default();
        ds.s8.insert("E1".into(), t7 { s27: "E1".into(), ..Default::default() });
        ds.s8.insert("E2".into(), t7 { s27: "E2".into(), ..Default::default() });
        let ids = ds.f8();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains("E1"));
        assert!(ids.contains("E2"));
    }

    #[test]
    fn load_from_path_all_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.json"), r#"[{"id":"C1","labor_cats":{}}]"#).unwrap();
        std::fs::write(tmp.path().join("employees.json"), r#"[{"id":"E1","quals":[],"verified":false}]"#).unwrap();
        std::fs::write(tmp.path().join("labor_charges.json"), r#"[{"contract_id":"C1","employee_id":"E1","labor_cat":"X","hours":1.0}]"#).unwrap();
        std::fs::write(tmp.path().join("billing_records.json"), r#"[{"contract_id":"C1","employee_id":"E1","billed_hours":1.0,"billed_cat":"X"}]"#).unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s7.len(), 1);
        assert_eq!(ds.s8.len(), 1);
        assert_eq!(ds.s9.len(), 1);
        assert_eq!(ds.s10.len(), 1);
    }

    #[test]
    fn load_from_path_invalid_json_fails() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.json"), "not json").unwrap();
        assert!(t4::f5(tmp.path()).is_err());
    }

    // --- CSV ingest ---

    #[test]
    fn csv_ingest_contracts() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.csv"), "id,cage_code,agency\nC1,1X,DoD\nC2,,GSA\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s7.len(), 2);
        assert_eq!(ds.f6("C1").unwrap().s23.as_deref(), Some("1X"));
        assert_eq!(ds.f6("C2").unwrap().s23, None); // empty → None
    }

    #[test]
    fn csv_ingest_employees() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("employees.csv"), "id,quals,labor_cat_min,verified\nE1,BA;Senior,Junior,true\nE2,,,false\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s8.len(), 2);
        assert_eq!(ds.f7("E1").unwrap().s28, vec!["BA", "Senior"]);
        assert!(ds.f7("E1").unwrap().s30);
        assert!(!ds.f7("E2").unwrap().s30);
    }

    #[test]
    fn csv_ingest_labor_charges() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("labor_charges.csv"), "contract_id,employee_id,labor_cat,hours,rate\nC1,E1,Senior,40.0,150.0\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s9.len(), 1);
        assert_eq!(ds.s9[0].s34, 40.0);
        assert_eq!(ds.s9[0].s35, Some(150.0));
    }

    #[test]
    fn csv_ingest_billing_records() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("billing_records.csv"), "contract_id,employee_id,billed_hours,billed_cat,period\nC1,E1,80.0,Senior,2026-01\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s10.len(), 1);
        assert_eq!(ds.s10[0].s38, 80.0);
        assert_eq!(ds.s10[0].s40.as_deref(), Some("2026-01"));
    }

    #[test]
    fn csv_ingest_quoted_fields() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.csv"), "id,cage_code,agency\n\"C1\",\"1X\",\"DoD\"\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert_eq!(ds.s7.len(), 1);
        assert_eq!(ds.f6("C1").unwrap().s24.as_deref(), Some("DoD"));
    }

    #[test]
    fn csv_ingest_json_preferred() {
        // When both .json and .csv exist, JSON should be used
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.json"), r#"[{"id":"J1","labor_cats":{}}]"#).unwrap();
        std::fs::write(tmp.path().join("contracts.csv"), "id,cage_code,agency\nC1,1X,DoD\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert!(ds.f6("J1").is_some()); // JSON wins
        assert!(ds.f6("C1").is_none());
    }

    #[test]
    fn csv_ingest_empty_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.csv"), "id,cage_code,agency\n").unwrap();
        let ds = t4::f5(tmp.path()).unwrap();
        assert!(ds.s7.is_empty()); // header only, no data
    }
}
