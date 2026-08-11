---
name: xgc-modelarmor-sanitize-response
description: "Google Model Armor: Sanitize a model response through a Model Armor template."
metadata:
  version: 0.22.5
  openclaw:
    category: "security"
    requires:
      bins:
        - xgc
    cliHelp: "xgc modelarmor +sanitize-response --help"
---

# modelarmor +sanitize-response

> **PREREQUISITE:** Read `../xgc-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `xgc generate-skills` to create it.

Sanitize a model response through a Model Armor template

## Usage

```bash
xgc modelarmor +sanitize-response --template <NAME>
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--template` | ✓ | — | Full template resource name (projects/PROJECT/locations/LOCATION/templates/TEMPLATE) |
| `--text` | — | — | Text content to sanitize |
| `--json` | — | — | Full JSON request body (overrides --text) |

## Examples

```bash
xgc modelarmor +sanitize-response --template projects/P/locations/L/templates/T --text 'model output'
model_cmd | xgc modelarmor +sanitize-response --template ...
```

## Tips

- Use for outbound safety (model -> user).
- For inbound safety (user -> model), use +sanitize-prompt.

## See Also

- [xgc-shared](../xgc-shared/SKILL.md) — Global flags and auth
- [xgc-modelarmor](../xgc-modelarmor/SKILL.md) — All filter user-generated content for safety commands
