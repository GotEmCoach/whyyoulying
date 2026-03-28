# Export Control Classification — whyyoulying

## ITAR (International Traffic in Arms Regulations)

**Not ITAR-controlled.** whyyoulying is a fraud detection analysis tool. It does not contain:
- Defense articles (22 CFR 121, USML)
- Technical data for defense articles
- Military-specific algorithms or hardware interfaces
- Classified information

The tool analyzes publicly-structured data (contract labor categories, billing records) using standard comparison logic. No defense-specific technology.

## EAR (Export Administration Regulations)

### Category 5, Part 2 (Encryption)

**Not controlled under EAR 5A002/5D002.** whyyoulying does not contain, use, or implement any encryption, decryption, or cryptographic functionality.

| Criterion | Status |
|-----------|--------|
| Uses encryption for data confidentiality? | No |
| Uses encryption for authentication? | No |
| Uses digital signatures? | No |
| Uses key exchange/management? | No |
| Contains crypto source code? | No |
| Contains crypto object code? | No |

### EAR99 Classification

whyyoulying is likely **EAR99** (items not elsewhere specified on the CCL). It is:
- Open-source software (Unlicense = public domain equivalent)
- Published and available to the public (GitHub)
- Per 15 CFR 734.7, publicly available software is not subject to EAR

### Open Source Exception

Under EAR 15 CFR 742.15(b), encryption source code that is publicly available is excluded from export controls. However, since whyyoulying contains no encryption at all, this exception is noted but not relied upon.

## Recommendations

- If cryptographic features are added (encrypted exports, signed audit trails), re-evaluate EAR 5D002 applicability.
- If classified data processing is added, ITAR review required.
- Current status: **unrestricted export**.
