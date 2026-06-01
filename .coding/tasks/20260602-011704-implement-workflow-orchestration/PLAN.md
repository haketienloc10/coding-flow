# Code Plan

## 1. Objective

Implement cflow next and cflow run commands

## 2. Scope

### In Scope

- Add command_next and command_run in src/main.rs
- Update usage text

### Out of Scope

- Other commands

## 3. Requirements

- cflow next to output next command and reason
- cflow run to loop and run agent plan, agent coding, ship --dry-run automatically

## 4. Technical Approach

- Parse task folder to determine next steps based on US-next-flow.md logic
- Add new CLI matching arms for 'next' and 'run'

## 5. Files to Change

- src/main.rs

## 6. Implementation Steps

- [todo] Add get_verify_status helper
- [todo] Add determine_next_action
- [todo] Add command_next
- [todo] Add command_run
- [todo] Wire up to main CLI

## 7. Test Plan

### Planned

- Run cargo build
- Run cflow next manually to see output

### Result

- _None_

## 8. Risks

- Infinite loop in cflow run - solved by checking if next command is in safe list

## 9. Done Criteria

### Criteria

- cflow next outputs correctly
- cflow run executes safe steps

### Verified

- _None_
