---
name: xgc-workflow-weekly-digest
description: "Google Workflow: Weekly summary: this week's meetings + unread email count."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc workflow +weekly-digest --help"
---

# workflow +weekly-digest

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Weekly summary: this week's meetings + unread email count

## Usage

```bash
xgc workflow +weekly-digest
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--format` | — | — | Output format: json (default), table, yaml, csv |

## Examples

```bash
xgc workflow +weekly-digest
xgc workflow +weekly-digest --format table
```

## Tips

- Read-only — never modifies data.
- Combines calendar agenda (week) with gmail triage summary.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-workflow](../xgc-workflow/SKILL.md) — All cross-service productivity workflows commands
