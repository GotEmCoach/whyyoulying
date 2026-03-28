//! whyyoulying CLI — proactive labor category and ghost billing detection. P13 compressed.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use whyyoulying::{Alert, Config, DuplicateDetector, GhostDetector, Ingest, LaborDetector, TimeDetector};

#[derive(Parser)]
#[command(name = "whyyoulying")]
#[command(about = "Proactive Labor Category Fraud and Ghost Billing detection")]
#[command(version)]
struct Cli {
    /// Dump SPDX SBOM (machine-readable) and exit
    #[arg(long, global = true)]
    sbom: bool,
    #[arg(long, global = true, help = "Config file path (JSON)")]
    config: Option<PathBuf>,
    #[arg(long, global = true, help = "Directory with contracts/employees/labor/billing JSON")]
    data_path: Option<PathBuf>,
    #[arg(long, global = true, value_parser = clap::value_parser!(f64), help = "Labor variance threshold 0-100 (default 15)")]
    threshold: Option<f64>,
    #[arg(long, global = true, value_parser = clap::value_parser!(u8).range(0..=100), help = "Min confidence 0-100 (S4 false-positive control)")]
    min_confidence: Option<u8>,
    #[arg(long, global = true, help = "DoD nexus: filter by agency (e.g. DoD, Army)")]
    agency: Option<String>,
    #[arg(long, global = true, help = "DoD nexus: filter by CAGE code")]
    cage_code: Option<String>,
    #[arg(long, short, global = true, default_value = "json", value_enum)]
    output: OutputFormat,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum OutputFormat { Json, Csv }

#[derive(Subcommand)]
enum Commands {
    /// Run labor + ghost detection, output alerts (default)
    Run,
    /// Load and validate data only
    Ingest {
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Export GAGAS referral package or FBI case-opening docs
    ExportReferral {
        #[arg(long)]
        path: Option<PathBuf>,
        #[arg(long, default_value_t = false, help = "FBI case-opening format (AG Guidelines)")]
        fbi: bool,
    },
    /// Print federal compliance docs (sbom, security, privacy, fips, cmmc, supply-chain, fedramp, itar, accessibility, federal-use-cases, ssdf, all)
    Govdocs {
        /// Document to display
        doc: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.sbom {
        print!("{}", live_spdx_sbom());
        std::process::exit(0);
    }

    let result = match &cli.command {
        None | Some(Commands::Run) => run(&cli),
        Some(Commands::Ingest { path }) => cmd_ingest(&cli, path.as_deref()),
        Some(Commands::ExportReferral { path, fbi }) => cmd_export_referral(&cli, path.as_deref(), *fbi),
        Some(Commands::Govdocs { doc }) => cmd_govdocs(doc.as_deref()),
    };
    match result {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => { eprintln!("error: {e:?}"); std::process::exit(2); }
    }
}

fn load_config(cli: &Cli) -> Result<whyyoulying::config::t1> {
    let mut cfg = if let Some(ref p) = cli.config {
        Config::f2(p)?
    } else {
        Config::f1()?
    };
    cfg.f3(
        cli.data_path.as_ref().map(|p| p.to_string_lossy().into_owned()),
        cli.threshold, cli.min_confidence,
        cli.agency.clone(), cli.cage_code.clone(),
    )?;
    Ok(cfg)
}

fn run(cli: &Cli) -> Result<i32> {
    let config = load_config(cli)?;
    let data_path = config.s2.as_ref().map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("--data-path or config data_path required"))?;

    eprintln!("loading data from {}", data_path.display());
    let ds = Ingest::f5(&data_path)?;
    eprintln!("loaded {} contracts, {} employees, {} labor charges, {} billing records",
        ds.s7.len(), ds.s8.len(), ds.s9.len(), ds.s10.len());

    let labor = LaborDetector::f10(config.s1);
    let ghost = GhostDetector::f12();
    let time = TimeDetector::f14(config.s6);
    let duplicate = DuplicateDetector::f16();
    let mut alerts: Vec<Alert> = labor.f11(&ds).into_iter()
        .chain(ghost.f13(&ds))
        .chain(time.f15(&ds))
        .chain(duplicate.f17(&ds))
        .collect();

    let nexus_ids = ds.f9(config.s4.as_deref(), config.s5.as_deref());
    let has_nexus_filter = config.s4.is_some() || config.s5.is_some();
    alerts.retain(|a| {
        if a.s14 < config.s3 { return false; }
        match a.s16.as_ref() {
            Some(id) => nexus_ids.contains(id.as_str()),
            None if !has_nexus_filter => true,
            None => {
                let agency_ok = config.s4.as_ref().is_none_or(|fa| {
                    a.s19.as_ref().is_some_and(|x| x.eq_ignore_ascii_case(fa))
                });
                let cage_ok = config.s5.as_ref().is_none_or(|fc| {
                    a.s18.as_ref().is_some_and(|x| x.eq_ignore_ascii_case(fc))
                });
                agency_ok && cage_ok
            }
        }
    });

    match cli.output {
        OutputFormat::Json => { println!("{}", serde_json::to_string_pretty(&alerts)?); }
        OutputFormat::Csv => {
            println!("fraud_type,rule_id,severity,confidence,summary,contract_id,employee_id,cage_code,agency,timestamp");
            for a in &alerts {
                println!("{},{},{},{},{},{},{},{},{},{}",
                    a.s11, a.s12, a.s13, a.s14, escape_csv(&a.s15),
                    a.s16.as_deref().unwrap_or(""), a.s17.as_deref().unwrap_or(""),
                    a.s18.as_deref().unwrap_or(""), a.s19.as_deref().unwrap_or(""),
                    a.s21.as_deref().unwrap_or(""));
            }
        }
    }
    Ok(if alerts.is_empty() { 0 } else { 1 })
}

fn cmd_ingest(cli: &Cli, path: Option<&std::path::Path>) -> Result<i32> {
    let config = load_config(cli)?;
    let p = path.map(PathBuf::from)
        .or_else(|| config.s2.as_ref().map(PathBuf::from))
        .ok_or_else(|| anyhow::anyhow!("--path or --data-path required"))?;
    let ds = Ingest::f5(&p)?;
    eprintln!("ingested: {} contracts, {} employees, {} labor charges, {} billing records",
        ds.s7.len(), ds.s8.len(), ds.s9.len(), ds.s10.len());
    Ok(0)
}

fn cmd_export_referral(cli: &Cli, path: Option<&std::path::Path>, fbi_format: bool) -> Result<i32> {
    let config = load_config(cli)?;
    let data_path = config.s2.as_ref().map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("--data-path required for export-referral"))?;
    let ds = Ingest::f5(&data_path)?;
    let labor = LaborDetector::f10(config.s1);
    let ghost = GhostDetector::f12();
    let time = TimeDetector::f14(config.s6);
    let duplicate = DuplicateDetector::f16();
    let mut alerts: Vec<Alert> = labor.f11(&ds).into_iter()
        .chain(ghost.f13(&ds))
        .chain(time.f15(&ds))
        .chain(duplicate.f17(&ds))
        .collect();

    let nexus_ids = ds.f9(config.s4.as_deref(), config.s5.as_deref());
    let has_nexus_filter = config.s4.is_some() || config.s5.is_some();
    alerts.retain(|a| {
        if a.s14 < config.s3 { return false; }
        match a.s16.as_ref() {
            Some(id) => nexus_ids.contains(id.as_str()),
            None if !has_nexus_filter => true,
            None => {
                let agency_ok = config.s4.as_ref().is_none_or(|fa| {
                    a.s19.as_ref().is_some_and(|x| x.eq_ignore_ascii_case(fa))
                });
                let cage_ok = config.s5.as_ref().is_none_or(|fc| {
                    a.s18.as_ref().is_some_and(|x| x.eq_ignore_ascii_case(fc))
                });
                agency_ok && cage_ok
            }
        }
    });

    let out = if fbi_format {
        serde_json::to_string_pretty(&whyyoulying::export::f19(&alerts))?
    } else {
        serde_json::to_string_pretty(&whyyoulying::export::f18(&alerts))?
    };

    if let Some(p) = path {
        std::fs::write(p, &out)?;
        eprintln!("wrote {} package to {}", if fbi_format { "FBI case-opening" } else { "GAGAS referral" }, p.display());
    } else {
        println!("{out}");
    }
    Ok(if alerts.is_empty() { 0 } else { 1 })
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

// --- Govdocs: baked-in compliance docs ---

const GOVDOC_SBOM: &str = include_str!("../govdocs/SBOM.md");
const GOVDOC_SECURITY: &str = include_str!("../govdocs/SECURITY.md");
const GOVDOC_PRIVACY: &str = include_str!("../govdocs/PRIVACY.md");
const GOVDOC_FIPS: &str = include_str!("../govdocs/FIPS.md");
const GOVDOC_CMMC: &str = include_str!("../govdocs/CMMC.md");
const GOVDOC_SUPPLY_CHAIN: &str = include_str!("../govdocs/SUPPLY_CHAIN.md");
const GOVDOC_FEDRAMP: &str = include_str!("../govdocs/FedRAMP_NOTES.md");
const GOVDOC_ITAR: &str = include_str!("../govdocs/ITAR_EAR.md");
const GOVDOC_ACCESSIBILITY: &str = include_str!("../govdocs/ACCESSIBILITY.md");
const GOVDOC_SSDF: &str = include_str!("../govdocs/SSDF.md");
const GOVDOC_FEDERAL: &str = include_str!("../govdocs/FEDERAL_USE_CASES.md");
const BAKED_CARGO_TOML: &str = include_str!("../Cargo.toml");

fn cmd_govdocs(doc: Option<&str>) -> Result<i32> {
    match doc {
        Some("sbom") => println!("{GOVDOC_SBOM}"),
        Some("security") => println!("{GOVDOC_SECURITY}"),
        Some("privacy") => println!("{GOVDOC_PRIVACY}"),
        Some("fips") => println!("{GOVDOC_FIPS}"),
        Some("cmmc") => println!("{GOVDOC_CMMC}"),
        Some("supply-chain") => println!("{GOVDOC_SUPPLY_CHAIN}"),
        Some("fedramp") => println!("{GOVDOC_FEDRAMP}"),
        Some("itar") => println!("{GOVDOC_ITAR}"),
        Some("accessibility") => println!("{GOVDOC_ACCESSIBILITY}"),
        Some("ssdf") => println!("{GOVDOC_SSDF}"),
        Some("federal-use-cases") => println!("{GOVDOC_FEDERAL}"),
        Some("all") => {
            for (name, content) in govdoc_list() {
                println!("=== {name} ===\n{content}");
            }
        }
        None | Some(_) => {
            println!("whyyoulying v{} — federal compliance docs\n", env!("CARGO_PKG_VERSION"));
            println!("Available documents:");
            for (name, _) in govdoc_list() {
                println!("  whyyoulying govdocs {name}");
            }
            println!("\n  whyyoulying govdocs all         — print all");
            println!("  whyyoulying --sbom              — machine-readable SPDX SBOM");
        }
    }
    Ok(0)
}

fn govdoc_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("sbom", GOVDOC_SBOM),
        ("security", GOVDOC_SECURITY),
        ("privacy", GOVDOC_PRIVACY),
        ("fips", GOVDOC_FIPS),
        ("cmmc", GOVDOC_CMMC),
        ("supply-chain", GOVDOC_SUPPLY_CHAIN),
        ("fedramp", GOVDOC_FEDRAMP),
        ("itar", GOVDOC_ITAR),
        ("accessibility", GOVDOC_ACCESSIBILITY),
        ("ssdf", GOVDOC_SSDF),
        ("federal-use-cases", GOVDOC_FEDERAL),
    ]
}

/// Live SPDX 2.3 SBOM generated at runtime from baked Cargo.toml.
fn live_spdx_sbom() -> String {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let pkg_license = env!("CARGO_PKG_LICENSE");
    let timestamp = whyyoulying::util::f20();

    let mut out = format!(
        "SPDXVersion: SPDX-2.3\n\
         DataLicense: CC0-1.0\n\
         SPDXID: SPDXRef-DOCUMENT\n\
         DocumentName: {pkg_name}-{pkg_version}\n\
         DocumentNamespace: https://github.com/gotemcoach/{pkg_name}/spdx/{pkg_version}\n\
         Creator: Tool: whyyoulying-{pkg_version}\n\
         Created: {timestamp}\n\
         \n\
         PackageName: {pkg_name}\n\
         SPDXID: SPDXRef-Package\n\
         PackageVersion: {pkg_version}\n\
         PackageDownloadLocation: https://github.com/gotemcoach/{pkg_name}\n\
         PackageLicenseConcluded: {pkg_license}\n\
         PackageLicenseDeclared: {pkg_license}\n\
         FilesAnalyzed: false\n\n"
    );

    // Parse deps from baked Cargo.toml
    let mut in_deps = false;
    for line in BAKED_CARGO_TOML.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if trimmed.starts_with('[') && in_deps {
            break;
        }
        if !in_deps {
            continue;
        }
        // Parse: name = { version = "X", ... } or name = "X"
        if let Some((name, rest)) = trimmed.split_once('=') {
            let name = name.trim();
            let rest = rest.trim();
            // Skip optional deps (test-only)
            if rest.contains("optional = true") {
                continue;
            }
            let version = if rest.starts_with('"') {
                rest.trim_matches('"')
            } else if rest.contains("version") {
                rest.split("version")
                    .nth(1)
                    .and_then(|s| s.split('"').nth(1))
                    .unwrap_or("?")
            } else {
                "?"
            };
            let spdx_id = format!("SPDXRef-Crate-{name}");
            out.push_str(&format!(
                "PackageName: {name}\n\
                 SPDXID: {spdx_id}\n\
                 PackageVersion: {version}\n\
                 PackageDownloadLocation: https://crates.io/crates/{name}\n\
                 PackageLicenseConcluded: MIT OR Apache-2.0\n\
                 PackageLicenseDeclared: MIT OR Apache-2.0\n\
                 FilesAnalyzed: false\n\
                 \n\
                 Relationship: SPDXRef-Package DEPENDS_ON {spdx_id}\n\n"
            ));
        }
    }
    out
}
