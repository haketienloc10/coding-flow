# Verify

## Status

passed

## Automated Checks

- cargo fmt --check
- cargo test
- cargo build --release
- bin/cflow agent providers
- bin/cflow agent doctor --provider codex
- bin/cflow agent doctor --provider custom
- CFLOW_AGENT_CMD="printf '{}'" bin/cflow agent doctor --provider custom

## Manual Checks

- Confirmed .coding contains no transient agent JSON output files beyond existing state.json.
- Confirmed antigravity reports unconfigured without hardcoded syntax.

## Acceptance Criteria Checked

- Provider selection supports --provider, CFLOW_AGENT_PROVIDER, .coding/agent.toml default_provider, and codex fallback.
- Built-in defaults exist for codex, claude, and gemini plan/coding commands.
- Custom provider resolves CFLOW_AGENT_CMD or .coding/agent.toml phase config.
- prompt_mode arg/stdin is supported and defaults to arg.
- Agent stdout/stderr are captured separately and full output is gated behind --verbose.
- Agent output is parsed as JSON, wrapper text extraction is supported, and markdown rendering remains validator-gated.

## Findings

- _None_

## Known Issues

- _None_

## Done Criteria Verified

- Existing cflow agent plan/coding still have Codex defaults.
- Provider can be selected by CLI/env/config in the specified order.
- Custom/config commands can use prompt as arg or stdin.
- Agent provider introspection commands work and docs mention examples.
