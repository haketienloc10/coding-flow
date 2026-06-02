# Code Plan

## 1. Objective

Allow cflow agent plan/coding to resolve and run multiple agent providers instead of a hardcoded Codex command while preserving transient JSON validation and markdown rendering.

## 2. Scope

### In Scope

- Provider selection via --provider, CFLOW_AGENT_PROVIDER, .coding/agent.toml default_provider, then codex fallback
- Built-in defaults for codex, claude, gemini, config/custom handling for antigravity and custom
- prompt_mode arg/stdin, separate stdout/stderr capture, verbose diagnostics, provider listing and doctor commands
- Docs and focused Rust tests

### Out of Scope

- Official Antigravity CLI syntax
- Persisting provider JSON output
- Changing ship/commit behavior

## 3. Requirements

- Agent stdout remains transient JSON parsed and validated against existing structs/schema validators before rendering markdown.
- No JSON output is written to .coding/.
- Plan providers receive a no-edit prompt expectation; coding providers retain artifact editing restrictions.
- Non-verbose failures should avoid dumping full stdout/stderr.

## 4. Technical Approach

- Add provider config structs and a small TOML parser dependency to load .coding/agent.toml without ad hoc parsing.
- Resolve provider name in the required precedence and resolve phase command from built-in defaults, CFLOW_AGENT_CMD/custom config, or provider config overrides.
- Replace run_agent(prompt) with run_agent(provider, phase, prompt, verbose) that appends prompt as final arg unless prompt_mode=stdin.
- Improve JSON extraction to support plain JSON, fenced JSON, and simple wrapper JSON message/content/text extraction where possible.
- Add cflow agent providers and cflow agent doctor --provider <name>, update usage, README, and AGENTS.md.

## 5. Files to Change

- src/main.rs
- Cargo.toml
- Cargo.lock
- README.md
- AGENTS.md

## 6. Implementation Steps

- [todo] Add provider/phase/prompt-mode types and .coding/agent.toml loading.
- [todo] Implement provider resolution and command resolution with built-in defaults and custom/config handling.
- [todo] Update agent plan/coding commands to accept --provider and --verbose and use the new runner.
- [todo] Add providers/doctor subcommands and usage/docs.
- [todo] Add focused tests and run cargo test plus CLI smoke checks.

## 7. Test Plan

### Planned

- cargo test
- cargo fmt --check
- bin/cflow agent providers
- bin/cflow agent doctor --provider codex

### Result

- _None_

## 8. Risks

- Shell-like CFLOW_AGENT_CMD parsing can be limited if implemented as whitespace split; document or improve cautiously.
- Provider wrapper JSON formats vary, so extraction should be permissive but still fail cleanly if no valid task JSON exists.

## 9. Done Criteria

### Criteria

- Existing cflow agent plan/coding still work with Codex defaults.
- Provider can be selected by CLI/env/config in the specified order.
- Custom/config commands can use prompt as arg or stdin.
- Agent provider introspection commands work and docs mention examples.

### Verified

- _None_
