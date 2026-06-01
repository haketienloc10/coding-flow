#!/usr/bin/env bash
cat <<'JSON' | node ../bin/cflow request --task current 
{
  "summary": "Add a lightweight copy button to code blocks in documentation pages.",
  "type": "new_feature",
  "planning_needed": true,
  "lane": "tiny",
  "risk_flags": ["existing_behavior_change"],
  "hard_gates": [],
  "assumptions": [
    "The button only affects documentation code blocks.",
    "No backend or data model changes are required."
  ],
  "clarifying_questions": [],
  "next_action": "plan"
}
JSON
