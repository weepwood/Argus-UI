# Argus UI

A reusable, native Windows desktop UI starter built with **Rust + GPUI Kit**, with a restrained Fluent-inspired visual language based on the public design references of [Argus](https://github.com/Ray-lea/Argus).

> This is an independent scaffold, **not** a fork of Argus, and contains no copied Argus source code, vendor patches, branding assets, or system-management functionality.

The scaffold includes a desktop shell, compact sidebar, navigation, overview, sample process table, component gallery, settings, light/dark themes, and a Windows GitHub Actions build.

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
- Fluent-like density, 1px outlines, small radii, soft layered surfaces, Segoe typography.
- Collapsible sidebar, four navigable pages, theme and density controls.
- Showcase for buttons, badges, panels, progress meters, and inline toolbars.
- A mock process list with search, CPU/memory sorting, row selection, and detail panel.
- Reusable design tokens and simple stat/chart visuals.
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
    overview.rs             # metrics / mini visualization
    processes.rs            # interactive demo table
    showcase.rs             # component catalogue
    settings.rs             # theme / density controls
.github/workflows/windows.yml
```

## Customization

Edit `src/design.rs` for colors, spacing, and typography. Replace sample records in `src/data.rs` with your own strongly typed models. Add a page in `src/pages/`, and route to it from `src/app.rs`. Do not move domain-specific state into reusable visual components.

## Relationship to Argus

The project borrows **design ideas** (Fluent-inspired surfaces, restrained accent, stable numerics, information hierarchy), not implementation code. [Argus](https://github.com/Ray-lea/Argus) is separately maintained under AGPL-3.0-or-later; consult its license before copying any code or resources. This independently authored scaffold is MIT-licensed.

## Status

Initial scaffold. The included GitHub Actions workflow is the authoritative Windows build check; no claim is made about runtime screenshots until tested on a Windows desktop.

## License

MIT. See [LICENSE](LICENSE).
