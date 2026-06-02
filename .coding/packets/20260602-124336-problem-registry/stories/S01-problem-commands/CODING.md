# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added markdown-backed cflow problem commands for add, list, show, resolve, cancel, and update.
- Updated README.md and AGENTS.md with durable problem recording guidance.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs
- README.md
- AGENTS.md

## Notes

- Problem JSON is transient stdin and renders directly into .coding/knowledge/PROBLEMS.md.
- Optional state.json problem counters were skipped for this small implementation.

## Next

verify
