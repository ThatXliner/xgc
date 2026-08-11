<h1 align="center">xgc</h1>

<p align="center"><strong>xliner’s GWS-CLI</strong></p>

<p align="center">
  <a href="https://github.com/ThatXliner/xgc/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/ThatXliner/xgc/ci.yml?branch=main&label=CI" alt="CI status"></a>
  <a href="https://github.com/ThatXliner/xgc/blob/main/LICENSE"><img src="https://img.shields.io/github/license/ThatXliner/xgc" alt="license"></a>
</p>

`xgc` is an independently maintained fork of
[Google Workspace CLI](https://github.com/googleworkspace/cli). It keeps the
upstream project's dynamic Google Discovery-based command surface while giving
the fork its own identity and first-class support for multiple authenticated
accounts.

This is not the official Google distribution and is not an officially supported
Google product.

## What changed in this fork

### The CLI is `xgc`

The executable and command name are `xgc`, fork-owned environment variables use
the `XGC_` prefix, and configuration is stored under `~/.config/xgc`.

```bash
xgc --help
xgc drive files list --params '{"pageSize": 5}'
```

This keeps the fork separate from an upstream `gws` installation. `xgc` does
not silently read, modify, or delete `~/.config/gws`.

### Named authentication profiles

Profiles let several Google accounts stay authenticated at the same time:

```bash
xgc auth login --profile personal
xgc auth login --profile school
xgc auth login --profile work

xgc --profile personal gmail users messages list
xgc --profile school drive files list
xgc --profile work calendar events list
```

You can select a profile through the environment as well. An explicit flag wins:

```bash
export XGC_PROFILE=personal
xgc gmail users messages list
xgc --profile work calendar events list
```

With no profile option, `xgc` uses the `default` profile.

```text
~/.config/xgc/
├── credentials.enc
├── token_cache.json
└── profiles/
    ├── personal/
    │   ├── credentials.enc
    │   └── token_cache.json
    └── work/
        ├── credentials.enc
        └── token_cache.json
```

Credentials, token caches, temporary OAuth state, and account-derived timezone
data are isolated per profile. OAuth client configuration and the encryption key
remain shared.

### Fork-specific configuration

The most important fork-owned variables are:

| Variable | Purpose |
|---|---|
| `XGC_PROFILE` | Select a named authentication profile |
| `XGC_CONFIG_DIR` | Override the default `~/.config/xgc` directory |
| `XGC_TOKEN` | Use a pre-obtained OAuth access token |
| `XGC_CREDENTIALS_FILE` | Read credentials from an explicit JSON file |
| `XGC_CLIENT_ID` | Supply an OAuth client ID |
| `XGC_CLIENT_SECRET` | Supply the matching OAuth client secret |

Google-defined interfaces such as `GOOGLE_APPLICATION_CREDENTIALS` retain their
standard names. See [`.env.example`](.env.example) for the complete list.

## Installation

There are no npm, Homebrew, or Nix packages for this fork at present. Install it
from source with Rust:

```bash
cargo install --git https://github.com/ThatXliner/xgc --package xgc --locked
```

Or clone the repository and build it locally:

```bash
git clone https://github.com/ThatXliner/xgc.git
cd xgc
cargo build --release --package xgc
./target/release/xgc --help
```

## Getting started

`xgc` does not borrow a shared OAuth application. Instead, the bootstrap command
walks you through creating your own Google Cloud project and Desktop OAuth client
in the browser. It does not require the `gcloud` CLI:

```bash
xgc auth bootstrap
```

After creating the Desktop client, download its JSON file and let `xgc` validate
and install it:

```bash
xgc auth bootstrap \
  --project your-google-cloud-project \
  --client-secret ./client_secret_....json

xgc auth login --profile personal
xgc --profile personal drive files list --params '{"pageSize": 5}'
```

The client configuration is shared by all local profiles; each profile receives
its own encrypted user credentials and token cache. The existing
`xgc auth setup` command remains available as an optional automated wizard for
people who already use `gcloud`.

Run `xgc <service> --help` to browse resources and methods, or inspect a method
schema directly:

```bash
xgc drive --help
xgc schema drive.files.list
```

## Migrating intentionally from `gws`

To reuse an upstream OAuth client without silently importing authenticated
tokens, copy only the client configuration and log in again:

```bash
mkdir -p ~/.config/xgc
cp ~/.config/gws/client_secret.json ~/.config/xgc/client_secret.json
xgc auth login
```

The original `~/.config/gws` directory remains untouched. Add
`--profile <name>` to the login command when creating a named profile.

## Upstream documentation

The core command model, Google API coverage, Discovery behavior, request flags,
pagination, and structured output come from the upstream project. Refer to the
[official Google Workspace CLI README](https://github.com/googleworkspace/cli/blob/main/README.md)
for the full guide.

When following upstream examples, substitute `xgc` for `gws`. Also remember that
this fork uses `XGC_*` environment variables and `~/.config/xgc` rather than the
upstream names and paths.

Fork-specific references:

- [Agent skills index](docs/skills.md)
- [Contributing](docs/CONTRIBUTING.md)
- [Environment template](.env.example)

## Development

```bash
cargo build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## Attribution and license

`xgc` is derived from
[googleworkspace/cli](https://github.com/googleworkspace/cli). Upstream
attribution and history are retained. This fork is distributed under the
[Apache-2.0 license](LICENSE).
