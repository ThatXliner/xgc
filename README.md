<h1 align="center">xgc</h1>

**xliner’s GWS-CLI — one CLI for all of Google Workspace, built for humans and AI agents.**<br>
Drive, Gmail, Calendar, and every Workspace API. Zero boilerplate. Structured JSON output. 40+ agent skills included.

> [!NOTE]
> This is xliner’s fork of the [Google Workspace CLI](https://github.com/googleworkspace/cli). It is independently maintained, is not the official Google distribution, and is not an officially supported Google product.

<p>
  <!-- npm distribution is not published yet.
  <a href="https://www.npmjs.com/package/@thatxliner/xgc"><img src="https://img.shields.io/npm/v/@thatxliner/xgc" alt="npm version"></a>
  -->
  <a href="https://github.com/ThatXliner/xgc/blob/main/LICENSE"><img src="https://img.shields.io/github/license/ThatXliner/xgc" alt="license"></a>
  <a href="https://github.com/ThatXliner/xgc/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/ThatXliner/xgc/ci.yml?branch=main&label=CI" alt="CI status"></a>
  <!-- npm distribution is not published yet.
  <a href="https://www.npmjs.com/package/@thatxliner/xgc"><img src="https://img.shields.io/npm/unpacked-size/@thatxliner/xgc" alt="install size"></a>
  -->
</p>
<br>

⬇️ **[Download the latest release for your OS](https://github.com/ThatXliner/xgc/releases)**

`xgc` doesn't ship a static list of commands. It reads Google's own [Discovery Service](https://developers.google.com/discovery) at runtime and builds its entire command surface dynamically. When Google Workspace adds an API endpoint or method, `xgc` picks it up automatically.

> [!IMPORTANT]
> This project is under active development. Expect breaking changes as we march toward v1.0.

## Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Why xgc?](#why-xgc)
- [Authentication](#authentication)
- [AI Agent Skills](#ai-agent-skills)
- [Advanced Usage](#advanced-usage)
- [Environment Variables](#environment-variables)
- [Exit Codes](#exit-codes)
- [Architecture](#architecture)
- [Troubleshooting](#troubleshooting)
- [Development](#development)

## Prerequisites

- **A Google Cloud project** — required for OAuth credentials. You can create one via the [Google Cloud Console](https://console.cloud.google.com/) or with the [`gcloud` CLI](https://cloud.google.com/sdk/docs/install) or with the `xgc auth setup` command.
- **A Google account** with access to Google Workspace

## Installation

The recommended way to install `xgc` is to download the pre-built binary for your OS and architecture from the **[GitHub Releases](https://github.com/ThatXliner/xgc/releases)** page. Extract the archive and place the `xgc` binary in your `$PATH`.

<!-- npm distribution is not published yet. Re-enable after publishing @thatxliner/xgc.
For convenience, npm can automate downloading the appropriate binary from GitHub Releases:

    npm install -g @thatxliner/xgc
-->

Or build from source:

```bash
cargo install --git https://github.com/ThatXliner/xgc --locked
```

A Nix flake is also available:

```bash
nix run github:ThatXliner/xgc
```

## Quick Start

```bash
xgc auth setup     # walks you through Google Cloud project config
xgc auth login     # subsequent OAuth login
xgc drive files list --params '{"pageSize": 5}'
```

## Why xgc?

**For humans** — stop writing `curl` calls against REST docs. `xgc` gives you `--help` on every resource, `--dry-run` to preview requests, and auto‑pagination.

**For AI agents** — every response is structured JSON. Pair it with the included agent skills and your LLM can manage Workspace without custom tooling.

```bash
# List the 10 most recent files
xgc drive files list --params '{"pageSize": 10}'

# Create a spreadsheet
xgc sheets spreadsheets create --json '{"properties": {"title": "Q1 Budget"}}'

# Send a Chat message
xgc chat spaces messages create \
  --params '{"parent": "spaces/xyz"}' \
  --json '{"text": "Deploy complete."}' \
  --dry-run

# Introspect any method's request/response schema
xgc schema drive.files.list

# Stream paginated results as NDJSON
xgc drive files list --params '{"pageSize": 100}' --page-all | jq -r '.files[].name'
```

## Authentication

The CLI supports multiple auth workflows so it works on your laptop, in CI, and on a server.

### Which setup should I use?

| I have… | Use |
|---|---|
| `gcloud` installed and authenticated | [`xgc auth setup`](#interactive-local-desktop) (fastest) |
| A GCP project but no `gcloud` | [Manual OAuth setup](#manual-oauth-setup-google-cloud-console) |
| An existing OAuth access token | [`XGC_TOKEN`](#pre-obtained-access-token) |
| Existing Credentials | [`XGC_CREDENTIALS_FILE`](#service-account-server-to-server) |

### Interactive (local desktop)

Credentials are encrypted at rest (AES-256-GCM) with the key stored in your OS keyring (or `~/.config/xgc/.encryption_key` when `XGC_KEYRING_BACKEND=file`).

```bash
xgc auth setup       # one-time: creates a Cloud project, enables APIs, logs you in
xgc auth login       # subsequent scope selection and login
```

### Multiple accounts and profiles

Use named profiles to keep account credentials, token caches, and account-derived settings isolated:

```bash
xgc auth login --profile personal
xgc auth login --profile school
xgc --profile personal gmail users messages list
xgc --profile school drive files list
```

`XGC_PROFILE` provides a default for an invocation, and the CLI flag takes precedence:

```bash
export XGC_PROFILE=personal
xgc gmail users messages list
xgc --profile school drive files list
```

Without either setting, xgc uses the `default` profile. Its files remain directly under `~/.config/xgc/`; named profiles live under `~/.config/xgc/profiles/<profile>/`. OAuth client setup and the encryption key are shared across profiles, while credentials, token caches, and the cached account timezone are isolated.

### Migrating intentionally from upstream `gws`

xgc never reads, modifies, or deletes `~/.config/gws` automatically. To reuse an upstream OAuth client without copying authenticated tokens, copy only the client configuration and log in again:

```bash
mkdir -p ~/.config/xgc
cp ~/.config/gws/client_secret.json ~/.config/xgc/client_secret.json
xgc auth login
```

This makes the sensitive-file migration explicit and leaves the upstream installation untouched. For a named xgc profile, add `--profile <name>` to the login command. If the upstream client file is elsewhere, place it at `~/.config/xgc/client_secret.json` or set `XGC_CLIENT_ID` and `XGC_CLIENT_SECRET` instead.

> `xgc auth setup` requires the [`gcloud` CLI](https://cloud.google.com/sdk/docs/install). If you don't have `gcloud`, use the [manual setup](#manual-oauth-setup-google-cloud-console) below instead.

> [!WARNING]
> **Scope limits in testing mode:** If your OAuth app is unverified (testing mode),
> Google limits consent to ~25 scopes. The `recommended` scope preset includes 85+
> scopes and **will fail** for unverified apps (especially for `@gmail.com` accounts).
> Choose individual services instead to filter the scope picker:
> ```bash
> xgc auth login -s drive,gmail,sheets
> ```


### Manual OAuth setup (Google Cloud Console)

Use this when `xgc auth setup` cannot automate project/client creation, or when you want explicit control.

1. Open Google Cloud Console in the target project:
   - OAuth consent screen: `https://console.cloud.google.com/apis/credentials/consent?project=<PROJECT_ID>`
   - Credentials: `https://console.cloud.google.com/apis/credentials?project=<PROJECT_ID>`
2. Configure OAuth branding/audience if prompted:
   - App type: **External** (testing mode is fine)
3. Add your account under **Test users**
4. Create an OAuth client:
   - Type: **Desktop app**
5. Download the client JSON and save it to:
   - `~/.config/xgc/client_secret.json`

> [!IMPORTANT]
> **You must add yourself as a test user.** In the OAuth consent screen, click
> **Test users → Add users** and enter your Google account email. Without this,
> login will fail with a generic "Access blocked" error.

Then run:

```bash
xgc auth login
```

### Browser-assisted auth (human or agent)

You can complete OAuth either manually or with browser automation.

- **Human flow**: run `xgc auth login`, open the printed URL, approve scopes.
- **Agent-assisted flow**: the agent opens the URL, selects account, handles consent prompts, and returns control once the localhost callback succeeds.

If consent shows **"Google hasn't verified this app"** (testing mode), click **Continue**.
If scope checkboxes appear, select required scopes (or **Select all**) before continuing.

### Headless / CI (export flow)

1. Complete interactive auth on a machine with a browser.
2. Export credentials:
   ```bash
   xgc auth export --unmasked > credentials.json
   ```
3. On the headless machine:
   ```bash
   export XGC_CREDENTIALS_FILE=/path/to/credentials.json
   xgc drive files list   # just works
   ```

### Service Account (server-to-server)

Point to your key file; no login needed.

```bash
export XGC_CREDENTIALS_FILE=/path/to/service-account.json
xgc drive files list
```

### Pre-obtained Access Token

Useful when another tool (e.g. `gcloud`) already mints tokens for your environment.

```bash
export XGC_TOKEN=$(gcloud auth print-access-token)
```

### Precedence

| Priority | Source                 | Set via                                 |
| -------- | ---------------------- | --------------------------------------- |
| 1        | Access token           | `XGC_TOKEN`            |
| 2        | Credentials file       | `XGC_CREDENTIALS_FILE` |
| 3        | Encrypted credentials  | `xgc auth login`                        |
| 4        | Plaintext credentials  | `~/.config/xgc/credentials.json`        |

Environment variables can also live in a `.env` file.

## AI Agent Skills

The repo ships 100+ Agent Skills (`SKILL.md` files) — one for every supported API, plus higher-level helpers for common workflows and 50 curated recipes for Gmail, Drive, Docs, Calendar, and Sheets. See the full [Skills Index](docs/skills.md) for the complete list.

```bash
# Install all skills at once
npx skills add https://github.com/ThatXliner/xgc

# Or pick only what you need
npx skills add https://github.com/ThatXliner/xgc/tree/main/skills/xgc-drive
npx skills add https://github.com/ThatXliner/xgc/tree/main/skills/xgc-gmail
```

<details>
<summary>OpenClaw setup</summary>

```bash
# Symlink all skills (stays in sync with repo)
ln -s $(pwd)/skills/xgc-* ~/.openclaw/skills/

# Or copy specific skills
cp -r skills/xgc-drive skills/xgc-gmail ~/.openclaw/skills/
```

The `xgc-shared` skill includes an `install` block so OpenClaw auto-installs the CLI via `npm` if `xgc` isn't on PATH.

</details>

## Gemini CLI Extension

1. Authenticate the CLI first:

   ```bash
   xgc auth setup
   ```

2. Install the extension into the Gemini CLI:
   ```bash
   gemini extensions install https://github.com/ThatXliner/xgc
   ```

Installing this extension gives your Gemini CLI agent direct access to all `xgc` commands and Google Workspace agent skills. Because `xgc` handles its own authentication securely, you simply need to authenticate your terminal once prior to using the agent, and the extension will automatically inherit your credentials.

## Advanced Usage

### Multipart Uploads

```bash
xgc drive files create --json '{"name": "report.pdf"}' --upload ./report.pdf
```

### Pagination

| Flag                | Description                                    | Default |
| ------------------- | ---------------------------------------------- | ------- |
| `--page-all`        | Auto-paginate, one JSON line per page (NDJSON) | off     |
| `--page-limit <N>`  | Max pages to fetch                             | 10      |
| `--page-delay <MS>` | Delay between pages                            | 100 ms  |

### Google Sheets — Shell Escaping

Sheets ranges use `!` which bash interprets as history expansion. Always wrap values in **single quotes**:

```bash
# Read cells A1:C10 from "Sheet1"
xgc sheets spreadsheets values get \
  --params '{"spreadsheetId": "SPREADSHEET_ID", "range": "Sheet1!A1:C10"}'

# Append rows
xgc sheets spreadsheets values append \
  --params '{"spreadsheetId": "ID", "range": "Sheet1!A1", "valueInputOption": "USER_ENTERED"}' \
  --json '{"values": [["Name", "Score"], ["Alice", 95]]}'
```

### Helper Commands

Some services ship hand-crafted helper commands alongside the auto-generated Discovery surface. Helper commands are prefixed with `+` so they are visually distinct and never collide with Discovery-generated method names.

Time-aware helpers (`+agenda`, `+standup-report`, `+weekly-digest`, `+meeting-prep`) automatically use your **Google account timezone** (fetched from Calendar Settings API and cached for 24 hours). Override with `--timezone`/`--tz` on `+agenda`, or set the `--timezone` flag for explicit control.

Run `xgc <service> --help` to see both Discovery methods and helper commands together.

```bash
xgc gmail --help      # shows +send, +reply, +reply-all, +forward, +triage, +watch …
xgc calendar --help   # shows +insert, +agenda …
xgc drive --help      # shows +upload …
```

**Full helper reference:**

| Service | Command | Description |
|---------|---------|-------------|
| `gmail` | `+send` | Send an email |
| `gmail` | `+reply` | Reply to a message (handles threading automatically) |
| `gmail` | `+reply-all` | Reply-all to a message |
| `gmail` | `+forward` | Forward a message to new recipients |
| `gmail` | `+triage` | Show unread inbox summary (sender, subject, date) |
| `gmail` | `+watch` | Watch for new emails and stream them as NDJSON |
| `sheets` | `+append` | Append a row to a spreadsheet |
| `sheets` | `+read` | Read values from a spreadsheet |
| `docs` | `+write` | Append text to a document |
| `chat` | `+send` | Send a message to a space |
| `drive` | `+upload` | Upload a file with automatic metadata |
| `calendar` | `+insert` | Create a new event |
| `calendar` | `+agenda` | Show upcoming events (uses Google account timezone; override with `--timezone`) |
| `script` | `+push` | Replace all files in an Apps Script project with local files |
| `workflow` | `+standup-report` | Today's meetings + open tasks as a standup summary |
| `workflow` | `+meeting-prep` | Prepare for your next meeting: agenda, attendees, and linked docs |
| `workflow` | `+email-to-task` | Convert a Gmail message into a Google Tasks entry |
| `workflow` | `+weekly-digest` | Weekly summary: this week's meetings + unread email count |
| `workflow` | `+file-announce` | Announce a Drive file in a Chat space |
| `events` | `+subscribe` | Subscribe to Workspace events and stream them as NDJSON |
| `events` | `+renew` | Renew/reactivate Workspace Events subscriptions |
| `modelarmor` | `+sanitize-prompt` | Sanitize a user prompt through a Model Armor template |
| `modelarmor` | `+sanitize-response` | Sanitize a model response through a Model Armor template |
| `modelarmor` | `+create-template` | Create a new Model Armor template |

**Examples:**

```bash
# Send an email
xgc gmail +send --to alice@example.com --subject "Hello" --body "Hi there"

# Reply to a message
xgc gmail +reply --message-id MESSAGE_ID --body "Thanks!"

# Append a row to a spreadsheet
xgc sheets +append --spreadsheet SPREADSHEET_ID --values "Alice,95"

# Show today's calendar agenda
xgc calendar +agenda

# Upload a file to Drive
xgc drive +upload ./report.pdf --name "Q1 Report"

# Morning standup summary
xgc workflow +standup-report

# Show today's agenda in a specific timezone
xgc calendar +agenda --today --timezone America/New_York
```

### Model Armor (Response Sanitization)

Integrate [Google Cloud Model Armor](https://cloud.google.com/security/products/model-armor) to scan API responses for prompt injection before they reach your agent.

```bash
xgc gmail users messages get --params '...' \
  --sanitize "projects/P/locations/L/templates/T"
```

| Variable                                 | Description                  |
| ---------------------------------------- | ---------------------------- |
| `XGC_SANITIZE_TEMPLATE` | Default Model Armor template |
| `XGC_SANITIZE_MODE`     | `warn` (default) or `block`  |

## Environment Variables

All variables are optional. See [`.env.example`](.env.example) for a copy-paste template.

| Variable | Description |
|---|---|
| `XGC_TOKEN` | Pre-obtained OAuth2 access token (highest priority) |
| `XGC_CREDENTIALS_FILE` | Path to OAuth credentials JSON (user or service account) |
| `XGC_CLIENT_ID` | OAuth client ID (alternative to `client_secret.json`) |
| `XGC_CLIENT_SECRET` | OAuth client secret (paired with `CLIENT_ID`) |
| `XGC_CONFIG_DIR` | Override config directory (default: `~/.config/xgc`) |
| `XGC_PROFILE` | Named auth profile (default: `default`; overridden by `--profile`) |
| `XGC_SANITIZE_TEMPLATE` | Default Model Armor template |
| `XGC_SANITIZE_MODE` | `warn` (default) or `block` |
| `XGC_LOG` | Log level for stderr (e.g., `xgc=debug`). Off by default. |
| `XGC_LOG_FILE` | Directory for JSON log files with daily rotation. Off by default. |
| `XGC_PROJECT_ID` | GCP project ID override for quota/billing and fallback for helper commands |

Environment variables can also be set in a `.env` file (loaded via [dotenvy](https://crates.io/crates/dotenvy)).

## Exit Codes

`xgc` uses structured exit codes so scripts can branch on the failure type without parsing error output.

| Code | Meaning | Example cause |
|------|---------|---------------|
| `0` | Success | Command completed normally |
| `1` | API error | Google returned a 4xx/5xx response |
| `2` | Auth error | Credentials missing, expired, or invalid |
| `3` | Validation error | Bad arguments, unknown service, invalid flag |
| `4` | Discovery error | Could not fetch the API schema document |
| `5` | Internal error | Unexpected failure |

```bash
xgc drive files list --params '{"fileId": "bad"}'
echo $?   # 1 — API error

xgc unknown-service files list
echo $?   # 3 — validation error (unknown service)
```

## Architecture

`xgc` uses a **two-phase parsing** strategy:

1. Read `argv[1]` to identify the service (e.g. `drive`)
2. Fetch the service's Discovery Document (cached 24 h)
3. Build a `clap::Command` tree from the document's resources and methods
4. Re-parse the remaining arguments
5. Authenticate, build the HTTP request, execute

All output — success, errors, download metadata — is structured JSON.

## Troubleshooting

### "Access blocked" or 403 during login

Your OAuth app is in **testing mode** and your account is not listed as a test user.

**Fix:** Open the [OAuth consent screen](https://console.cloud.google.com/apis/credentials/consent) in your GCP project → **Test users** → **Add users** → enter your Google account email. Then retry `xgc auth login`.

### "Google hasn't verified this app"

Expected when your app is in testing mode. Click **Advanced** → **Go to \<app name\> (unsafe)** to proceed. This is safe for personal use; verification is only required to publish the app to other users.

### Too many scopes / consent screen error

Unverified (testing mode) apps are limited to ~25 OAuth scopes. The `recommended` scope preset includes many scopes and will exceed this limit.

**Fix:** Select only the scopes you need:

```bash
xgc auth login --scopes drive,gmail,calendar
```

### `gcloud` CLI not found

`xgc auth setup` requires the `gcloud` CLI to automate project creation. You have three options:

1. [Install gcloud](https://cloud.google.com/sdk/docs/install) and use `gcloud` directly.
2. Re-run `xgc auth setup` which wraps `gcloud` calls.
3. Skip `gcloud` entirely — set up OAuth credentials manually in the [Cloud Console](#manual-oauth-setup-google-cloud-console)

### `redirect_uri_mismatch`

The OAuth client was not created as a **Desktop app** type. In the [Credentials](https://console.cloud.google.com/apis/credentials) page, delete the existing client, create a new one with type **Desktop app**, and download the new JSON.

### API not enabled — `accessNotConfigured`

If a required Google API is not enabled for your GCP project, you will see a
403 error with reason `accessNotConfigured`:

```json
{
  "error": {
    "code": 403,
    "message": "Gmail API has not been used in project 549352339482 ...",
    "reason": "accessNotConfigured",
    "enable_url": "https://console.developers.google.com/apis/api/gmail.googleapis.com/overview?project=549352339482"
  }
}
```

`xgc` also prints an actionable hint to **stderr**:

```
💡 API not enabled for your GCP project.
   Enable it at: https://console.developers.google.com/apis/api/gmail.googleapis.com/overview?project=549352339482
   After enabling, wait a few seconds and retry your command.
```

**Steps to fix:**

1. Click the `enable_url` link (or copy it from the `enable_url` JSON field).
2. In the GCP Console, click **Enable**.
3. Wait ~10 seconds, then retry your `xgc` command.

> [!TIP]
> You can also run `xgc auth setup` which walks you through enabling all required
> APIs for your project automatically.

## Development

```bash
cargo build                       # dev build
cargo clippy -- -D warnings       # lint
cargo test                        # unit tests
./scripts/coverage.sh             # HTML coverage report → target/llvm-cov/html/
```

## License

Apache-2.0

## Disclaimer

> [!CAUTION]
> This is **not** an officially supported Google product.
