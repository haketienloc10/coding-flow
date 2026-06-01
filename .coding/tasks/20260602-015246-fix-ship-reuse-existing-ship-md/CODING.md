# Coding

## Status

ready_for_verify

## Summary

- Refactored ship to read optional JSON and reuse existing SHIP.md when appropriate.
- Added VERIFY.md gates for missing status, non-passed status, and findings.
- Added SHIP.md commit message parsing for commit mode.

## Changed Files

- src/main.rs

## Notes

- Commit mode intentionally commits with a single -m subject parsed from SHIP.md per requirement.

## Next

verify
