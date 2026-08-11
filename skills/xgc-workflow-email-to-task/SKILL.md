---
name: xgc-workflow-email-to-task
description: "Google Workflow: Convert a Gmail message into a Google Tasks entry."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc workflow +email-to-task --help"
---

# workflow +email-to-task

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Convert a Gmail message into a Google Tasks entry

## Usage

```bash
xgc workflow +email-to-task --message-id <ID>
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--message-id` | ✓ | — | Gmail message ID to convert |
| `--tasklist` | — | @default | Task list ID (default: @default) |

## Examples

```bash
xgc workflow +email-to-task --message-id MSG_ID
xgc workflow +email-to-task --message-id MSG_ID --tasklist LIST_ID
```

## Tips

- Reads the email subject as the task title and snippet as notes.
- Creates a new task — confirm with the user before executing.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-workflow](../xgc-workflow/SKILL.md) — All cross-service productivity workflows commands
