# Packet Brief Skill

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow packet brief --packet current
{
  "goal": "",
  "scope": {
    "in": [],
    "out": []
  },
  "global_acceptance_criteria": [],
  "technical_constraints": [],
  "shared_data_contracts": [],
  "validation_strategy": []
}
JSON
```

The CLI will validate and render `PACKET.md`. Do not create or edit markdown artifacts yourself.
