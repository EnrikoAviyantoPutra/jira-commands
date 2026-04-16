# Changelog

All notable changes to this project will be documented in this file.

Format: [Semantic Versioning](https://semver.org/) — `MAJOR.MINOR.PATCH`

---

## [0.4.0] — 2026-04-16

### Fixed
- **204 No Content**: `PUT`/`PATCH`/`DELETE` responses that return 204 are now treated as
  success. Previously, the JSON parser would error on an empty body.
- **Raw API (`jira api`)**: helper no longer tries to parse JSON when the response body is
  empty. `jira api put/delete/patch` commands now succeed silently on 204.
- **Assignee `accountId`**: `create` and `update` flows now resolve assignee to the correct
  Jira Cloud `accountId` instead of the legacy `emailAddress` / `name` field.
  - Pass an email → automatically looked up via `/user/search`
  - Pass a raw accountId (no `@`) → used directly
  - Pass `"me"` → resolved to current user via `/myself`

### Added
- `JiraClient::get_myself()` — fetches the current authenticated user's `accountId`
  from `/rest/api/3/myself`. Useful for "assign to me" flows.
- Quiet mode for non-interactive environments: spinners and progress bars are now
  suppressed when stdout is not a TTY (e.g. cron jobs, CI scripts, piped output).

### Changed
- `JiraClient::raw_request` return type changed from `Result<Value>` to
  `Result<Option<Value>>`. `None` indicates a successful 204 No Content response.
- Version bumped to **0.4.0** across `jira-core`, `jira-commands`, and the Claude Code plugin.

---

## [0.3.0] — 2026-04-15

### Added
- Claude Code plugin (`plugin/`) with 9 skills: `list-issues`, `view-issue`, `create-issue`,
  `transition`, `worklog`, `bulk-transition`, `attach`, `jql`, `api`

---

## [0.2.0] — 2026-04-15

### Added
- Phase 3 & 4: worklog CRUD, bulk transition/update, archive, JQL builder, `jira api` raw
  passthrough, `jira plan list` (Jira Premium)

---

## [0.1.0] — 2026-04-15

### Added
- Phase 1 & 2: auth, config, HTTP client with cursor-based pagination, issue CRUD,
  dynamic field introspection, attachment upload, TUI (ratatui + crossterm)
