# Changelog

All notable changes to this project will be documented in this file.

Format: [Semantic Versioning](https://semver.org/) — `MAJOR.MINOR.PATCH`

---

## [0.13.0](https://github.com/EnrikoAviyantoPutra/jira-commands/compare/v0.12.1...v0.13.0) (2026-04-21)


### Features

* add bulk-create, clone, batch, json mode, and TUI edit actions ([28e1893](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/28e18938cc8815041c71df04141c7f52c23963c1))
* add Claude Code plugin (9 skills) + bump version to 0.3.0 ([2d33454](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/2d334549ef04c217c31dd338311a1b4879fb7333))
* add homebrew tap & tui improvements ([e07e6a2](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/e07e6a215a0bd0b8c59e489849a8afc2760203d7))
* add Homebrew tap, TUI cursor fix, and created/updated columns ([2264699](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/2264699180b1091cf21a3d8fccccd30a15b7146f))
* add Jira issue comment support ([511ec27](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/511ec279591d592ef628de299e24396c20db718d))
* add jira mcp ([2189838](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/21898386c4f0c8e45c8d97a33e35f1cdd3075d88))
* add jira mcp ([d8a10eb](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/d8a10eb37d8a8ba3f0b986ca966ed0fc9f33a63d))
* add plausible analytics snippet ([93b254d](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/93b254dc55d003ebbba2a207a9df77ecdc32f8fa))
* add plausible analytics snippet for jirac docs ([9ee32cf](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/9ee32cf132e71bc43335baa9001837652b3458ae))
* add started date/time prompts to TUI worklog ([f79ec5c](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/f79ec5cbc3168dc74f532ad2444f2b64373c55a5))
* add started timestamp options for worklog CLI ([73e1283](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/73e12834201b074f834b5cc291fbb984fee0dbfb))
* add table ADF support and plugin skill coverage ([1999948](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/1999948cab85bb85ea6dc91852cf2b6cca8ef71a))
* adjust release please ([9f757ad](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/9f757ada51674c9fe7beeb3008b4cbe22cabc8d3))
* adjust release please config ([549e748](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/549e748cef5c66c32cd9947033b15f214a8e3d2b))
* adjust release please config ([5a6c4cb](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/5a6c4cb44b729a6380a77fe4bf062aed7a5915e2))
* adjust release please fix ([90d3a2f](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/90d3a2fe2f714b0511213123f30d6e8dac001376))
* adjust release please manifest ([2561cdc](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/2561cdcb0c78f2d398d3096385302bf53b850ac5))
* distribute via homebrew tap(mulham/tap) ([1683a65](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/1683a657c2c150140095c008dc9e37ac03c530c7))
* release please config crates ([18f7646](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/18f764624055e4d4bf1b35b0e09006cbf71966b1))
* rename binary to jirac with backward-compat shim and install script ([e640726](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/e640726558b34ba20dcf22ca79bc3728ae5ce67c))
* rename binary to jirac with backward-compat shim and install script agents.md ([8ceb7e6](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/8ceb7e6d8d9f8e433fe6f4d41686eb85197f1716))
* stabilize release-please workspace publishing ([e1e0b56](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/e1e0b561961f828f7ee162117eba3790f3ede772))
* update readme ([a9f5a9e](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/a9f5a9e2ce90bcc51adde9b0651bc6e54524063d))
* v0.2.0 ([5338e8d](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/5338e8d7e1b4df46b0a4c557045807e9dfee7bff))


### Bug Fixes

* align ADF table conversion with comrak ([63ae954](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/63ae9548fa3fc6fe3b1d35fb156640458124bfc7))
* align adf table tests and jirac migration docs ([f395763](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/f3957632811049cc3e427195ca67fd3f14661390))
* avoid unsupported comrak text_contents helper ([a09c313](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/a09c31337dfb3001e0fa04211241f68d1ad837e2))
* **ci:** broaden release-please package path from crates/jira to workspace root ([3e28357](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/3e28357fe6e0618d76fc6d835fbee28c6278a931))
* **ci:** comprehensive fixes for failing CI jobs ([1d19b46](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/1d19b46a2aac10ea36a6f0d43b77f40f2fc2ea8a))
* **ci:** idempotent crates.io publish + sparse index wait ([eed0bea](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/eed0bea095da1dac1761c1d2ca2e5ec2a0925d5d))
* **ci:** release-please path resolution + CHANGELOG location ([aa18a5f](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/aa18a5f4132a907be3cc7252c311439f5fae1ab8))
* **ci:** replace linked-versions with single-package driver ([0578e9b](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/0578e9b096d1b2189b03b034b0d61637286cb2e7))
* **ci:** resolve cargo-deny failures ([8059f70](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/8059f7058bc7631fb3d9cc3309aaf0069ca74b12))
* **ci:** revert release-please package path and widen scope via include-paths ([02a7d05](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/02a7d05b1159fcbf89bb87005f9859aa3f5b3af9))
* **ci:** rework release-please to scope whole repo via simple + VERSION ([87202a0](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/87202a0ed383edbeb969126d3d0708af14c90098))
* **ci:** sed handle magic comment + remove redundant extra-file ([610b9fa](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/610b9faf4d91aa7f4a21cf7bbfbf2c4c80b8b6f5))
* **ci:** sync jira-core dep version on release-please bumps ([f6528b3](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/f6528b378a0f8e5458bd7fd74e33e004a2a32db8))
* correct paths-filter predicate setting ([899a504](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/899a504ae1a48d4050c4862c21cf47f6714e9277))
* **deps:** tighten jira-core requirement to &gt;= 0.4.1 ([7d341f1](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/7d341f15a518d1da6bf1af839ed2a827dfbd2b2b))
* fix cargo-deny unmaintained format + disable audit issue creation ([27af9d3](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/27af9d3b3f4fd5abb868ede671804c7a99937722))
* **jira-mcp:** shorten keyword to fit crates.io 20-char limit ([437540e](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/437540ebb16dec94c9d73d73c6647aea5da5cd0b))
* keep ADF table rendering read-side only ([b0fc2cd](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/b0fc2cde8215659a0d7bd7205800c0cf216faa5c))
* match rustfmt output for comment model ([daf43db](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/daf43dbefe0d31d905a258cf9cdbc2f0d0bcc0f1))
* **release:** clarify shipped binaries in release docs ([5683204](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/5683204ba88fd646b75b06856ae1df17180db7bf))
* remove stray blank line in adf formatter output ([b2954bd](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/b2954bd0a0e405f47908e93921d7adf2f68e04fb))
* repair CI action pin and tidy crate READMEs ([150a5f5](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/150a5f51ac331478b92294315d4baf5f839e5dbd))
* switch release-please to rust workspace with linked-versions ([e9b9b96](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/e9b9b961b8b3b733da62b4a7809302bf1e2b3706))
* switch reqwest to rustls-tls for cross-compilation ([f0257c0](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/f0257c0323fa2aa66ccc4a79b15bd3cbeb34a725))
* track the full cargo workspace in release-please ([9f08a2f](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/9f08a2fe7c2ff81dc403d9e17d8b00577e77cebf))
* **tui:** show cursor in JQL search bar ([30424c6](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/30424c6f6e8ac9cc4af57043944d128aa043c674))
* use the official release-please action ([405514e](https://github.com/EnrikoAviyantoPutra/jira-commands/commit/405514ee91192177c161f30ab669364d3dcef49a))

## [0.12.1](https://github.com/mulhamna/jira-commands/compare/v0.12.0...v0.12.1) (2026-04-21)


### Bug Fixes

* **release:** clarify shipped binaries in release docs ([5683204](https://github.com/mulhamna/jira-commands/commit/5683204ba88fd646b75b06856ae1df17180db7bf))

## [0.12.0](https://github.com/mulhamna/jira-commands/compare/v0.11.0...v0.12.0) (2026-04-21)


### Features

* add Jira issue comment support ([511ec27](https://github.com/mulhamna/jira-commands/commit/511ec279591d592ef628de299e24396c20db718d))


### Bug Fixes

* align adf table tests and jirac migration docs ([f395763](https://github.com/mulhamna/jira-commands/commit/f3957632811049cc3e427195ca67fd3f14661390))
* avoid unsupported comrak text_contents helper ([a09c313](https://github.com/mulhamna/jira-commands/commit/a09c31337dfb3001e0fa04211241f68d1ad837e2))
* match rustfmt output for comment model ([daf43db](https://github.com/mulhamna/jira-commands/commit/daf43dbefe0d31d905a258cf9cdbc2f0d0bcc0f1))

## [0.11.0](https://github.com/mulhamna/jira-commands/compare/v0.10.0...v0.11.0) (2026-04-20)


### Features

* add table ADF support and plugin skill coverage ([1999948](https://github.com/mulhamna/jira-commands/commit/1999948cab85bb85ea6dc91852cf2b6cca8ef71a))


### Bug Fixes

* align ADF table conversion with comrak ([63ae954](https://github.com/mulhamna/jira-commands/commit/63ae9548fa3fc6fe3b1d35fb156640458124bfc7))
* correct paths-filter predicate setting ([899a504](https://github.com/mulhamna/jira-commands/commit/899a504ae1a48d4050c4862c21cf47f6714e9277))
* keep ADF table rendering read-side only ([b0fc2cd](https://github.com/mulhamna/jira-commands/commit/b0fc2cde8215659a0d7bd7205800c0cf216faa5c))
* remove stray blank line in adf formatter output ([b2954bd](https://github.com/mulhamna/jira-commands/commit/b2954bd0a0e405f47908e93921d7adf2f68e04fb))
* repair CI action pin and tidy crate READMEs ([150a5f5](https://github.com/mulhamna/jira-commands/commit/150a5f51ac331478b92294315d4baf5f839e5dbd))

## [0.10.0](https://github.com/mulhamna/jira-commands/compare/v0.9.0...v0.10.0) (2026-04-20)


### Features

* add plausible analytics snippet for jirac docs ([9ee32cf](https://github.com/mulhamna/jira-commands/commit/9ee32cf132e71bc43335baa9001837652b3458ae))

## [0.9.0](https://github.com/mulhamna/jira-commands/compare/v0.8.1...v0.9.0) (2026-04-19)


### Features

* add plausible analytics snippet ([93b254d](https://github.com/mulhamna/jira-commands/commit/93b254dc55d003ebbba2a207a9df77ecdc32f8fa))

## [0.8.1](https://github.com/mulhamna/jira-commands/compare/v0.8.0...v0.8.1) (2026-04-19)


### Bug Fixes

* **ci:** broaden release-please package path from crates/jira to workspace root ([3e28357](https://github.com/mulhamna/jira-commands/commit/3e28357fe6e0618d76fc6d835fbee28c6278a931))
* **ci:** revert release-please package path and widen scope via include-paths ([02a7d05](https://github.com/mulhamna/jira-commands/commit/02a7d05b1159fcbf89bb87005f9859aa3f5b3af9))
* **ci:** rework release-please to scope whole repo via simple + VERSION ([87202a0](https://github.com/mulhamna/jira-commands/commit/87202a0ed383edbeb969126d3d0708af14c90098))
* **jira-mcp:** shorten keyword to fit crates.io 20-char limit ([437540e](https://github.com/mulhamna/jira-commands/commit/437540ebb16dec94c9d73d73c6647aea5da5cd0b))

## [0.8.0](https://github.com/mulhamna/jira-commands/compare/v0.7.0...v0.8.0) (2026-04-19)


### Features

* adjust release please fix ([90d3a2f](https://github.com/mulhamna/jira-commands/commit/90d3a2fe2f714b0511213123f30d6e8dac001376))
* release please config crates ([18f7646](https://github.com/mulhamna/jira-commands/commit/18f764624055e4d4bf1b35b0e09006cbf71966b1))
* stabilize release-please workspace publishing ([e1e0b56](https://github.com/mulhamna/jira-commands/commit/e1e0b561961f828f7ee162117eba3790f3ede772))

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
