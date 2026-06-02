# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added src/workflow_vocab.rs as a small extracted module for workflow vocabulary and knowledge path constants.
- Replaced the removed top-level constants in src/main.rs with an explicit import list from workflow_vocab.
- Kept validation, rendering, command, and state behavior unchanged.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs
- src/workflow_vocab.rs

## Notes

- This is intentionally a foundation slice, not full src/main.rs modularization.

## Next

verify
