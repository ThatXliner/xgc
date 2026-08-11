---
name: xgc-workflow-standup-report
description: "Google Workflow: Today's meetings + open tasks as a standup summary."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc workflow +standup-report --help"
---

# workflow +standup-report

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Today's meetings + open tasks as a standup summary

## Usage

```bash
xgc workflow +standup-report
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--format` | — | — | Output format: json (default), table, yaml, csv |

## Examples

```bash
xgc workflow +standup-report
xgc workflow +standup-report --format table
```

## Tips

- Read-only — never modifies data.
- Combines calendar agenda (today) with tasks list.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-workflow](../xgc-workflow/SKILL.md) — All cross-service productivity workflows commands
