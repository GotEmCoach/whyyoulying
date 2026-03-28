# FIPS 140-2/3 Status — whyyoulying

## Cryptographic Primitives Used

**None.** whyyoulying does not use cryptographic algorithms for any purpose.

| Primitive | Used? | Context |
|-----------|-------|---------|
| AES-256-GCM | No | — |
| SHA-256/384/512 | No | — |
| RSA | No | — |
| ECDSA | No | — |
| Argon2id | No | — |
| HKDF | No | — |
| HMAC | No | — |

## Hash Usage (Non-Cryptographic)

The only hash in the codebase is `std::collections::hash_map::DefaultHasher` (Rust's SipHash-1-3), used to generate audit trail input hashes in referral packages (`export/mod.rs`). This is for traceability/deduplication, NOT for security purposes. It is not a cryptographic hash and does not require FIPS validation.

## FIPS Compliance Path

Since whyyoulying uses no cryptography:
- FIPS 140-2/3 validation is **not required** for this component
- No CAVP testing needed
- No CMVP module boundary to define

If cryptographic features are added in the future (e.g., encrypted storage, signed exports), a FIPS-validated crypto module (e.g., AWS-LC, BoringSSL via rust bindings) should be used rather than pure-Rust crypto.

## Deployment Note

If deployed on a FIPS-enabled operating system (e.g., RHEL in FIPS mode), whyyoulying will function normally because it does not invoke any crypto APIs.
