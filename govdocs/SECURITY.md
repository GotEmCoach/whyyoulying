# Security Posture — whyyoulying

## Cryptographic Usage

**This project does not use cryptography for data protection.** It is a detection/analysis tool that reads JSON files and outputs alerts.

The only hash usage is `std::collections::hash_map::DefaultHasher` (SipHash) for generating non-cryptographic audit trail hashes in referral packages. These are for traceability, not security.

## No Plaintext Secrets

- No API keys, passwords, or tokens in source code.
- No `.env` files. No environment variable secrets.
- `kovakey` and `kovakey.pub` in repo root are SSH keys for the kova C2 deployment. They are not used by the whyyoulying binary.

## Input Validation

| Boundary | Validation |
|----------|-----------|
| `--threshold` | clap: f64, checked `(0, 100]` in config.rs |
| `--min-confidence` | clap: u8, range `0..=100` |
| `--agency`, `--cage-code` | String, case-insensitive comparison |
| `--data-path` | Path, files parsed as JSON with serde (type-safe deserialization) |
| `--config` | JSON file, validated threshold range after parse |

## Attack Surfaces

| Surface | Risk | Mitigation |
|---------|------|-----------|
| Malformed JSON input | Denial of service via large files | serde deserialization fails fast. No streaming parser. Memory bounded by file size |
| Path traversal in `--data-path` | Read arbitrary files | Only reads 4 specific filenames (contracts.json, etc.) from the given directory. No recursive traversal |
| CSV injection in `--output csv` | Formula injection if output opened in Excel | Quotes fields containing commas/quotes. Does not prefix with `=`, `+`, `-`, `@` |
| Supply chain (deps) | Compromised crate | All deps are widely-used, MIT/Apache-2.0 licensed. Cargo.lock pins exact versions |

## Error Handling

- All errors return exit code 2 with stderr message.
- No stack traces in release mode (`panic = 'abort'`).
- No sensitive data in error messages (no file contents, no credentials).
