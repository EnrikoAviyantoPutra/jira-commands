---
description: Update an existing Jira issue, including summary, description, assignee, labels, components, fix versions, and custom fields
---

Update a Jira issue using the `jirac` CLI.

Steps:
1. Check if `jirac` is available by running `jirac --version`. If not found, tell the user to install it with `cargo install jira-commands`.
2. Extract the issue key and requested changes.
3. Map the request to supported flags such as:
   - `--summary`
   - `--assignee`
   - `--priority`
   - `--labels`
   - `--components`
   - `--fix-version`
   - `--description-file`
   - `--field customfield_XXXXX=value`
4. Run `jirac issue update <ISSUE-KEY> ...` with only the requested changes.
5. If custom fields are unclear, run `jirac issue fields -p <PROJECT> --issue-type '<TYPE>'` first.
6. Confirm the update result clearly.

Examples:
- "update PROJ-123 summary to fix OAuth callback" → `jirac issue update PROJ-123 --summary 'fix OAuth callback'`
- "set PROJ-123 priority to High and assign to me" → `jirac issue update PROJ-123 --priority High --assignee me`
