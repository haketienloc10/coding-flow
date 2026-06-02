# Packet Ship Skill

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow packet ship --packet current --dry-run
{
  "changelog": [],
  "commit_message": ""
}
JSON
```

The CLI will validate and render `PACKET_SHIP.md`. Only use `--commit` if explicitly requested by the user.
