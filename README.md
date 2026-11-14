# Argus UI

A reusable, native Windows component gallery built with **Rust + GPUI Kit**, closely following the public interface design of [Argus](https://github.com/Ray-lea/Argus).

> This is an independent UI component project. It does not include Argus source code, vendor patches, product assets, or system-management functionality.

The project packages the reference layout as reusable components: command bar, horizontal page navigation, CPU/memory/disk/network/GPU summary strip, dense process table, right-side detail rail, status bar, and component gallery. It includes light/dark themes and local synthetic fixtures.

## Quick start

Prerequisites: Windows 10/11 x64, Rust stable MSVC toolchain, Visual Studio C++ Build Tools, and Windows SDK.

```powershell
git clone https://github.com/weepwood/Argus-UI.git
cd Argus-UI
cargo run
```

```powershell
cargo generate-lockfile
cargo fmt --all
cargo check --locked
cargo test --locked
cargo build --release --locked
```

> The project pins GPUI Kit 0.8.0. GPUI currently evolves quickly; update its versions and locked transitive dependencies together.

## Features

- Independent Rust/GPUI Kit native window; no Electron or embedded WebView.
- Switchable native DWM materials (Mica, Mica Alt, Acrylic blur, transparent, solid) with translucent surface layers.
- Three text-clarity modes, including an opaque ClearType mode for the sharpest glyphs on translucent-window platforms.
- Compact command bar, horizontal navigation, and reference-aligned split pane.
- Five resource summaries with reusable line-chart metrics.
- Dense process rows with search, sorting, selection, grouping, and a device/detail rail.
- Secondary performance, services, startup, connections, history, and settings screens.
- Reusable semantic colors, toolbar controls, panels, status tags, progress bars, and chart builders.
- Windows CI and runnable release build artifact.

**Demo data is synthetic and local; no privileged services, telemetry, or real process operations are performed.**

## Layout

```text
src/
  main.rs                   # bootstrap
  app.rs                    # shell + typed route/actions
  backdrop.rs               # native window materials and OS capability detection
  design.rs                 # semantic light/dark design tokens
  typography.rs             # window-wide font stack, CJK fallbacks, tabular figures
  components.rs             # reusable visual builders
  data.rs                   # typed demo models
  pages/
    mod.rs
    overview.rs             # performance charts
    processes.rs            # process table and detail rail
    catalog.rs              # reusable secondary table screens
    showcase.rs             # component gallery
    settings.rs             # theme / density / material / text clarity controls
vendor/gpui-pre-windows/    # patched Apache-2.0 Windows text backend (see PATCHES.md)
.github/workflows/windows.yml
```

See [Architecture](docs/ARCHITECTURE.md) and [Design specification](docs/DESIGN.md) for structure and visual rules.

## Window materials

The shell uses DWM system backdrops rather than faking blur inside the UI. `src/backdrop.rs` maps five modes to `WindowBackgroundAppearance` and detects the OS build with `RtlGetVersion`:

| Mode | GPUI appearance | Minimum build |
|---|---|---|
| Solid | `Opaque` | any |
| Mica | `MicaBackdrop` | Windows 11 22H2 (22621) |
| Mica Alt | `MicaAltBackdrop` | Windows 11 22H2 (22621) |
| Acrylic blur | `Blurred` | Windows 10 1809 (17763) |
| Transparent | `Transparent` | Windows 10 1809 (17763) |

A translucent backdrop only reads as glass when the UI stays transparent too: `Palette::new` swaps solid fills for alpha-bearing surfaces (`apply_material`), the root keeps a tint, and text and data colours remain opaque. Switch modes in the Settings page; `ARGUS_UI_OPAQUE=1` forces the solid fallback. Mica follows the Windows system light/dark setting, so match the OS theme to the in-app theme for the closest result.

## Text rendering

GPUI only enables ClearType subpixel antialiasing when the window backdrop is `Opaque`. On Mica, Acrylic and transparent windows it falls back to DirectWrite grayscale antialiasing; `App::set_text_rendering_mode(Subpixel)` cannot override that guard. Grayscale glyph edges blend badly with a detailed wallpaper showing through the UI, which reads as jagged text.

Keeping the material **and** sharp grayscale glyphs requires a small fork of the Windows backend. `vendor/gpui-pre-windows/` is a copy of the upstream Apache-2.0 0.3.9 release, selected through `[patch.crates-io]`; [PATCHES.md](vendor/gpui-pre-windows/PATCHES.md) documents the changes and their licensing. Two matter here:

- Upstream re-rasterizes translucent-window text with `DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE` into a single-channel bitmap, discarding subpixel spatial detail. The local patch keeps DirectWrite's `CLEARTYPE_3x1` LCD coverage and folds the three stripes by the system pixel geometry's luminance (RGB `54/183/19`, BGR `19/183/54`, 1/256 units), returning the same single-channel mask the atlas expects.
- `alpha_correction.hlsl` applies Skia's mask gamma to dark ink on light surfaces and DirectWrite's brightness-attenuated contrast to light ink, so neither mode thickens antialiased stroke edges.

`src/typography.rs` installs the window-wide font stack through the root plugin: a probed Segoe UI face, an explicit Simplified Chinese fallback chain (on an English system locale DirectWrite otherwise resolves Chinese to Japanese Yu Gothic UI), `tnum` tabular figures, and a 1.4 line height. `Palette::text_plate` pre-composes each text-bearing surface over the opaque window colour and forces it opaque, the same technique the reference uses to stop stacked translucent fills from darkening the composite.

The Settings page offers three clarity modes (`MaterialClarity` in `src/design.rs`):

| Mode | Window | Behaviour |
|---|---|---|
| Crisp (default) | keeps the chosen material | A thicker root veil plus near-solid text plates via `Palette::text_plate`, and stronger secondary text contrast. Material tint stays visible, wallpaper detail does not reach glyph edges. |
| Glass | keeps the chosen material | Maximum see-through, closest to the reference glass look; small text may show rough edges. |
| Sharp ClearType | switches to `Opaque` | Drops the system material and forces `TextRenderingMode::Subpixel` so the platform uses ClearType. Sharpest glyphs, no glass. |

`text_plate` matters because call sites use `.opacity(k)` as a **multiplier**, not a setter: lowering the palette alpha and then multiplying again used to leave text panels at ~0.30 alpha. Only surfaces that carry text use the plate; strokes, hovers and chart tints keep their original alpha. Picking a material explicitly while in Sharp mode returns to Crisp.

Not reproduced from the reference: the variable-font `wght` axis and the line-height strut live in `gpui-pre`'s layout layer and would require a second vendored crate plus a `gpui-kit` 0.9 upgrade; per-region ClearType is deferred in the reference itself because its opaque plates hide Mica.

## Customization

Edit `src/design.rs` for colors, spacing, and typography. Replace sample records in `src/data.rs` with your own strongly typed models. Add a page in `src/pages/`, and route to it from `src/app.rs`. Do not move domain-specific state into reusable visual components.

## Relationship to Argus

The project follows [Argus](https://github.com/Ray-lea/Argus)'s public visual structure, typography notes and information hierarchy while independently implementing its components and fixtures. It contains no Argus source code or product assets: the vendored Windows backend is Zed's Apache-2.0 `gpui-pre-windows` 0.3.9 with locally written patches, and the typography algorithms come from Microsoft Terminal (MIT) and Chromium/Skia (BSD-3-Clause). Argus is licensed under AGPL-3.0-or-later; this repository's independent implementation remains MIT-licensed.

## Status

The Windows Actions workflow builds, checks, tests, and packages the native component gallery.

## License

MIT. See [LICENSE](LICENSE).
