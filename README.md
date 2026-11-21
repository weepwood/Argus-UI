# Argus UI

A reusable native **Windows and macOS** component gallery built with **Rust + GPUI Kit**, closely following the public interface design of [Argus](https://github.com/Ray-lea/Argus).

> This is an independent UI component project. It does not include Argus source code, proprietary product assets, or system-management functionality. The independently modified, Apache-2.0-licensed Windows backend is documented below.

The project packages the reference layout as reusable components: command bar, horizontal page navigation, CPU/memory/disk/network/GPU summary strip, dense process table, right-side detail rail, status bar, and component gallery. It includes light/dark themes and local synthetic fixtures.

## Downloads

Every tagged release publishes native packages for both supported desktop platforms:

| Platform | Package | Contents |
|---|---|---|
| Windows 10/11 x86_64 (AMD64) | `argus-ui-<version>-windows-x64.zip` | Portable `ArgusUI.exe`, README, LICENSE, CHANGELOG |
| macOS 12+ (Apple Silicon + Intel) | `argus-ui-<version>-macos-universal.dmg` | Universal `Argus UI.app` bundle with an Applications shortcut |

The release also contains a `SHA256SUMS.txt` file to verify both downloads. Neither app is signed with a trusted publisher certificate: Windows may show a SmartScreen prompt, and macOS requires **System Settings → Privacy & Security → Open Anyway** the first time the app launches. macOS builds are ad-hoc signed, which is the minimum arm64 requires.

## Quick start

Prerequisites:

- **Windows:** Windows 10/11 x64, Rust stable MSVC toolchain, Visual Studio C++ Build Tools, and the Windows SDK.
- **macOS:** macOS 12 or newer, Rust stable, and Xcode command line tools (`xcode-select --install`).

```powershell
git clone https://github.com/weepwood/Argus-UI.git
cd Argus-UI
cargo run
```

```powershell
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build -p argus-ui --release --locked
```

> The project pins GPUI Kit 0.8.0. GPUI currently evolves quickly; update its versions and locked transitive dependencies together. The vendored Windows text backend is applied through `[patch.crates-io]` and is simply unused on macOS.

## Features

- Independent Rust/GPUI Kit native window; no Electron or embedded WebView.
- Switchable native window materials: DWM Mica/Mica Alt/Acrylic on Windows, `NSVisualEffectView` vibrancy on macOS, plus transparent and solid fallbacks. Modes the running OS cannot paint are detected at runtime and never reach the backend.
- Three text-clarity modes, including an opaque ClearType mode for the sharpest glyphs on translucent-window platforms.
- Compact command bar, horizontal navigation, and reference-aligned split pane.
- Five resource summaries with reusable line-chart metrics.
- Three working process layouts: category groups, synthetic parent/child tree, and flat sorted list; searchable rows, selection, and a detail rail.
- Secondary performance, services, startup, connections, history, and settings screens; shared search across all demo-data tables.
- Reusable semantic colors, toolbar controls, panels, status tags, progress bars, charts, and stateless dense table builders.
- Versioned local preferences with safe startup fallback and automatic saving (no extra Rust dependencies).
- Windows and macOS CI plus a tag-driven release workflow.

**All displayed resources, process relationships, device specifications, services, network endpoints, and history entries are synthetic local fixtures.** The app performs no system-management operations, privileged actions, automatic polling, or telemetry.

## Interaction guide

- **View controls:** The command bar switches to the Processes page and selects category groups, parent/child tree, or a flat list. Tree parents are illustrative fixture IDs, not measured system relationships.
- **Search:** Filter process names, executables, categories, or PIDs; on Services, Startup, Connections, and History, search all visible columns. Multiple search terms are combined with AND. If a process is matched but its parent is filtered out, the process becomes a top-level tree row.
- **Sort:** Cycle between CPU, memory, and name order with the Sort control. In tree mode this controls sibling ordering, while keeping children under their parents.
- **Details:** Select a process to inspect its fixture values. The right pane can be hidden from the command bar or Settings.
- **Appearance:** Settings offers theme, row density, window material, text-clarity modes, and saved layout options. Preferences persist in the per-user configuration directory and are restored on launch.

The **DEMO** indicator is intentional: the scaffold is meant for UI reuse and testing, not as an actual task manager. It never launches, suspends, or terminates a process.

## Platform support

| Capability | Windows 10/11 | macOS 12+ |
|---|---|---|
| Solid window | yes | yes |
| Mica / Mica Alt (DWM) | Windows 11 22H2 (build 22621)+ | not available (falls back to solid) |
| Acrylic-style blur | Windows 10 1809 (build 17763)+ | `NSVisualEffectView` vibrancy |
| Transparent window | Windows 10 1809+ | yes |
| Subpixel text mode | ClearType via patched backend | Core Text default |
| Settings file | `%LOCALAPPDATA%\Argus-UI\settings.conf` | `~/Library/Application Support/Argus-UI/settings.conf` |

`ARGUS_UI_OPAQUE=1` forces the solid fallback on both platforms, and `ARGUS_UI_CONFIG_DIR` overrides the settings directory. A preferences file copied from another OS is validated on load, so an unsupported material degrades to a solid window instead of failing.

## Workspace structure

```text
Cargo.toml                  # workspace root + demo executable
crates/
  argus-ui-core/            # std-only: settings_store and search (portable)
  argus-ui-kit/             # GPUI 0.8: components, design, typography, backdrop
src/
  main.rs                   # demo window bootstrap / re-export adapter
  app.rs                    # demo shell and navigation state
  data.rs                   # synthetic demonstration models
  preferences.rs            # demo-specific configuration schema
  pages/                    # feature renderers and component gallery
vendor/gpui-pre-windows/    # local Apache-2.0 Windows backend patch
.github/workflows/ci.yml    # Windows + macOS + portable-core checks
.github/workflows/release.yml # tagged Windows/macOS packages
```

The demo depends on both local crates using Cargo path dependencies. To reuse them,
add the crates via a Git dependency (pin a commit), or vendor the specific crate;
**do not copy demo app state or fixture data into your library**.

```toml
[dependencies]
argus-ui-core = { git = "https://github.com/weepwood/Argus-UI", package = "argus-ui-core" }
argus-ui-kit = { git = "https://github.com/weepwood/Argus-UI", package = "argus-ui-kit" }
```

The visual crate follows this repository's pinned GPUI Kit version and builds on Windows and macOS. The core crate does not depend on GPUI at all.

See [Architecture](docs/ARCHITECTURE.md), [Starter and extension guide](docs/STARTER.md), and [Design specification](docs/DESIGN.md) for structure, portability limits, and visual rules.

## Window materials

The shell asks the OS for a real backdrop rather than faking blur inside the UI. `crates/argus-ui-kit/src/backdrop.rs` maps five modes to `WindowBackgroundAppearance`, detects the Windows build with `RtlGetVersion`, and resolves availability per platform:

| Mode | GPUI appearance | Windows | macOS |
|---|---|---|---|
| Solid | `Opaque` | any | any |
| Mica | `MicaBackdrop` | Windows 11 22H2 (22621) | unavailable |
| Mica Alt | `MicaAltBackdrop` | Windows 11 22H2 (22621) | unavailable |
| Acrylic blur | `Blurred` | Windows 10 1809 (17763) | vibrancy |
| Transparent | `Transparent` | Windows 10 1809 (17763) | any |

A translucent backdrop only reads as glass when the UI stays transparent too: `Palette::new` swaps solid fills for alpha-bearing surfaces (`apply_material`), the root keeps a tint, and text and data colours remain opaque. Switch modes in the Settings page; `ARGUS_UI_OPAQUE=1` forces the solid fallback. Mica follows the Windows system light/dark setting, so match the OS theme to the in-app theme for the closest result.

## Text rendering

GPUI only enables ClearType subpixel antialiasing when the window backdrop is `Opaque`. On Mica, Acrylic and transparent windows it falls back to DirectWrite grayscale antialiasing; `App::set_text_rendering_mode(Subpixel)` cannot override that guard. Grayscale glyph edges blend badly with a detailed wallpaper showing through the UI, which reads as jagged text.

Keeping the material **and** sharp grayscale glyphs requires a small fork of the Windows backend. `vendor/gpui-pre-windows/` is a copy of the upstream Apache-2.0 0.3.9 release, selected through `[patch.crates-io]`; [PATCHES.md](vendor/gpui-pre-windows/PATCHES.md) documents the changes and their licensing. Two matter here:

- Upstream re-rasterizes translucent-window text with `DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE` into a single-channel bitmap, discarding subpixel spatial detail. The local patch keeps DirectWrite's `CLEARTYPE_3x1` LCD coverage and folds the three stripes by the system pixel geometry's luminance (RGB `54/183/19`, BGR `19/183/54`, 1/256 units), returning the same single-channel mask the atlas expects.
- `alpha_correction.hlsl` applies Skia's mask gamma to dark ink on light surfaces and DirectWrite's brightness-attenuated contrast to light ink, so neither mode thickens antialiased stroke edges.

`crates/argus-ui-kit/src/typography.rs` installs the window-wide font stack through the root plugin: a probed Segoe UI face on Windows (San Francisco on macOS), an explicit Simplified Chinese fallback chain (Microsoft YaHei on Windows, PingFang SC on macOS — on an English Windows locale DirectWrite otherwise resolves Chinese to Japanese Yu Gothic UI), `tnum` tabular figures, and a 1.4 line height. `Palette::text_plate` pre-composes each text-bearing surface over the opaque window colour and forces it opaque, the same technique the reference uses to stop stacked translucent fills from darkening the composite.

The Settings page offers three clarity modes (`MaterialClarity` in `crates/argus-ui-kit/src/design.rs`):

| Mode | Window | Behaviour |
|---|---|---|
| Crisp (default) | keeps the chosen material | A thicker root veil plus near-solid text plates via `Palette::text_plate`, and stronger secondary text contrast. Material tint stays visible, wallpaper detail does not reach glyph edges. |
| Glass | keeps the chosen material | Maximum see-through, closest to the reference glass look; small text may show rough edges. |
| Sharp ClearType | switches to `Opaque` | Drops the system material and forces `TextRenderingMode::Subpixel` so the platform uses ClearType. Sharpest glyphs, no glass. |

`text_plate` matters because call sites use `.opacity(k)` as a **multiplier**, not a setter: lowering the palette alpha and then multiplying again used to leave text panels at ~0.30 alpha. Only surfaces that carry text use the plate; strokes, hovers and chart tints keep their original alpha. Picking a material explicitly while in Sharp mode returns to Crisp.

Not reproduced from the reference: the variable-font `wght` axis and the line-height strut live in `gpui-pre`'s layout layer and would require a second vendored crate plus a `gpui-kit` 0.9 upgrade; per-region ClearType is deferred in the reference itself because its opaque plates hide Mica. macOS always antialiases glyphs in grayscale, so the Sharp mode only changes the window material there.

## Customization

Edit `crates/argus-ui-kit/src/design.rs` for colors, spacing, and typography. Replace sample records in `src/data.rs` with your own strongly typed models. Add a page in `src/pages/`, and route to it from `src/app.rs`. Do not move domain-specific state into reusable visual components. For a new product, replace `Preferences` and the `Argus-UI` config directory with your own schema and application identifier. See [Starter and extension guide](docs/STARTER.md).

Preferences are stored as versioned plain text in the per-user configuration directory (`%LOCALAPPDATA%\Argus-UI\settings.conf` on Windows, `~/Library/Application Support/Argus-UI/settings.conf` on macOS; override the directory with `ARGUS_UI_CONFIG_DIR`). The file contains only non-sensitive appearance/layout values. Saving occurs in event handlers, not per render. Startup always falls back to defaults if the configuration is unreadable or its version unsupported.

## Releases

1. Bump `version` in `Cargo.toml` and both crate manifests, update `CHANGELOG.md`, commit.
2. Tag the merge commit: `git tag v0.2.0 && git push origin v0.2.0`.
3. `.github/workflows/release.yml` builds only a Windows x86_64 portable ZIP and a macOS universal DMG, validates the source tag and expected asset names, generates `SHA256SUMS.txt`, and publishes the GitHub release.

A manual `Release` run without a tag only builds the Windows ZIP and macOS DMG as workflow artifacts; CI does not upload loose executables. To package an existing tag, provide that tag in the manual input: the workflow checks out the tag before building. You can reproduce the same packages locally using `powershell -File scripts/package-windows.ps1 -Version <version>` on Windows or `bash scripts/package-macos.sh <version>` on macOS.

## Relationship to Argus

The project follows [Argus](https://github.com/Ray-lea/Argus)'s public visual structure, typography notes and information hierarchy while independently implementing its components and fixtures. It contains no Argus source code or product assets: the vendored Windows backend is Zed's Apache-2.0 `gpui-pre-windows` 0.3.9 with locally written patches, and the typography algorithms come from Microsoft Terminal (MIT) and Chromium/Skia (BSD-3-Clause). Argus is licensed under AGPL-3.0-or-later; this repository's independent implementation remains MIT-licensed.

## Status

`Native desktop CI` checks, lints, and tests the workspace on Windows and macOS and runs the portable core library on Linux. `Release` packages Windows x64 and macOS universal builds from version tags.

## License

MIT. See [LICENSE](LICENSE).
