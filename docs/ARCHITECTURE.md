# Architecture and extension guide

Argus UI is a Rust + GPUI Kit component gallery for a dense Windows desktop interface. Its application shell and component functions demonstrate how to reuse the Argus-inspired visual system without depending on Argus product code or host-system behavior.

## Rendering flow

```text
main.rs
  preferences::load_user() → SettingsStore
  gpui_kit::application()
    -> open_window()
       -> app::Shell (Render)
          ├─ command bar
          ├─ horizontal page tabs
          ├─ five-resource summary strip
          ├─ pages/*
          │  ├─ processes + detail rail
          │  ├─ performance charts
          │  ├─ secondary data tables
          │  ├─ settings
          │  └─ component gallery
          └─ status bar
```

- **`app.rs`** owns global appearance, navigation, search, view mode, and shell controls.
- **`pages/`** composes the reusable controls into process, performance, data-table, settings, and component-gallery screens.
- **`crates/argus-ui-kit/src/components.rs`** contains stateless visual primitives. Components receive semantic palette values and leave domain state and actions to the caller.
- **`crates/argus-ui-kit/src/design.rs`** defines appearance modes, row density, and semantic light/dark colors.
- **`preferences.rs`** owns typed app settings, format version, safe fallbacks, and Windows material resolution.
- **`crates/argus-ui-core/src/settings_store.rs`** is a dependency-free file store, separate from GPUI and domain data.
- **`crates/argus-ui-core/src/search.rs`** provides pure keyword matching shared by demo feature tables.
- **`data.rs`** provides deterministic local fixtures and pure filtering/sorting functions.
- **`main.rs`** loads user preferences, configures the native window and initial backdrop, and opens the application shell.

## Adding a screen

1. Add the route to `app::Page` and its visible title.
2. Create a renderer in `src/pages/` that composes the existing shell primitives.
3. Keep screen-specific fixture data and interactions out of `components.rs`.
4. Add a reusable component only when it has a stable visual role across screens.

## Entity boundaries

| Case | Recommended pattern |
|---|---|
| Labels, tags, buttons, summary cells | Stateless element or builder |
| Search input and other editable controls | GPUI entity owned by the shell or page |
| Interactive data tables | Page state plus `dense_table_header`, `dense_table_row`, and empty-state builders |
| Local configuration | Versioned app-specific preferences over a reusable filesystem store |
| Continuous or high-frequency charts | GPUI Kit charts first; a dedicated render layer only after measuring a need |

The Cargo workspace contains three packages: a Windows-only executable `argus-ui`, a portable `argus-ui-core` library, and a Windows-first `argus-ui-kit` library. The libraries expose public Rust modules and can be imported from another crate instead of copying page code.

Read [Starter and extension guide](STARTER.md) for module boundaries and an example of adding a data table or preference.

The demo uses synthetic data and does not access process, network, or hardware APIs. Replace fixtures at the page boundary when connecting another application; keep operating-system capabilities out of generic visual components.
