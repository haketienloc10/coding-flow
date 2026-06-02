# Request Intake

## Summary

Add first-class Antigravity agent provider using agy --prompt for planning and coding

## Type

new_feature

## Planning Needed

true

## Lane

normal

## Risk Flags

- agent provider execution
- stdout JSON parsing
- documentation updates

## Hard Gates

- cflow agent plan --provider antigravity --task current resolves and runs agy --prompt <prompt>
- cflow agent coding --provider antigravity --task current resolves and runs agy --prompt <prompt>
- cflow agent coding --provider antigravity --task current --fix resolves and runs agy --prompt <prompt>
- stdout is captured, parsed as JSON, validated with existing plan/coding structs, and rendered to PLAN.md or CODING.md without saving JSON output
- full stdout is not printed unless --verbose
- cflow agent providers lists antigravity
- cflow agent doctor --provider antigravity checks agy in PATH and prints resolved command agy --prompt "<PROMPT>"
- README provider examples and AGENTS.md provider note mention antigravity

## Assumptions

- Existing AgentCommand prompt_mode=arg behavior can support Antigravity without new JSON schemas
- Antigravity is a built-in provider with default command config equivalent to the provided TOML

## Clarifying Questions

- _None_

## Next Action

plan
