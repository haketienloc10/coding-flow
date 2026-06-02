# Coding

## Mode

initial

## Status

ready_for_verify

## Summary

- Added provider resolution for codex, claude, gemini, antigravity, and custom with CLI/env/config fallback order.
- Replaced hardcoded agent runner with resolved command execution supporting prompt_mode arg/stdin and separate stdout/stderr capture.
- Added agent providers and doctor subcommands plus README/AGENTS provider documentation.
- Added tests for provider resolution, config parsing, wrapper JSON extraction, final JSON extraction, and Codex defaults.

## Fixed Findings

- _None_

## Changed Files

- src/main.rs
- Cargo.toml
- Cargo.lock
- README.md
- AGENTS.md

## Notes

- Antigravity remains unconfigured unless supplied through .coding/agent.toml or custom command.
- bin/cflow uses target/release/cflow when present, so release build was refreshed for local smoke checks.

## Next

verify
