# Accessibility — whyyoulying

Section 508 / WCAG 2.1 compliance assessment for CLI application.

## CLI Accessibility

| Criterion | Status | Evidence |
|-----------|--------|----------|
| `--help` completeness | Pass | All flags have descriptions. All subcommands documented. `--help` available at top level and per-subcommand |
| Exit codes | Pass | 0=no alerts, 1=alerts found, 2=error. Documented in README |
| Error messages | Pass | Human-readable, printed to stderr. No cryptic codes |
| Machine-readable output | Pass | JSON (default) and CSV formats. Parseable by downstream tools, screen readers with JSON viewers |
| Color usage | N/A | Release binary produces no colored output. Test binary uses ANSI colors for PASS/FAIL but content is also text-labeled |
| Keyboard navigation | N/A | CLI tool. No interactive mode. All input via command-line arguments |
| Screen reader compatibility | Pass | Plain text output to stdout/stderr. No cursor manipulation. No terminal UI |

## Limitations

- No GUI. No web interface. No WCAG visual compliance applicable.
- CSV output does not include a header row description (only column names).
- JSON output may be large for screen readers if many alerts generated. No pagination.

## Recommendations

- Add `--quiet` flag to suppress stderr progress messages for piped/automated usage.
- Add `--count` flag to output only alert count for quick status checks.
