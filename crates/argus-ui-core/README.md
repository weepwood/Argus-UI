# argus-ui-core

A dependency-free Rust library for native desktop applications. It can be
built and tested on Windows, macOS, and Linux without GPUI.

## Add it to another project

```toml
[dependencies]
argus-ui-core = { git = "https://github.com/weepwood/Argus-UI", package = "argus-ui-core" }
```

For reproducible builds, pin a Git `rev` to an audited commit.

## API

- `argus_ui_core::search::matches_fields(query, fields)` — case-insensitive
  AND matching across a row's visible text columns.
- `argus_ui_core::settings_store::SettingsStore` — UTF-8 text file read and
  replacement-safe write. Callers own encoding/versioning and all sensitive data.
- `SettingsStore::user_path(app_id, override_env)` — optional per-user config
  location: `%LOCALAPPDATA%` on Windows, `~/Library/Application Support`
  on macOS, `XDG_CONFIG_HOME` / `~/.config` on Linux.

Use a distinct app ID and a distinct override variable in each product.
The store does not encrypt files and must not store credentials.

```sh
cargo test -p argus-ui-core --locked
cargo run -p argus-ui-core --example consumer --locked
```
