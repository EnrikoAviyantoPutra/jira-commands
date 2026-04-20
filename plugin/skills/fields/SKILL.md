---
description: List Jira field metadata for a project and issue type, especially before creating issues with custom fields or backlog-specific schemas
---

List available Jira fields using the `jirac` CLI.

Steps:
1. Check if `jirac` is available by running `jirac --version`. If not found, tell the user to install it with `cargo install jira-commands`.
2. Extract from the user's request:
   - Project key, ask if missing
   - Issue type, optional. If omitted, let the CLI prompt interactively.
   - Whether they only want required fields
3. Run:
   - `jirac issue fields -p <PROJECT>`
   - or `jirac issue fields -p <PROJECT> --issue-type '<TYPE>'`
   - add `--required-only` when requested
4. Explain the important field IDs and expected value shapes so the next create or update step is easier.

Examples:
- "show required fields for Story in PROJ" → `jirac issue fields -p PROJ --issue-type 'Story' --required-only`
- "what custom fields are available in PROJ" → `jirac issue fields -p PROJ`
