# Privacy Impact Assessment — whyyoulying

## Data Collection

**whyyoulying does not collect, transmit, or store any data.** It is a local CLI tool that:
1. Reads JSON files from a user-specified directory
2. Runs detection rules in memory
3. Outputs results to stdout or a specified file
4. Exits

No network connections. No telemetry. No analytics. No crash reporting.

## Data Processed

| Data type | Contains PII? | How handled |
|-----------|--------------|-------------|
| Employee IDs | Potentially | Read from input, included in alert output. Not persisted |
| Contract IDs | No | Organizational identifiers |
| CAGE codes | No | Public contractor identifiers |
| Agency names | No | Public government entities |
| Labor rates ($/hr) | Potentially sensitive | Read from input, may appear in alert summaries |
| Hours worked | Potentially sensitive | Read from input, used for detection math |

## Data Storage

- **None.** whyyoulying does not create databases, logs, cache files, or temp files during normal operation.
- Output is to stdout (ephemeral) or a user-specified file path (user-controlled).
- No data retained between runs.

## PII Handling

Employee IDs and labor rates in the input data may constitute PII depending on whether they can be linked to real individuals. whyyoulying:
- Does not enrich or deanonymize data
- Does not correlate across external sources
- Does not transmit data anywhere
- Outputs only what was in the input + detection metadata

## GDPR / CCPA Applicability

- No data collection = no GDPR data controller obligations
- No data sales = no CCPA opt-out requirements
- The user (DoD IG / FBI investigator) is responsible for handling input data per their agency's privacy policies
