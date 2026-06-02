# Verify

## Status

passed

## Automated Checks

- no input fallback or phase json filenames: passed
  - Command: `rg -n -- "--input|input file| via stdin or |cat (request|verify|ship|intake|packet|stories|packet_verify|packet_ship)\.json" AGENTS.md skills`
  - Notes: No matches returned.
- skill files exist: passed
  - Command: `for f in request verify ship intake packet-brief packet-split packet-verify packet-ship; do test -f skills/$f.md || exit 1; done`
  - Notes: All skill files referenced by AGENTS.md exist.
- agents routes to skills: passed
  - Command: `rg -n 'Run .* using `skills/.*\.md`|Use stdin heredoc only|Phase commands that are not `agent' AGENTS.md`
  - Notes: AGENTS.md routes phase commands to skills and states stdin heredoc only.

## Manual Checks

- _None_

## Acceptance Criteria Checked

- Agent-facing phase JSON guidance uses stdin heredoc only.
- AGENTS.md does not mention --input for phase JSON.
- AGENTS.md does not use cat phase.json examples.
- AGENTS.md skill references point to existing skill files.

## Findings

- _None_

## Known Issues

- _None_

## Done Criteria Verified

- No --input fallback wording remains in AGENTS.md or skills/.
- No persistent phase JSON filename pattern remains in AGENTS.md or skills/.
- All referenced skill files exist.
