# Federal Use Cases — whyyoulying

## Primary: DoD Inspector General / DCIS

**Use case:** Proactive detection of labor category fraud and ghost billing on DoD contracts.

- **How:** Investigators export contract, labor, and billing data from DCAA/SAP systems as JSON. Run whyyoulying to identify anomalies before annual floorchecks. Prioritize investigations using severity/confidence scores.
- **Legal basis:** DoDI 5505.02 (Criminal Investigations of Fraud Offenses), DoDI 5505.03 (Initiation of Investigations by DCIOs)
- **Current fit:** 8 detection rules directly mapped to DoD OIG fraud scenarios. DoD nexus filter (--agency, --cage-code) built in.
- **Gap:** No direct integration with DCAA data systems. JSON export step is manual.

## Primary: FBI (DoJ)

**Use case:** Preliminary inquiry predicate generation for government contract fraud.

- **How:** FBI fraud investigators use whyyoulying to analyze contractor billing data. FBI case-opening export generates factual basis documentation per AG Guidelines. Predicate acts (False Claims, Wire Fraud, Identity Fraud) automatically tagged.
- **Legal basis:** Attorney General's Guidelines, DIOG
- **Current fit:** `export-referral --fbi` generates AG Guidelines-compliant output. Predicate routing maps fraud types to specific statutes.
- **Gap:** No integration with FBI case management systems.

## Secondary: DHS / CBP

**Use case:** Contractor oversight for border infrastructure contracts.

- **How:** DHS contracts for border technology, surveillance systems, and facility construction involve large labor-category workforces. whyyoulying detects overbilling patterns.
- **Current fit:** CAGE code filtering works for contractor identification. Detection rules apply generically to any labor-category contract.
- **Gap:** Would need DHS-specific data connectors (e.g., FPDS feeds).

## Secondary: VA (Veterans Affairs)

**Use case:** Fraud detection on IT and healthcare staffing contracts.

- **How:** VA relies heavily on contractor staff for IT modernization and healthcare support. Labor substitution (billing a Senior engineer but providing a Junior) is a known pattern.
- **Current fit:** LABOR_QUAL_BELOW and LABOR_RATE_OVERBILL directly address this. Ghost employee detection catches billing for non-existent staff.
- **Gap:** VA uses VistA and custom procurement systems, not standard JSON feeds.

## Tertiary: GSA

**Use case:** Schedule contract compliance monitoring.

- **How:** GSA Schedule holders must bill at or below their scheduled rates. whyyoulying's LABOR_RATE_OVERBILL rule detects rate overages.
- **Current fit:** Rate comparison logic is directly applicable. Threshold is configurable per schedule.
- **Gap:** No GSA Advantage / SAM.gov integration for automated rate lookup.

## Not Applicable

| Agency | Why |
|--------|-----|
| NASA | Scientific computing contracts don't typically use labor categories in the same way. Different fraud patterns |
| DOE | National lab contractors use cost-plus structures. Different billing model |
| NSF | Grant-based funding, not labor-category contracts |
