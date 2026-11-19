# Reusing Argus-UI as a GPUI desktop starter

Argus-UI is a **Windows-first Rust + GPUI Kit 0.8** desktop starter. It
demonstrates architecture and reusable visual elements; it is **not** a process
manager or a platform-neutral GPUI template yet. The demo's process, hardware,
connection, and history values are synthetic.

## Dependency boundaries

| Layer | Location | Reuse strategy |
| --- | --- | --- |
| Presentation tokens | `src/design.rs` | Keep semantic colors and density controls; substitute your product palette |
| Native window materials | `src/backdrop.rs`, `src/typography.rs` | Optional Windows-only integration; select opaque mode if a material isn't supported |
| Visual primitives | `src/components.rs` | Copy stateless builders into a shared `ui` module or crate |
| Settings file storage | `src/foundation/settings_store.rs` | Pure standard library; reusable without GPUI |
| Typed preferences | `src/preferences.rs` | Replace with your application's settings schema and version |
| Application state | `src/app.rs` | Replace demo-specific navigation and state; retain clear mutation methods |
| Feature pages | `src/pages/` | Compose primitive components; own queries, selection and actions here |
| Demo data | `src/data.rs` | Replace wholesale with domain types or repositories |
| Search | `src/search.rs` | Pure matching logic; keep it independent from the renderer |

The native Windows renderer is a locally patched version of an Apache-2.0
GPUI backend. Its notes and provenance are in
[`vendor/gpui-pre-windows/PATCHES.md`](../vendor/gpui-pre-windows/PATCHES.md).
Do not copy the backend without reviewing its license and maintenance costs.

## First steps for a new app

1. Change the `Cargo.toml` package name and `main.rs` window title.
2. Keep `components.rs`, `design.rs`, and `foundation/` as application-neutral modules.
3. Add a typed feature model and a page module under `src/pages/`.
4. Add a route to `app::Page` and register it in the shell's `Render` match.
5. Replace sample fixtures and the resource strip with your product's information.
6. Replace the Argus-specific `Preferences` keys and the per-user directory
   `Argus-UI` in `SettingsStore::user_path()`.
7. Preserve the explicit `DEMO` label until all data shown is backed by real
   sources. Do not keep simulated metrics labeled as live values.

## Example: adding a dense table

Use `components::dense_table_header`, `dense_table_row`, and
`table_empty_state` when a feature needs a compact, equal-width table.
The table builders do not import your product types or manage state:

```rust
let palette = shell.palette();
let header = components::dense_table_header(["Name", "Status", "Owner"], palette);
let row = components::dense_table_row(
    "task-42",                    // stable view identity
    ["Indexing", "Active", "You"], // replace with your own typed model
    palette,
    shell.density.row_height(),
    false,                        // selected state is owned by the page
);
```

Create a distinct ID per row. If results are filtered or sorted, IDs must come
from stable record keys rather than the filtered row's current position. A page
may attach its own `on_click` or other event handler to the returned row.

For tables with heavy datasets, variable row heights, or many columns, introduce
virtualization and a separate column model **before** scaling this simple
renderer. The included table is intentionally small and easy to customize.

## Example: typed, persistent settings

`SettingsStore` only reads and writes text. `Preferences` owns encoding and
parsing. This separation makes the storage reusable and the preferences
application-specific.

The demo stores versioned `key=value` text at:

`%LOCALAPPDATA%\\Argus-UI\\settings.conf`

Use `ARGUS_UI_CONFIG_DIR` to redirect it, for example for a portable build
or integration test. `ARGUS_UI_OPAQUE=1` forces a solid window regardless of
stored materials. Settings include theme, density, backdrop, text clarity,
details-panel visibility, and animation preference.

File saving writes to a unique sibling temporary file, syncs it, then replaces
the target via rename. If reading fails or the version is not supported, the
application starts with defaults. A failed save is reported in the shell; it
does not crash the UI. Never store credentials, OAuth tokens, or secrets in
this plain-text settings file.

When adding a preference:

1. Add its typed field and default to `Preferences`.
2. Update `parse`, `encode`, and the round-trip test.
3. Add a controlled state mutation method in `Shell`.
4. Call `persist_preferences()` from that method, not from `Render`.
5. If the format changes incompatibly, increment `version` and implement
   a migration or documented fallback before shipping.

## Validation and current limits

Use the existing Windows CI for `cargo fmt --all -- --check`,
`cargo check --locked`, `cargo clippy --all-targets --locked -- -D warnings`,
and `cargo test --locked`. Window behavior, DPI scaling, contrast, backdrop
availability, keyboard input, and GPU memory usage still require a real
Windows 10/11 desktop session.

Not yet included: application-wide keyboard command routing, focus management,
virtualized multi-thousand-row tables, auto-updates, crash reporting, or
cross-platform builds. Those should be separate, testable stages rather than
being silently implied by the demo.
