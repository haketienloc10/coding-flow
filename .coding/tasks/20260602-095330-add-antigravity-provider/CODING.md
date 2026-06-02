# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added built-in Antigravity provider command resolution for plan and coding as agy --prompt with prompt_mode arg.
- Updated doctor command display to include a prompt placeholder for arg-mode providers, yielding agy --prompt "<PROMPT>" for Antigravity.
- Added a focused unit test covering Antigravity plan/coding command shape and resolved doctor display.
- Updated README provider examples/config notes and AGENTS.md provider note.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs
- README.md
- AGENTS.md

## Notes

- Manual fallback used because bin/cflow agent coding --task current failed before implementation with provider command exit code Some(2); verbose plan output showed the current codex command shape rejected the prompt argument.
- JSON output remains transient through the existing agent parse/validate/render flow; no new JSON output files were introduced.

## Next

verify
