---
name: xgc-calendar-agenda
description: "Google Calendar: Show upcoming events across all calendars."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc calendar +agenda --help"
---

# calendar +agenda

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Show upcoming events across all calendars

## Usage

```bash
xgc calendar +agenda
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--today` | — | — | Show today's events |
| `--tomorrow` | — | — | Show tomorrow's events |
| `--week` | — | — | Show this week's events |
| `--days` | — | — | Number of days ahead to show |
| `--calendar` | — | — | Filter to specific calendar name or ID |
| `--timezone` | — | — | IANA timezone override (e.g. America/Denver). Defaults to Google account timezone. |

## Examples

```bash
xgc calendar +agenda
xgc calendar +agenda --today
xgc calendar +agenda --week --format table
xgc calendar +agenda --days 3 --calendar 'Work'
xgc calendar +agenda --today --timezone America/New_York
```

## Tips

- Read-only — never modifies events.
- Queries all calendars by default; use --calendar to filter.
- Uses your Google account timezone by default; override with --timezone.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-calendar](../xgc-calendar/SKILL.md) — All manage calendars and events commands
