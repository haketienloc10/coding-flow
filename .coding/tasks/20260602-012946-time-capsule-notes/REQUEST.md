# Request Intake

## Summary

Build the Time Capsule Notes MVP as a local single-page app: create notes with future unlock time, hide locked content, reveal unlocked content, persist to localStorage, delete items, and verify core flows.

## Type

new_feature

## Planning Needed

true

## Lane

normal

## Risk Flags

- localStorage_persistence
- time_based_ui_state
- frontend_behavior_change

## Hard Gates

- Do not ship unless verification passes
- Use localStorage key time_capsule_notes
- No backend, auth, database, or heavy date-time package

## Assumptions

- The existing local app under focus-garden is the target web app.
- Native Date and browser localStorage are sufficient.
- There is no existing test setup beyond build scripts unless discovered otherwise.

## Clarifying Questions

- _None_

## Next Action

plan
