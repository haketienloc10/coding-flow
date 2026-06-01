# Verify

## Status

passed

## Automated Checks

- Rust format: passed
  - Command: `cargo fmt --check`
  - Notes: Formatter check passed.
- Rust tests: passed
  - Command: `cargo test`
  - Notes: 6 unit tests passed.
- Ship no-input guidance: passed
  - Command: `bin/cflow ship --task current`
  - Notes: Printed guidance without reading JSON when SHIP.md was missing and later when SHIP.md existed.
- Ship dry-run reuse: passed
  - Command: `bin/cflow ship --task current --dry-run`
  - Notes: Reused existing SHIP.md and ran git status --short without committing.

## Manual Checks

- Verified ship without JSON and without flags does not fail with invalid JSON.
- Verified existing SHIP.md guidance lists dry-run and commit commands.
- Verified dry-run reuses existing SHIP.md and does not commit.

## Regressions Checked

- Existing JSON input rendering path still creates SHIP.md.
- Existing verify status parsing used by cflow next still works through the shared section parser.

## Known Issues

- _None_

## Done Criteria Verified

- No-input ship no longer fails with invalid JSON.
- Existing SHIP.md can be reused for dry-run.
- Commit subject parsing from SHIP.md is covered by unit test.
- VERIFY.md status and findings gates are implemented before git actions.
