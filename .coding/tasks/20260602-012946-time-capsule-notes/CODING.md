# Coding

## Status

ready_for_verify

## Summary

- Replaced the Focus Garden screen with the Time Capsule Notes single-page MVP.
- Implemented capsule creation with Title, Message, Unlock at, exact past-time validation error, and a Seal capsule button that stays disabled until the form is currently valid.
- Persisted capsules under localStorage key time_capsule_notes with defensive load validation and newest-first sorting.
- Rendered locked capsules with title, unlock time, Locked badge, and placeholder only; rendered unlocked capsules with Unlocked badge and real content.
- Added one-second refresh for automatic lock-to-unlock updates and ongoing form validity, per-item Delete, exact empty state text, and favicon link to avoid console/network 404 noise.

## Changed Files

- focus-garden/index.html
- focus-garden/src/main.js
- focus-garden/src/style.css

## Notes

- No backend, auth, database, state management library, or date-time package was added.
- focus-garden has build/dev scripts only; no lint/typecheck/test scripts are defined.

## Next

verify
