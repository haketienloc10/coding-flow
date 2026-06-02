# Request Intake

## Request Summary

Add durable markdown-backed problem recording commands and docs for cflow workflow issues.

## Input Type

workflow_improvement

## Lane

normal

## Risk Flags

- CLI command surface
- markdown parsing and mutation
- workflow documentation

## Hard Gates

- No persistent problem JSON
- Only .coding/state.json may remain persistent JSON
- PROBLEMS.md must be created automatically

## Split Required

true

## Reason

Touches CLI behavior, markdown artifacts, and documentation; keep implementation contained in one story for this packet.

## Next Action

packet_brief

## Assumptions

- State metadata integration is optional and can be skipped for small implementation

## Clarifying Questions

- _None_
