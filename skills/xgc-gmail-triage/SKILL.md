---
name: xgc-gmail-triage
description: "Gmail: Show unread inbox summary (sender, subject, date)."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc gmail +triage --help"
---

# gmail +triage

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Show unread inbox summary (sender, subject, date)

## Usage

```bash
xgc gmail +triage
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--max` | — | 20 | Maximum messages to show (default: 20) |
| `--query` | — | — | Gmail search query (default: is:unread) |
| `--labels` | — | — | Include label names in output |

## Examples

```bash
xgc gmail +triage
xgc gmail +triage --max 5 --query 'from:boss'
xgc gmail +triage --format json | jq '.[].subject'
xgc gmail +triage --labels
```

## Tips

- Read-only — never modifies your mailbox.
- Defaults to table output format.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-gmail](../xgc-gmail/SKILL.md) — All send, read, and manage email commands
