# Request Intake

## Summary

Fix built-in Codex provider prompt parsing by adding -- before arg-mode prompt

## Type

bug_fix

## Planning Needed

true

## Lane

normal

## Risk Flags

- agent provider command resolution
- CLI argument parsing

## Hard Gates

- Built-in Codex plan command resolves with -- before the prompt argument
- Built-in Codex coding command resolves with -- before the prompt argument
- Fix preserves existing stdout capture, JSON parsing, validation, and rendering behavior
- Focused tests verify the separator is present

## Assumptions

- codex exec treats -- as the end-of-options marker before PROMPT
- Keeping prompt_mode = arg is preferred over switching Codex to stdin

## Clarifying Questions

- _None_

## Next Action

plan
