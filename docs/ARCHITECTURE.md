# Architecture and extension guide

Argus UI is a Rust + GPUI Kit component gallery for a dense Windows desktop interface. Its application shell and component functions demonstrate how to reuse the Argus-inspired visual system without depending on Argus product code or host-system behavior.

## Rendering flow

```text
main.rs
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
- **`components.rs`** contains stateless visual primitives. Components receive semantic palette values and leave domain state and actions to the caller.
- **`design.rs`** defines appearance modes, row density, and semantic light/dark colors.
- **`data.rs`** provides deterministic local fixtures and pure filtering/sorting functions.
- **`main.rs`** configures the native window, Mica backdrop, and application startup.

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
| Interactive data tables | Page state plus stateless row builders |
| Continuous or high-frequency charts | GPUI Kit charts first; a dedicated render layer only after measuring a need |

The demo uses synthetic data and does not access process, network, or hardware APIs. Replace fixtures at the page boundary when connecting another application; keep operating-system capabilities out of generic visual components.
