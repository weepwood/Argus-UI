# Local patches to gpui-pre-windows 0.3.9

This directory is a copy of the crates.io release of `gpui-pre-windows`
0.3.9 (Zed's `gpui_windows` snapshot), licensed Apache-2.0. The upstream
`LICENSE-APACHE` file is retained verbatim. `Cargo.toml` selects it through
`[patch.crates-io]` in the workspace root.

## Why it exists

GPUI disables ClearType when a window's backdrop is not `Opaque`
(`gpui-pre/src/window.rs`, `should_use_subpixel_rendering`). Upstream 0.3.9 then
asks DirectWrite for `DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE` and rasterizes into
a single-channel `DWRITE_TEXTURE_ALIASED_1x1` bitmap. That discards the
subpixel spatial detail of the LCD coverage, which is what makes small text on
a Mica/Acrylic window look thin and jagged.

The reference implementation (Argus, `docs/design/window.md` and
`docs/architecture/frontend.md`) keeps the LCD coverage and folds it into a
grayscale mask instead. This patch reproduces that behaviour for 0.3.9.

## Changes

| File | Change |
|---|---|
| `src/direct_write.rs` | `DirectWriteComponents` gains `grayscale_weights: [u16; 3]`, derived from `IDWriteRenderingParams::GetPixelGeometry()` at startup: RGB `[54, 183, 19]`, BGR `[19, 183, 54]`, flat/unknown `[37, 182, 37]` (1/256 units, sum 256). |
| `src/direct_write.rs` | `create_glyph_run_analysis` always requests `DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE`; `raster_bounds` always uses `DWRITE_TEXTURE_CLEARTYPE_3x1`. |
| `src/direct_write.rs` | `rasterize_monochrome` on a translucent window samples the 3x1 LCD texture and folds each triplet by the luminance weights, returning a single-channel mask (same shape the atlas expects). The `subpixel_rendering` path is unchanged. |
| `src/alpha_correction.hlsl` | Dark ink (foreground brightness below 50%) uses Skia's mask gamma model instead of DirectWrite's contrast boost, which otherwise thickens antialiased edges on light surfaces. Light ink keeps the existing brightness-attenuated correction. |
| `build.rs` | Track `src/alpha_correction.hlsl` as a build input. Debug builds compile HLSL from source at runtime; release builds precompile with `fxc`, and the header is an `#include` of `shaders.hlsl`. |

## Licensing

The reference project is AGPL-3.0 while this workspace is MIT, so **no source
code from that project is copied here**. The behaviour is reimplemented from
its published design notes and from the upstream algorithms those notes cite:

- `src/alpha_correction.hlsl` is derived from Microsoft Terminal's
  `dwrite.hlsl` (MIT), already present in upstream 0.3.9.
- The Skia mask gamma model comes from Chromium/Skia (BSD-3-Clause).
- The LCD luminance weights are standard RGB stripe-order luminance constants.

Upstream Apache-2.0 continues to cover the unmodified parts of this crate.

## Not reproduced

- **Variable-font `wght` axis and line-height strut.** Both live in
  `gpui-pre` / the text-system layout layer, not in this Windows backend.
  Adding them would require a second vendored crate and an API bump to
  `gpui-kit` 0.9.x, which would break the rest of this starter.
- **Per-region ClearType.** The reference project explicitly deferred this
  (`docs/design/window.md`, "按区域使用 ClearType（暂缓）") because the opaque
  plates needed for it hide the Mica backdrop.
