# Packet Split Skill

Pipe JSON directly into the CLI via stdin:

```bash
cat <<'JSON' | cflow packet split --packet current
{
  "stories": [
    {
      "id": "S01-example",
      "title": "",
      "description": "",
      "acceptance_criteria": [],
      "files_to_change": []
    }
  ]
}
JSON
```

The CLI will validate and render `STORIES.md` plus story folders. Do not create or edit markdown artifacts yourself.
