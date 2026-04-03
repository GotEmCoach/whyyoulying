// Unlicense — cochranblock.org
//! Demo mode: baked-in sample contracts with embedded fraud patterns.
//! Sales tool for federal investigators. P13 compressed.

use crate::data::t3;
use crate::detect::{duplicate::t16, ghost::t14, labor::t13, rate_escalation::t23, subcontractor::t22, time::t15};
use crate::types::{t5, t6, t7, t8, t9, t11};

/// Build the demo dataset: 3 contracts, realistic fraud patterns.
pub fn demo_dataset() -> t3 {
    let mut ds = t3::default();

    // --- Contract A: DLOG-2024-0847 — Acme Defense Logistics LLC ---
    // Fraud: ghost employees, weekend/overtime overbilling
    ds.s7.insert("DLOG-2024-0847".into(), t6 {
        s22: "DLOG-2024-0847".into(),
        s23: Some("3KF71".into()),
        s24: Some("DoD".into()),
        s25: [
            ("Logistics Analyst".into(), "BA".into()),
            ("Warehouse Supervisor".into(), "Associate".into()),
            ("Supply Chain Manager".into(), "MS".into()),
        ].into_iter().collect(),
        s26: [
            ("Logistics Analyst".into(), 85.0),
            ("Warehouse Supervisor".into(), 62.0),
            ("Supply Chain Manager".into(), 135.0),
        ].into_iter().collect(),
    });

    // Real employees
    for (id, cat, verified) in [
        ("ACM-101", Some("Logistics Analyst"), true),
        ("ACM-102", Some("Warehouse Supervisor"), true),
        ("ACM-103", Some("Supply Chain Manager"), true),
    ] {
        ds.s8.insert(id.into(), t7 {
            s27: id.into(), s28: vec!["BA".into()],
            s29: cat.map(String::from), s30: verified, ..Default::default()
        });
    }

    // Labor charges (actual work)
    for (eid, cat, hrs, rate) in [
        ("ACM-101", "Logistics Analyst", 160.0, 85.0),
        ("ACM-102", "Warehouse Supervisor", 168.0, 62.0),
        ("ACM-103", "Supply Chain Manager", 140.0, 135.0),
    ] {
        ds.s9.push(t8 { s31: "DLOG-2024-0847".into(), s32: eid.into(), s33: cat.into(), s34: hrs, s35: Some(rate), s71: None });
    }

    // Billing records — includes ghost employee ACM-201 + inflated hours
    for (eid, hrs, cat) in [
        ("ACM-101", 160.0, "Logistics Analyst"),
        ("ACM-102", 212.0, "Warehouse Supervisor"),    // overbilled 44 hrs
        ("ACM-103", 140.0, "Supply Chain Manager"),
        ("ACM-201", 176.0, "Logistics Analyst"),        // GHOST — not in roster
        ("ACM-202", 88.0, "Warehouse Supervisor"),      // GHOST — not in roster
    ] {
        ds.s10.push(t9 { s36: "DLOG-2024-0847".into(), s37: eid.into(), s38: hrs, s39: cat.into(), s40: Some("2025-11".into()) });
    }

    // --- Contract B: ITSVC-2025-1293 — Pinnacle Systems Group Inc ---
    // Fraud: rate overbill, duplicate billing across contracts
    ds.s7.insert("ITSVC-2025-1293".into(), t6 {
        s22: "ITSVC-2025-1293".into(),
        s23: Some("7RP42".into()),
        s24: Some("DoD".into()),
        s25: [
            ("Senior Developer".into(), "BS".into()),
            ("Systems Architect".into(), "MS".into()),
            ("Help Desk".into(), "Associate".into()),
        ].into_iter().collect(),
        s26: [
            ("Senior Developer".into(), 175.0),
            ("Systems Architect".into(), 210.0),
            ("Help Desk".into(), 55.0),
        ].into_iter().collect(),
    });
    // Second contract for dup billing
    ds.s7.insert("ITSVC-2025-1294".into(), t6 {
        s22: "ITSVC-2025-1294".into(),
        s23: Some("7RP42".into()),
        s24: Some("DoD".into()),
        s25: [("Senior Developer".into(), "BS".into())].into_iter().collect(),
        s26: [("Senior Developer".into(), 175.0)].into_iter().collect(),
    });

    for (id, cat, verified) in [
        ("PIN-301", Some("Senior Developer"), true),
        ("PIN-302", Some("Systems Architect"), true),
        ("PIN-303", Some("Help Desk"), true),
    ] {
        ds.s8.insert(id.into(), t7 {
            s27: id.into(), s28: vec!["BS".into()],
            s29: cat.map(String::from), s30: verified, ..Default::default()
        });
    }

    // Labor charges — actual work
    for (cid, eid, cat, hrs, rate) in [
        ("ITSVC-2025-1293", "PIN-301", "Senior Developer", 160.0, 175.0),
        ("ITSVC-2025-1293", "PIN-302", "Systems Architect", 152.0, 210.0),
        ("ITSVC-2025-1293", "PIN-303", "Help Desk", 176.0, 55.0),
    ] {
        ds.s9.push(t8 { s31: cid.into(), s32: eid.into(), s33: cat.into(), s34: hrs, s35: Some(rate), s71: None });
    }

    // Billing — rate overbill on PIN-301 ($225 vs contract $175), duplicate on second contract
    for (cid, eid, hrs, cat) in [
        ("ITSVC-2025-1293", "PIN-301", 160.0, "Senior Developer"),
        ("ITSVC-2025-1293", "PIN-302", 152.0, "Systems Architect"),
        ("ITSVC-2025-1293", "PIN-303", 176.0, "Help Desk"),
        ("ITSVC-2025-1294", "PIN-301", 80.0, "Senior Developer"),   // DUP BILLING — same employee, second contract
    ] {
        ds.s10.push(t9 { s36: cid.into(), s37: eid.into(), s38: hrs, s39: cat.into(), s40: Some("2025-11".into()) });
    }
    // Overbilled rate on labor charge
    ds.s9.push(t8 { s31: "ITSVC-2025-1293".into(), s32: "PIN-301".into(), s33: "Senior Developer".into(), s34: 160.0, s35: Some(225.0), s71: None });

    // --- Contract C: CNST-2024-0516 — Ironclad Construction Partners ---
    // Fraud: qual below (Junior billed as Lead), unverified employee
    ds.s7.insert("CNST-2024-0516".into(), t6 {
        s22: "CNST-2024-0516".into(),
        s23: Some("9BL58".into()),
        s24: Some("DoD".into()),
        s25: [
            ("Project Lead".into(), "PE".into()),
            ("Site Inspector".into(), "BA".into()),
            ("Safety Officer".into(), "OSHA-30".into()),
        ].into_iter().collect(),
        s26: [
            ("Project Lead".into(), 165.0),
            ("Site Inspector".into(), 95.0),
            ("Safety Officer".into(), 110.0),
        ].into_iter().collect(),
    });

    ds.s8.insert("ICP-401".into(), t7 {
        s27: "ICP-401".into(), s28: vec!["PE".into()],
        s29: Some("Lead".into()), s30: true, ..Default::default()
    });
    ds.s8.insert("ICP-402".into(), t7 {
        s27: "ICP-402".into(), s28: vec!["Associate".into()],
        s29: Some("Junior".into()), s30: false, ..Default::default() // UNVERIFIED
    });
    ds.s8.insert("ICP-403".into(), t7 {
        s27: "ICP-403".into(), s28: vec!["OSHA-30".into()],
        s29: Some("Mid".into()), s30: true, ..Default::default()
    });

    for (eid, cat, hrs, rate) in [
        ("ICP-401", "Project Lead", 160.0, 165.0),
        ("ICP-402", "Site Inspector", 152.0, 95.0),
        ("ICP-403", "Safety Officer", 144.0, 110.0),
    ] {
        ds.s9.push(t8 { s31: "CNST-2024-0516".into(), s32: eid.into(), s33: cat.into(), s34: hrs, s35: Some(rate), s71: None });
    }

    // ICP-402 billed as Project Lead (qual below — Junior billing as Lead)
    for (eid, hrs, cat) in [
        ("ICP-401", 160.0, "Project Lead"),
        ("ICP-402", 176.0, "Project Lead"),     // QUAL BELOW — Junior as Lead
        ("ICP-403", 144.0, "Safety Officer"),
    ] {
        ds.s10.push(t9 { s36: "CNST-2024-0516".into(), s37: eid.into(), s38: hrs, s39: cat.into(), s40: Some("2025-11".into()) });
    }

    ds
}

/// Run all detectors on demo data, return alerts.
pub fn run_demo() -> (t3, Vec<t5>) {
    let ds = demo_dataset();
    let labor = t13::f10(15.0);
    let ghost = t14::f12();
    let time = t15::f14(176.0);
    let dup = t16::f16();
    let sub = t22::f23();
    let rate_esc = t23::f25(10.0);
    let alerts: Vec<t5> = labor.f11(&ds).into_iter()
        .chain(ghost.f13(&ds))
        .chain(time.f15(&ds))
        .chain(dup.f17(&ds))
        .chain(sub.f24(&ds))
        .chain(rate_esc.f26(&ds))
        .collect();
    (ds, alerts)
}

/// Severity label from numeric score.
fn severity_label(sev: u8) -> &'static str {
    match sev {
        0..=3 => "LOW",
        4..=5 => "MEDIUM",
        6..=7 => "HIGH",
        _ => "CRITICAL",
    }
}

/// Statute citation for predicate acts.
fn statute(act: &str) -> String {
    match act {
        "E12" | "FalseClaims" => "False Claims Act, 31 USC 3729".into(),
        "E13" | "WireFraud" => "Wire Fraud, 18 USC 1343".into(),
        "E14" | "IdentityFraud" => "Identity Fraud, 18 USC 1028".into(),
        other => other.into(),
    }
}

/// Format number with comma grouping.
fn commas(n: f64) -> String {
    let s = format!("{:.0}", n);
    let bytes = s.as_bytes();
    let mut out = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 && b != b'-' { out.push(','); }
        out.push(b as char);
    }
    out
}

/// Estimate fraud dollar amount from an alert.
fn estimate_fraud(a: &t5, ds: &t3) -> f64 {
    match a.s12 {
        t11::E6 => { // RATE_OVERBILL — (charged-contract)*hrs
            if let (Some(cid), Some(eid)) = (&a.s16, &a.s17) {
                if let Some(c) = ds.f6(cid) {
                    let charges: Vec<&t8> = ds.s9.iter()
                        .filter(|lc| &lc.s31 == cid && &lc.s32 == eid)
                        .collect();
                    let mut total = 0.0;
                    for lc in charges {
                        if let (Some(rate), Some(&crate_)) = (lc.s35, c.s26.get(&lc.s33)) {
                            if rate > crate_ { total += (rate - crate_) * lc.s34; }
                        }
                    }
                    return total;
                }
            }
            0.0
        }
        t11::E7 => { // GHOST_NO_EMPLOYEE — full billed amount
            if let Some(eid) = &a.s17 {
                ds.s10.iter()
                    .filter(|br| &br.s37 == eid)
                    .map(|br| {
                        let cid = &br.s36;
                        let rate = ds.f6(cid)
                            .and_then(|c| c.s26.get(&br.s39).copied())
                            .unwrap_or(100.0);
                        br.s38 * rate
                    })
                    .sum()
            } else { 0.0 }
        }
        t11::E9 => { // BILLED_NOT_PERFORMED — excess hours * rate
            if let (Some(cid), Some(eid)) = (&a.s16, &a.s17) {
                let billed: f64 = ds.s10.iter()
                    .filter(|br| &br.s36 == cid && &br.s37 == eid)
                    .map(|br| br.s38).sum();
                let performed: f64 = ds.s9.iter()
                    .filter(|lc| &lc.s31 == cid && &lc.s32 == eid)
                    .map(|lc| lc.s34).sum();
                let excess = (billed - performed).max(0.0);
                let rate = ds.f6(cid)
                    .and_then(|c| {
                        ds.s10.iter().find(|br| &br.s36 == cid && &br.s37 == eid)
                            .and_then(|br| c.s26.get(&br.s39).copied())
                    })
                    .unwrap_or(100.0);
                return excess * rate;
            }
            0.0
        }
        _ => 0.0,
    }
}

/// Format terminal report (plain text).
pub fn format_text(ds: &t3, alerts: &[t5]) -> String {
    let mut out = String::new();
    let contracts: Vec<&str> = ds.s7.keys().map(|s| s.as_str()).collect();
    let total_fraud: f64 = alerts.iter().map(|a| estimate_fraud(a, ds)).sum();

    // Executive Summary
    out.push_str("==============================================================================\n");
    out.push_str("  WHYYOULYING — Fraud Detection Report (DEMO)\n");
    out.push_str("==============================================================================\n\n");
    out.push_str(&format!("  Contracts analyzed:    {}\n", contracts.len()));
    out.push_str(&format!("  Total alerts:          {}\n", alerts.len()));
    out.push_str(&format!("  Estimated fraud:       ${}\n", commas(total_fraud)));
    out.push_str(&format!("  Critical alerts:       {}\n", alerts.iter().filter(|a| a.s13 >= 8).count()));
    out.push_str(&format!("  High alerts:           {}\n", alerts.iter().filter(|a| a.s13 >= 6 && a.s13 < 8).count()));
    out.push_str(&format!("  Generated:             {}\n\n", crate::util::f20()));

    // Per-contract breakdown
    for cid in &contracts {
        let contract = ds.f6(cid).unwrap();
        let contract_alerts: Vec<&t5> = alerts.iter()
            .filter(|a| a.s16.as_deref() == Some(cid) || a.s17.as_ref().is_some_and(|eid| {
                ds.s10.iter().any(|br| &br.s36 == cid && br.s37 == *eid)
            }))
            .collect();
        if contract_alerts.is_empty() { continue; }

        let fraud_amt: f64 = contract_alerts.iter().map(|a| estimate_fraud(a, ds)).sum();
        out.push_str("------------------------------------------------------------------------------\n");
        out.push_str(&format!("  Contract: {}  |  CAGE: {}  |  Agency: {}\n",
            cid,
            contract.s23.as_deref().unwrap_or("—"),
            contract.s24.as_deref().unwrap_or("—"),
        ));
        out.push_str(&format!("  Alerts: {}  |  Estimated: ${}\n", contract_alerts.len(), commas(fraud_amt)));
        out.push_str("------------------------------------------------------------------------------\n\n");

        for a in &contract_alerts {
            let est = estimate_fraud(a, ds);
            out.push_str(&format!("  [{:>8}]  {}  (confidence {}%)\n",
                severity_label(a.s13), a.s12, a.s14));
            out.push_str(&format!("              {}\n", a.s15));
            if est > 0.0 {
                out.push_str(&format!("              Estimated loss: ${}\n", commas(est)));
            }
            if let Some(acts) = &a.s20 {
                let statutes: Vec<String> = acts.iter().map(|p| statute(&format!("{:?}", p))).collect();
                out.push_str(&format!("              Predicates: {}\n", statutes.join("; ")));
            }
            out.push('\n');
        }
    }

    // Recommended actions
    out.push_str("==============================================================================\n");
    out.push_str("  RECOMMENDED INVESTIGATION ACTIONS\n");
    out.push_str("==============================================================================\n\n");
    if alerts.iter().any(|a| a.s12 == t11::E7) {
        out.push_str("  1. GHOST EMPLOYEES: Request employee roster verification from contractor.\n");
        out.push_str("     Cross-reference with DCAA floorcheck records (DCAM 13500).\n\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E6) {
        out.push_str("  2. RATE OVERBILLING: Compare invoiced rates against contract rate schedule.\n");
        out.push_str("     Request contractor rate justification per FAR 15.404-1.\n\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E5) {
        out.push_str("  3. LABOR SUBSTITUTION: Verify employee qualifications against contract requirements.\n");
        out.push_str("     Request resumes and certifications for flagged employees.\n\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E11) {
        out.push_str("  4. DUPLICATE BILLING: Cross-reference invoices across contracts for flagged employees.\n");
        out.push_str("     Verify time-and-attendance records against billing submissions.\n\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E10) {
        out.push_str("  5. TIME OVERCHARGE: Verify total hours are physically plausible.\n");
        out.push_str("     Request badge-in/badge-out records and building access logs.\n\n");
    }
    out.push_str("  Refer to DoDI 5505.02 for criminal investigation thresholds.\n");
    out.push_str("  Refer to AG Guidelines for FBI preliminary inquiry standards.\n\n");
    out
}

/// Format HTML report (self-contained, printable).
pub fn format_html(ds: &t3, alerts: &[t5]) -> String {
    let contracts: Vec<&str> = ds.s7.keys().map(|s| s.as_str()).collect();
    let total_fraud: f64 = alerts.iter().map(|a| estimate_fraud(a, ds)).sum();
    let ts = crate::util::f20();

    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>whyyoulying — Fraud Detection Report</title>
<style>
  body { font-family: 'Segoe UI', Arial, sans-serif; max-width: 900px; margin: 40px auto; color: #1a1a1a; line-height: 1.5; padding: 0 20px; }
  h1 { font-size: 22px; border-bottom: 3px solid #003366; padding-bottom: 8px; color: #003366; }
  h2 { font-size: 17px; color: #003366; margin-top: 32px; border-bottom: 1px solid #ccc; padding-bottom: 4px; }
  h3 { font-size: 14px; color: #444; margin-top: 20px; }
  .summary-box { background: #f0f4f8; border: 1px solid #b0c4de; border-radius: 4px; padding: 16px 20px; margin: 16px 0; }
  .summary-box td { padding: 4px 16px 4px 0; font-size: 14px; }
  .summary-box .val { font-weight: bold; font-size: 16px; }
  table.alerts { width: 100%; border-collapse: collapse; font-size: 13px; margin: 12px 0; }
  table.alerts th { background: #003366; color: #fff; text-align: left; padding: 6px 10px; }
  table.alerts td { padding: 6px 10px; border-bottom: 1px solid #ddd; vertical-align: top; }
  table.alerts tr:nth-child(even) { background: #f9f9f9; }
  .sev-CRITICAL { color: #b71c1c; font-weight: bold; }
  .sev-HIGH { color: #e65100; font-weight: bold; }
  .sev-MEDIUM { color: #f57f17; }
  .sev-LOW { color: #558b2f; }
  .actions { background: #fff8e1; border: 1px solid #ffe082; border-radius: 4px; padding: 16px 20px; margin: 16px 0; }
  .actions li { margin: 6px 0; font-size: 13px; }
  .footer { font-size: 11px; color: #888; margin-top: 40px; border-top: 1px solid #ccc; padding-top: 8px; }
  .disclaimer { font-size: 11px; color: #666; font-style: italic; margin: 12px 0; }
  @media print { body { max-width: 100%; margin: 20px; } .summary-box, .actions { break-inside: avoid; } }
</style>
</head>
<body>
"#);

    html.push_str("<h1>Fraud Detection Report — DEMONSTRATION</h1>\n");
    html.push_str("<p class='disclaimer'>This report was generated from synthetic sample data for demonstration purposes. No real contract, employee, or billing data was used.</p>\n");

    // Executive Summary
    html.push_str("<h2>Executive Summary</h2>\n<div class='summary-box'><table>\n");
    html.push_str(&format!("<tr><td>Contracts analyzed</td><td class='val'>{}</td></tr>\n", contracts.len()));
    html.push_str(&format!("<tr><td>Total alerts</td><td class='val'>{}</td></tr>\n", alerts.len()));
    html.push_str(&format!("<tr><td>Estimated fraud exposure</td><td class='val'>${}</td></tr>\n", commas(total_fraud)));
    html.push_str(&format!("<tr><td>Critical / High alerts</td><td class='val'>{} / {}</td></tr>\n",
        alerts.iter().filter(|a| a.s13 >= 8).count(),
        alerts.iter().filter(|a| a.s13 >= 6 && a.s13 < 8).count()));
    html.push_str(&format!("<tr><td>Report generated</td><td>{}</td></tr>\n", ts));
    html.push_str("</table></div>\n");

    // Per-contract
    for cid in &contracts {
        let contract = ds.f6(cid).unwrap();
        let ca: Vec<&t5> = alerts.iter()
            .filter(|a| a.s16.as_deref() == Some(cid))
            .collect();
        if ca.is_empty() { continue; }

        let fraud_amt: f64 = ca.iter().map(|a| estimate_fraud(a, ds)).sum();
        html.push_str(&format!("<h2>Contract: {} <span style='font-weight:normal;font-size:13px;color:#666'>| CAGE: {} | Agency: {}</span></h2>\n",
            cid,
            contract.s23.as_deref().unwrap_or("—"),
            contract.s24.as_deref().unwrap_or("—"),
        ));
        html.push_str(&format!("<p><strong>{}</strong> alerts | Estimated exposure: <strong>${}</strong></p>\n", ca.len(), commas(fraud_amt)));
        html.push_str("<table class='alerts'><tr><th>Severity</th><th>Rule</th><th>Description</th><th>Employee</th><th>Est. Loss</th><th>Legal Predicates</th></tr>\n");
        for a in &ca {
            let sev = severity_label(a.s13);
            let est = estimate_fraud(a, ds);
            let predicates = a.s20.as_ref().map(|acts| {
                acts.iter().map(|p| statute(&format!("{:?}", p))).collect::<Vec<_>>().join("<br>")
            }).unwrap_or_default();
            html.push_str(&format!(
                "<tr><td class='sev-{}'>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td style='font-size:11px'>{}</td></tr>\n",
                sev, sev, a.s12, a.s15,
                a.s17.as_deref().unwrap_or("—"),
                if est > 0.0 { format!("${}", commas(est)) } else { "—".into() },
                predicates,
            ));
        }
        html.push_str("</table>\n");
    }

    // Cross-entity alerts (no contract_id)
    let cross: Vec<&t5> = alerts.iter().filter(|a| a.s16.is_none()).collect();
    if !cross.is_empty() {
        html.push_str("<h2>Cross-Contract Alerts</h2>\n");
        html.push_str("<table class='alerts'><tr><th>Severity</th><th>Rule</th><th>Description</th><th>Employee</th><th>Legal Predicates</th></tr>\n");
        for a in &cross {
            let sev = severity_label(a.s13);
            let predicates = a.s20.as_ref().map(|acts| {
                acts.iter().map(|p| statute(&format!("{:?}", p))).collect::<Vec<_>>().join("<br>")
            }).unwrap_or_default();
            html.push_str(&format!(
                "<tr><td class='sev-{}'>{}</td><td>{}</td><td>{}</td><td>{}</td><td style='font-size:11px'>{}</td></tr>\n",
                sev, sev, a.s12, a.s15,
                a.s17.as_deref().unwrap_or("—"),
                predicates,
            ));
        }
        html.push_str("</table>\n");
    }

    // Recommended actions
    html.push_str("<h2>Recommended Investigation Actions</h2>\n<div class='actions'><ol>\n");
    if alerts.iter().any(|a| a.s12 == t11::E7) {
        html.push_str("<li><strong>Ghost Employees:</strong> Request employee roster verification. Cross-reference with DCAA floorcheck records (DCAM 13500).</li>\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E6) {
        html.push_str("<li><strong>Rate Overbilling:</strong> Compare invoiced rates against contract rate schedule. Request rate justification per FAR 15.404-1.</li>\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E5) {
        html.push_str("<li><strong>Labor Substitution:</strong> Verify employee qualifications against contract requirements. Request resumes and certifications.</li>\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E11) {
        html.push_str("<li><strong>Duplicate Billing:</strong> Cross-reference invoices across contracts. Verify time-and-attendance records.</li>\n");
    }
    if alerts.iter().any(|a| a.s12 == t11::E10) {
        html.push_str("<li><strong>Time Overcharge:</strong> Verify total hours are physically plausible. Request badge-in/badge-out and access logs.</li>\n");
    }
    html.push_str("</ol>\n");
    html.push_str("<p style='font-size:12px'>Refer to DoDI 5505.02 for criminal investigation thresholds. Refer to AG Guidelines for FBI preliminary inquiry standards.</p>\n");
    html.push_str("</div>\n");

    html.push_str(&format!("<p class='footer'>Generated by whyyoulying v{} | {} | Synthetic demo data — not for investigative use</p>\n",
        env!("CARGO_PKG_VERSION"), ts));
    html.push_str("</body></html>\n");
    html
}
