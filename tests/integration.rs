//! Integration tests — full pipeline, CLI behavior.

use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[test]
fn run_with_fixtures_produces_alerts() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .output()
        .unwrap();
    assert!(out.status.code() == Some(1));
    let stdout = String::from_utf8(out.stdout).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    assert!(!parsed.is_empty());
}

#[test]
fn run_with_min_confidence_filters() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .arg("--min-confidence")
        .arg("99")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    for a in &parsed {
        assert!(a["confidence"].as_u64().unwrap_or(0) >= 99);
    }
}

#[test]
fn ingest_subcommand() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("contracts.json"), "[]").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("ingest")
        .arg("--data-path")
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success());
}

#[test]
fn run_missing_data_path_fails() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("run")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("data_path") || stderr.contains("error"));
}

#[test]
fn run_with_agency_filter() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .arg("--agency")
        .arg("DoD")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    for a in &parsed {
        assert_eq!(a["agency"].as_str(), Some("DoD"));
    }
}

#[test]
fn run_csv_output() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .arg("--output")
        .arg("csv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("fraud_type"));
    assert!(stdout.contains("confidence"));
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines.len() >= 2, "CSV should have header + at least one row");
}

#[test]
fn export_referral_subcommand() {
    let tmp = TempDir::new().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .arg("export-referral")
        .arg("--path")
        .arg(tmp.path().join("ref.json"))
        .output()
        .unwrap();
    assert!(out.status.code() == Some(0) || out.status.code() == Some(1));
    let p = tmp.path().join("ref.json");
    assert!(p.exists());
    let content = std::fs::read_to_string(&p).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["document_type"].as_str().unwrap().contains("DoD"));
}

#[test]
fn export_referral_fbi() {
    let tmp = TempDir::new().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(fixtures_path())
        .arg("export-referral")
        .arg("--fbi")
        .arg("--path")
        .arg(tmp.path().join("fbi.json"))
        .output()
        .unwrap();
    assert!(out.status.code() == Some(0) || out.status.code() == Some(1));
    let content = std::fs::read_to_string(tmp.path().join("fbi.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(parsed["document_type"].as_str().unwrap().contains("FBI"));
}

#[test]
fn run_empty_data_exit_zero() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("contracts.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("employees.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("labor_charges.json"), "[]").unwrap();
    std::fs::write(tmp.path().join("billing_records.json"), "[]").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--data-path")
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert_eq!(stdout.trim(), "[]");
}

#[test]
fn test_flag_runs() {
    let out = Command::new(env!("CARGO_BIN_EXE_whyyoulying"))
        .arg("--test")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("PASS"));
}
