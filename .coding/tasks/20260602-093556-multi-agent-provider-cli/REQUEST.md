# Request Intake

## Summary

Upgrade cflow Rust CLI to support multiple agent providers instead of hardcoded Codex.

## Type

new_feature

## Planning Needed

true

## Lane

normal

## Risk Flags

- agent command execution
- provider config parsing
- CLI compatibility

## Hard Gates

- Agent stdout remains transient JSON
- No JSON output stored under .coding/
- Existing Codex UX remains supported

## Assumptions

- Antigravity remains config/custom-only until official CLI syntax is confirmed

## Clarifying Questions

- _None_

## Next Action

plan
