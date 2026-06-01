# Request Intake

## Summary

Fix cflow ship so existing SHIP.md can be reused for dry-run and commit when JSON is no longer stored.

## Type

bug_fix

## Planning Needed

true

## Lane

normal

## Risk Flags

- ship command controls git commit behavior
- must avoid blocking on stdin when no JSON is provided

## Hard Gates

- dry-run requires passed VERIFY.md without findings
- commit parses the commit message from SHIP.md

## Assumptions

- Current task has a single latest VERIFY.md at VERIFY.md
- Findings are represented by Known Issues or Findings sections in VERIFY.md

## Clarifying Questions

- _None_

## Next Action

plan
