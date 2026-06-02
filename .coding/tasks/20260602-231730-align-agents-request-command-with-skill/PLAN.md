# Code Plan

## 1. Objective

Make agent-facing phase JSON guidance consistent: stdin heredoc only, no --input fallback, and no persistent phase JSON filenames.

## 2. Scope

### In Scope

- Update AGENTS.md to route phase commands to skills/*.md instead of cat *.json examples.
- Remove --input fallback wording from agent-facing skill guidance.
- Add missing packet phase skill files referenced by AGENTS.md.
- Clarify verify and ship skills for both Tiny Flow and Story Flow commands.

### Out of Scope

- Changing CLI runtime support for --input.
- Rewriting README general CLI reference.
- Changing problem/decision durable workflow guidance.

## 3. Requirements

- AGENTS.md and skills must not mention --input for phase JSON guidance.
- AGENTS.md must not use cat request.json, verify.json, ship.json, intake.json, packet.json, stories.json, packet_verify.json, or packet_ship.json phase examples.
- Every skill file referenced by AGENTS.md must exist.
- Agent guidance must use stdin heredoc only for phase JSON.

## 4. Technical Approach

- Keep AGENTS.md as a short router to skills/*.md.
- Create concise packet phase skills for intake, packet brief, packet split, packet verify, and packet ship.
- Remove --input wording from skills/coding.md.
- Add explicit Tiny Flow and Story Flow commands to verify and ship skills.

## 5. Files to Change

- AGENTS.md
- skills/coding.md
- skills/verify.md
- skills/ship.md
- skills/intake.md
- skills/packet-brief.md
- skills/packet-split.md
- skills/packet-verify.md
- skills/packet-ship.md

## 6. Implementation Steps

- [done] Replace AGENTS.md phase JSON command examples with skill-file references.
- [done] Remove --input fallback wording from skills/coding.md.
- [done] Add missing packet phase skill files.
- [done] Clarify verify and ship skill commands for task and story flows.
- [done] Verify agent-facing docs no longer contain --input or cat phase.json patterns.

## 7. Test Plan

### Planned

- rg -n -- "--input|input file| via stdin or |cat (request|verify|ship|intake|packet|stories|packet_verify|packet_ship)\.json" AGENTS.md skills
- for f in request verify ship intake packet-brief packet-split packet-verify packet-ship; do test -f skills/$f.md; done

### Result

- No disallowed --input or cat phase.json patterns found in AGENTS.md or skills/.
- All skill files referenced by AGENTS.md exist.

## 8. Risks

- README still documents broader CLI capability separately; this change is intentionally limited to agent-facing guidance.

## 9. Done Criteria

### Criteria

- Agent-facing phase JSON guidance uses stdin heredoc only.
- AGENTS.md stays concise by routing payload details to skills/*.md.
- No missing skill files are referenced by AGENTS.md.

### Verified

- Agent-facing phase JSON guidance uses stdin heredoc only.
- AGENTS.md stays concise by routing payload details to skills/*.md.
- No missing skill files are referenced by AGENTS.md.
