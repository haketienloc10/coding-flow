# Request Intake

## Request Summary

Add Decision + Tradeoff workflow to cflow, including markdown decision log, CLI commands, parser/update logic, documentation, and tests.

## Input Type

workflow_improvement

## Lane

normal

## Risk Flags

- CLI behavior change
- markdown parser/update logic
- documentation update

## Hard Gates

- _None_

## Split Required

true

## Reason

The change adds a new workflow namespace and persistent markdown log with lifecycle transitions, so it should use packet/story flow.

## Next Action

packet_brief

## Assumptions

- Decision log should follow the existing problem knowledge-log convention and live under .coding/knowledge unless task-local problem storage is discovered.
- Tradeoffs stay inside decision entries only.

## Clarifying Questions

- _None_
