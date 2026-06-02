# Request Intake

## Request Summary

Refactor oversized src/main.rs into a module tree incrementally, starting with a library boundary and thin binary entrypoint.

## Input Type

refactor

## Lane

normal

## Risk Flags

- large monolithic binary
- many command domains share helpers
- stateful workflow artifacts

## Hard Gates

- _None_

## Split Required

true

## Reason

The full extraction is too broad for one implementation slice; module boundaries should be introduced story by story to preserve CLI behavior.

## Next Action

packet_brief

## Assumptions

- No CLI behavior changes are intended
- Existing markdown artifact formats must remain stable
- Each story should be behavior-neutral unless explicitly scoped

## Clarifying Questions

- _None_
