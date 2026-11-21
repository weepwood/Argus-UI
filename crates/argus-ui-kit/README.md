# argus-ui-kit

A cross-platform GPUI Kit 0.8 visual library extracted from the Argus-UI
desktop demo. It exposes stateless components, semantic design tokens,
typography helpers and native window materials (DWM Mica/Acrylic on Windows,
`NSVisualEffectView` vibrancy on macOS).

## In another GPUI desktop project

```toml
[dependencies]
gpui-kit = "=0.8.0"
argus-ui-kit = { git = "https://github.com/weepwood/Argus-UI", package = "argus-ui-kit" }
```

Pin the dependency to an audited Git revision for reproducible builds.

```rust
use argus_ui_kit::{
    backdrop::Backdrop,
    components,
    design::{Appearance, MaterialClarity, Palette},
};
use gpui_kit::*;

fn example_panel() -> Div {
    let colors = Palette::new(
        Appearance::Dark,
        Backdrop::Opaque,
        MaterialClarity::Sharp,
    );
    components::panel(colors)
        .p(px(16.))
        .child(components::heading("Dashboard", colors))
        .child(components::tag("Ready", colors, true))
}
```

Components are presentation-only: they don't fetch system data, manage routes,
spawn processes or store user credentials. Your app supplies the data and
attaches GPUI callbacks. `dense_table_row` requires a stable `ElementId`.

## Compatibility

This crate targets Windows because its backdrop integration uses DWM and the
Windows native backend. It currently requires `gpui-kit = 0.8.0`.
The original demo repository has an independently maintained patch for
`gpui-pre-windows` text rasterization in `vendor/`. **Cargo's patches in a
Git dependency are not inherited by consuming projects**. A consumer should
explicitly decide whether it needs that patch in its own root `Cargo.toml`,
or use the upstream backend and accept potential visual differences.

The crate is not a promise of ABI/API stability or cross-platform window
compatibility. Start with `Backdrop::Opaque` when integrating it elsewhere.
