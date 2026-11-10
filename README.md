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
- Mica-backed window, compact command bar, horizontal navigation, and reference-aligned split pane.
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
  design.rs                 # semantic light/dark design tokens
  components.rs             # reusable visual builders
  data.rs                   # typed demo models
  pages/
    mod.rs
    overview.rs             # performance charts
    processes.rs            # process table and detail rail
    catalog.rs              # reusable secondary table screens
    showcase.rs             # component gallery
    settings.rs             # theme / density controls
.github/workflows/windows.yml
```

See [Architecture](docs/ARCHITECTURE.md) and [Design specification](docs/DESIGN.md) for structure and visual rules.

## Customization

Edit `src/design.rs` for colors, spacing, and typography. Replace sample records in `src/data.rs` with your own strongly typed models. Add a page in `src/pages/`, and route to it from `src/app.rs`. Do not move domain-specific state into reusable visual components.

## Relationship to Argus

The project follows [Argus](https://github.com/Ray-lea/Argus)'s public visual structure and information hierarchy while independently implementing its components and fixtures. It contains no Argus source code or product assets. Argus is licensed under AGPL-3.0-or-later; this repository's independent implementation remains MIT-licensed.

## Status

The Windows Actions workflow builds, checks, tests, and packages the native component gallery.

## License

MIT. See [LICENSE](LICENSE).
