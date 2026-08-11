---
name: xgc-workflow
description: "Google Workflow: Cross-service productivity workflows."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - xgc
    cliHelp: "xgc workflow --help"
---

# workflow (v1)

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

```bash
xgc workflow <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+standup-report`](../xgc-workflow-standup-report/SKILL.md) | Today's meetings + open tasks as a standup summary |
| [`+meeting-prep`](../xgc-workflow-meeting-prep/SKILL.md) | Prepare for your next meeting: agenda, attendees, and linked docs |
| [`+email-to-task`](../xgc-workflow-email-to-task/SKILL.md) | Convert a Gmail message into a Google Tasks entry |
| [`+weekly-digest`](../xgc-workflow-weekly-digest/SKILL.md) | Weekly summary: this week's meetings + unread email count |
| [`+file-announce`](../xgc-workflow-file-announce/SKILL.md) | Announce a Drive file in a Chat space |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
xgc workflow --help

# Inspect a method's required params, types, and defaults
xgc schema workflow.<resource>.<method>
```

Use `xgc schema` output to build your `--params` and `--json` flags.
