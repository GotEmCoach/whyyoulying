# CMMC Compliance — whyyoulying

CMMC Level 1-2 practices supported by this project.

## Applicability

whyyoulying is a **tool for detecting contractor fraud**, not a contractor system handling CUI. However, the data it processes (contract labor data) may include CUI. The practices below apply to how whyyoulying handles that data.

## CMMC Level 1 Practices

| Domain | Practice | Status | Evidence |
|--------|----------|--------|----------|
| AC (Access Control) | AC.L1-3.1.1 Limit system access | Pass | No authentication layer, but single-user CLI. No network access. No shared state |
| AC | AC.L1-3.1.2 Limit to authorized transactions | Pass | Tool only reads specified directory, writes to stdout or specified file |
| IA (Identification) | IA.L1-3.5.1 Identify system users | N/A | Local CLI tool. OS-level authentication applies |
| MP (Media Protection) | MP.L1-3.8.3 Sanitize media | Pass | No persistent data. Binary deletion = full removal |
| SC (System/Comm) | SC.L1-3.13.1 Monitor communications | N/A | No network communications |
| SI (System Integrity) | SI.L1-3.14.1 Identify flaws | Pass | cargo clippy, 67 unit tests, TRIPLE SIMS 3x pass |

## CMMC Level 2 Practices

| Domain | Practice | Status | Evidence |
|--------|----------|--------|----------|
| AU (Audit) | AU.L2-3.3.1 Create audit records | Pass | Export includes audit_entries with rule_id + input_hash. chain_of_custody in referral package |
| AU | AU.L2-3.3.2 Traceability | Pass | Every alert traces to a rule_id (LABOR_VARIANCE, GHOST_NO_EMPLOYEE, etc.) |
| CM (Config Mgmt) | CM.L2-3.4.1 Establish baselines | Pass | Cargo.lock pins all deps. Release profile defined in Cargo.toml |
| CM | CM.L2-3.4.2 Track changes | Pass | Git version control. All commits signed with contributor attribution |
| RA (Risk Assessment) | RA.L2-3.11.2 Scan for vulnerabilities | Partial | cargo clippy enforced. cargo audit not yet automated |
| SA (Security Assessment) | SA.L2-3.12.1 Assess security controls | Done | This document. SSDF.md. SECURITY.md |
| SC | SC.L2-3.13.8 Use cryptography | N/A | No crypto needed or used. See FIPS.md |

## Gaps

- No RBAC (single-user CLI)
- No automated vulnerability scanning (cargo audit)
- No incident response plan documented
