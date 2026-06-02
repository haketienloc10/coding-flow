Add a lightweight persistent state file to cflow.

Create `.coding/state.json` as the only persistent JSON file.

Do not reintroduce request.json, plan.json, coding.json, verify.json, or ship.json.

State file stores only metadata:
- version
- current_task_id
- tasks map
- task title
- status
- phase
- next_action
- created_at
- updated_at
- type
- lane
- summary
- artifact presence
- verify status and findings count
- ship ready/committed/commit_sha

Update state after every command:
- new -> phase=new
- request -> requested
- plan / agent plan -> planned
- coding / agent coding -> coding_done
- verify passed -> verify_passed
- verify failed/partial/skipped -> verify_failed
- ship --dry-run -> commit_pending
- ship --commit -> committed

Add commands:
- cflow status: read state and print current task summary
- cflow tasks: list known tasks
- cflow switch <task-id>: set current_task_id
- cflow state repair: rebuild state from `.coding/tasks/*` artifacts

Replace or deprecate `.coding/current`.
Prefer `state.json.current_task_id` as the source of truth.

Important:
- Markdown artifacts remain the detailed source of truth.
- State must be small and human-readable.
- If state and filesystem disagree, status should warn and suggest `cflow state repair`.
- Do not store full request/plan/coding/verify/ship content in state.json.