---
name: xgc-workflow-file-announce
description: "Google Workflow: Announce a Drive file in a Chat space."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc workflow +file-announce --help"
---

# workflow +file-announce

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Announce a Drive file in a Chat space

## Usage

```bash
xgc workflow +file-announce --file-id <ID> --space <SPACE>
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--file-id` | ✓ | — | Drive file ID to announce |
| `--space` | ✓ | — | Chat space name (e.g. spaces/SPACE_ID) |
| `--message` | — | — | Custom announcement message |
| `--format` | — | — | Output format: json (default), table, yaml, csv |

## Examples

```bash
xgc workflow +file-announce --file-id FILE_ID --space spaces/ABC123
xgc workflow +file-announce --file-id FILE_ID --space spaces/ABC123 --message 'Check this out!'
```

## Tips

- This is a write command — sends a Chat message.
- Use `xgc drive +upload` first to upload the file, then announce it here.
- Fetches the file name from Drive to build the announcement.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-workflow](../xgc-workflow/SKILL.md) — All cross-service productivity workflows commands
