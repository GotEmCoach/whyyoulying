# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (project-specific — adapt below)

**Records of consequence whyyoulying emits:**

- **`Alert` records** — `rule_id` (E4–E17), `contract_id`, `employee_id`, `cage_code`, `agency`, `predicate_acts`, `estimated_loss`, `timestamp`. Cited by name in federal referrals.
- **DoD IG referral packages** — GAGAS-compliant, via `f18 referral_package`. Consumed by DoD OIG.
- **FBI case-opening packages** — via `f19 fbi_case_opening`. Consumed by FBI field offices.
- **Chain-of-custody hashes** — FNV-1a via `f27`. The integrity proof under FRE 901.

**Threats that apply:**

- **Output forgery → fabricated federal referrals.** Flipping one alert's `contract_id` or `employee_id` weaponizes the tool against innocents (IG SWATting). Flipping the other way suppresses real fraud before an investigator sees it.
- **Detection-threshold subversion.** Rules E4–E17 encode DoDI 5505.02 thresholds (e.g., 15% variance for LABOR_RATE_OVERBILL, default `--min-loss` in the exporter). Silent modification — 15% → 50% — produces "clean" output that passes superficial review but masks real overbilling. Per-rule disablement (e.g., silencing `SUB_BILLED_AS_PRIME`) lets an attacker choose which frauds get reported.
- **Input-dataset tampering.** Detectors are deterministic functions of input JSON/CSV. Flipping an employee's `labor_cat_min`, removing a contract's approved-category list, or editing `hours_billed` silently mutes specific detections. The threat is upstream at file ingestion — there is no persistent storage layer to defend.
- **Chain-of-custody hash subversion.** `f27` FNV-1a is deterministic across platforms and Rust versions; `DefaultHasher` is forbidden here for legal reasons (see CLAUDE.md). Swapping the hasher or removing the hash pipeline leaves outputs with no tamper-evidence.
- **Binary replacement.** Single reproducible binary, 637 KB, 6 direct deps. An attacker swapping `whyyoulying` for a forked version emits same-schema alerts with different semantics. Investigators cannot distinguish without binary-level signature verification against a published fingerprint.
- **Timestamp manipulation.** `timestamp` on alerts is wall-clock at run time. Clock rewind on the host produces legitimate-looking alerts dated to periods where the underlying events didn't yet exist — or post-dates them past the False Claims Act 6-year statute of limitations (31 USC 3731(b)).
- **Referral-package tampering in transit.** `export-referral` and `export-referral --fbi` produce packages for federal consumption. Tampering between disk write and investigator receipt (email, cloud share, USB transfer) breaks chain of custody even if the emitting binary was honest. The FNV-1a hash is the last-mile integrity signal — if the investigator doesn't re-verify on receipt, the protection is notional.
- **Supply chain (deps).** Six direct deps (anyhow, clap, serde, serde_json, tempfile, thiserror) plus two test-only deps (exopack, tokio). Small but nonzero. A backdoored `serde_json` could corrupt alert serialization mid-export without touching detection logic. Zero-dep posture is a mitigation, not an immunity.
- **Physical device seizure.** Host carries fixtures plus real contract data during active investigations. Unencrypted disk exposes investigator targets (CAGE codes, employee IDs, contractor names, estimated losses) before they are served. `kovakey.pub` / `kovakey` at repo root indicates a signing keypair is present — theft of the private half forges referral-package signatures.
- **Insider → whistleblower-doxxing via alert metadata.** `Alert.summary` and the structured fields are designed to cite witnesses. Leaked alerts identify the insider who filed or triggered them. Aggregated alert logs are attractive targets for retaliation.

**N/A for this project:**

- **Persistent-storage compromise** — whyyoulying has no sled, no database, no runtime state. Each run is stateless: files in → alerts out. Storage-rewrite attacks don't apply here the same way they do to projects with runtime DBs. (Public-chain deployment of emitted alert hashes is still in scope; the chain itself becomes the "storage" under attack.)
- **User-account compromise** — CLI binary, no auth layer, no user accounts. Filesystem permissions on `--data-path` are the authorization model.
- **Network MITM during detection** — the binary is air-gap-capable by default; `cargo run -- run` needs no network. Network is only in scope for (a) publishing the daily alert-hash chain, (b) transmitting referrals — both handled under their own threats above.
- **Audit-log tampering** — there is no separate audit log tree; the output stream (alerts + exported referrals + chain-of-custody hashes) is itself the audit record. The "audit log compromise" threat collapses into "output forgery" above.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
