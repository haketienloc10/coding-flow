# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added -- separator to built-in Codex plan args before the prompt argument.
- Added -- separator to built-in Codex coding args before the prompt argument.
- Added focused test coverage for Codex plan and coding separator placement.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs

## Notes

- Manual fallback used because bin/cflow agent coding --task current failed with the exact Codex prompt parsing bug being fixed.

## Next

verify
