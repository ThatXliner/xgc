# xgc

`xgc` is the executable package for
[xliner’s GWS-CLI](https://github.com/ThatXliner/xgc), an independently
maintained fork of
[Google Workspace CLI](https://github.com/googleworkspace/cli).

This fork renames the command to `xgc`, stores configuration under
`~/.config/xgc`, uses fork-owned `XGC_*` environment variables, and adds named
authentication profiles.

```bash
cargo install --git https://github.com/ThatXliner/xgc --package xgc --locked
xgc auth bootstrap
xgc auth login --profile personal
xgc --profile personal drive files list
```

There are currently no npm, Homebrew, or Nix packages for this fork.

See the [fork README](https://github.com/ThatXliner/xgc#readme) for profile and
migration details. For the inherited command surface and broader usage guide,
see the
[upstream README](https://github.com/googleworkspace/cli/blob/main/README.md).

Apache-2.0. This is not an officially supported Google product.
