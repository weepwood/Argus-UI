# Argus UI design system

Argus UI is a reusable GPUI Kit component gallery based on the public interface of [Argus](https://github.com/Ray-lea/Argus). The implementation, fixtures, and component APIs in this repository are independently authored; Argus source code, product icons, and system-management behavior are not included.

## Interface structure

The desktop shell follows the same information hierarchy across its pages:

1. A compact command bar with the app identity, process-view selector, search, and common controls.
2. A horizontal page strip with a single accent underline for the active page.
3. A five-column resource band for CPU, memory, disk, network, and GPU.
4. A dense work area with a table or chart surface and an optional right detail rail.
5. A narrow status bar for refresh state and local demo status.

## Visual rules

- Use a native DWM backdrop (Mica by default, with Mica Alt, Acrylic blur, transparent and solid as alternatives) and restrained translucent surfaces over it. The root layer must stay translucent for the material to show through; see `src/backdrop.rs` and `Palette::apply_material`.
- Keep text on an opaque plate. GPUI disables ClearType on translucent windows, so grayscale glyph edges must not blend into wallpaper detail leaking through a panel. `Palette::text_plate` pre-composes the layer over the opaque window colour and forces it opaque in Crisp mode; Glass mode and the Sharp/ClearType mode are explicit opt-outs.
- Preserve the reference's compact information density, thin separators, flat surfaces, and modest 5–8px corner radii.
- Use a probed Segoe UI face with an explicit Simplified Chinese fallback chain, 10–13px labels, restrained medium weights, `tnum` tabular figures and a 1.4 line height (`src/typography.rs`). Keep secondary labels high-contrast in translucent modes, where grayscale antialiasing thins light strokes.
- Rasterize glyphs from DirectWrite's LCD coverage folded by stripe luminance rather than a fresh grayscale pass; apply Skia mask gamma to dark ink and brightness-attenuated contrast to light ink. See `vendor/gpui-pre-windows/PATCHES.md`.
- Give each resource its own color: CPU cyan, memory violet, disk amber, network green, GPU pink. Keep navigation and selection on the system blue accent.
- Make selection, hover, muted text, and chart colors semantic roles so both appearance modes share the same components.
- Keep chart lines and resource labels readable at both wide and minimum window sizes.

## Reusable components

| Component | Source | Role |
|---|---|---|
| Toolbar button | `components::toolbar_button` | Compact icon-and-label controls with selected and hover states |
| Panel | `components::panel` | Thin outlined surface used for tables, charts, and settings |
| Tag | `components::tag` | Small count, status, and metadata label |
| Progress | `components::progress` | Semantic resource bar |
| Sparkline | `components::sparkline` | Lightweight GPUI Kit `LineChart` with hidden axes |
| Resource metric | `components::resource_metric` | Summary strip segment with value, note, and line chart |
| Shell | `app::Shell` | Shared command bar, page tabs, metric strip, and status bar |

## Semantic colors

The complete light and dark palettes are defined in `src/design.rs`.

| Role | Light | Dark |
|---|---|---|
| Window | `#EEF3F6` | `#1F2225` |
| Surface | `#F8FAFB` | `#25292C` |
| Text | `#242A32` | `#F1F3F5` |
| Accent | `#168BD2` | `#50C8F1` |
| CPU | `#159ED1` | `#43C9ED` |
| Memory | `#755BE5` | `#A393FF` |
| Disk | `#D78A00` | `#F0B64C` |
| Network | `#1CA65B` | `#56D39A` |
| GPU | `#E13D79` | `#F277A9` |

The hex values above are the solid-mode palette. With a translucent backdrop, `Palette::apply_material` replaces the window and card fills with layered alpha surfaces, keeps a faint root tint, and uses low-alpha ink for strokes and hover. Text, accent, and resource colours stay opaque so dense tables keep their contrast. Solid mode remains the fallback on builds or platforms that cannot paint the requested material (for example Mica on macOS).

## Scope

- All process and device values are local synthetic fixtures.
- The demo includes no process operations, privileged services, drivers, or host telemetry.
- The reference is Argus's public visual design; this repository keeps its own implementation and MIT license.
