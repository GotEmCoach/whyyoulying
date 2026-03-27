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
    #[arg(long, global = true)]
    config: Option<PathBuf>,
    #[arg(long, global = true)]
    data_path: Option<PathBuf>,
    #[arg(long, global = true, value_parser = clap::value_parser!(f64))]
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
    /// c1=run. Run labor + ghost detection, output alerts (default)
    Run,
    /// c2=ingest. Load and validate data only
    Ingest {
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// c3=export-referral. Export GAGAS referral package or FBI case-opening docs
    ExportReferral {
        #[arg(long)]
        path: Option<PathBuf>,
        #[arg(long, default_value_t = false, help = "FBI case-opening format (AG Guidelines)")]
        fbi: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match &cli.command {
        None | Some(Commands::Run) => run(&cli),
        Some(Commands::Ingest { path }) => cmd_ingest(&cli, path.as_deref()),
        Some(Commands::ExportReferral { path, fbi }) => cmd_export_referral(&cli, path.as_deref(), *fbi),
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
