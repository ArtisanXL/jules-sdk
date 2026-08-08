# jules-cli

> The official command-line interface for Jules-SDK.

[![Crates.io](https://img.shields.io/crates/v/jules-cli)](https://crates.io/crates/jules-cli)
[![Docs.rs](https://img.shields.io/docsrs/jules-cli)](https://docs.rs/jules-cli)
[![License](https://img.shields.io/crates/l/jules-cli)](../../LICENSE)

> **Status:** Pre-Alpha. `chat`, `sessions`, and `sources` are wired to the real `v1alpha` Jules API through [`jules-api`](../jules-api)'s `JulesClient`. See [PROJECT_STATE.md](../../PROJECT_STATE.md) for authoritative status.

## Installation

```bash
cargo install jules-cli
```

Or build from the workspace:

```bash
cargo build --release -p jules-cli
```

## Configuration

`jules-cli` needs an API key before any network command will run. Set it once and it's persisted to your platform's config directory (`jules-cli/config.json`):

```bash
jules-cli config set --api-key YOUR_API_KEY
```

Configuration is resolved in this precedence order (highest first):

1. Explicit CLI flags, where a command exposes them
2. Environment variables: `JULES_API_KEY`, `JULES_BASE_URL`
3. The persisted config file
4. Unset (network commands fail with `MissingApiKey` until an API key is configured)

```bash
# Inspect the resolved configuration (API key is redacted in output).
jules-cli config show

# Override the API base URL (e.g. for a local test server).
jules-cli config set --base-url https://example.test
```

Without a configured API key, any network command fails with:

```text
Error: no Jules API key configured; run `jules-cli config set --api-key <KEY>`
```

## Commands

Every command accepts a global `--format <plain|json>` flag (default `plain`).

### `chat` — create a session or send a message

```bash
# Create a new, repoless session with this prompt.
jules-cli chat "Summarize the last commit"

# Create a session against a GitHub repo, optionally on a specific branch.
jules-cli chat "Fix the failing test" --source owner/repo --branch main --title "Fix CI"

# Send a follow-up message to an existing session (by id or full resource name).
jules-cli chat "keep going" --session 12345
```

`--branch` requires `--source`. `--source` must be in `owner/repo` form — it's expanded to the `sources/github/owner/repo` resource name internally.

### `sessions` — list or inspect sessions (read-only)

```bash
jules-cli sessions list --page-size 20
jules-cli sessions list --page-token <token-from-previous-response>
jules-cli sessions get 12345          # bare ids are qualified to `sessions/12345`
jules-cli sessions get sessions/12345 # already-qualified names pass through unchanged
```

### `sources` — list connected sources (read-only)

```bash
jules-cli sources list
jules-cli sources list --page-size 50 --page-token <token>
```

### `config` — inspect or persist local configuration

```bash
jules-cli config show
jules-cli config set --api-key YOUR_API_KEY --base-url https://jules.googleapis.com
```

## Output formats

```bash
jules-cli sessions list --format json
```

`plain` renders a human-readable summary; `json` renders the same data as pretty-printed JSON, suitable for piping into `jq` or other tooling.

## More

* [jules-sdk](../jules-sdk) — the SDK crate this CLI is built on
* [Root README](../../README.md) · [PROJECT_STATE.md](../../PROJECT_STATE.md)

## References

Command behavior (resource naming, pagination, session states) follows the [Jules API REST reference](https://developers.google.com/jules/api/reference/rest), cross-checked against [google-labs-code/jules-sdk](https://github.com/google-labs-code/jules-sdk).

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache-2.0](../../LICENSE-APACHE), at your option.
