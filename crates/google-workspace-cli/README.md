# xgc

**One CLI for all of Google Workspace — built for humans and AI agents.**

`xgc` dynamically generates its command surface at runtime by reading Google's [Discovery Service](https://developers.google.com/discovery). Drive, Gmail, Calendar, and every Workspace API — zero boilerplate, structured JSON output, 40+ agent skills included.

## Install

Download the pre-built binary for your OS and architecture from the **[GitHub Releases](https://github.com/ThatXliner/xgc/releases)** page.

Alternatively, you can use package managers as a convenience layer:

```bash
npm install -g @thatxliner/xgc             # npm (downloads GitHub release binary)
cargo install xgc                          # crates.io
nix run github:ThatXliner/xgc              # nix
```

## Quick Start

```bash
xgc auth login
xgc drive files list --params '{"pageSize": 5}'
xgc gmail users.messages list --params '{"maxResults": 3}'
```

## Documentation

See the [full README](https://github.com/ThatXliner/xgc#readme) for authentication setup, helper commands, agent skills, and more.

## License

Apache-2.0 — see [LICENSE](https://github.com/ThatXliner/xgc/blob/main/LICENSE).
