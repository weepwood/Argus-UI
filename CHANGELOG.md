# Changelog

All notable changes to Argus UI are documented here. The project follows
[Semantic Versioning](https://semver.org/) and every entry ships as a GitHub
release with Windows x64 and macOS universal packages.

## [0.2.0] - 2026-11-21

First packaged release for Windows and macOS.

### Added

- Cargo Workspace split into two reusable libraries:
  - `argus-ui-core`: standard-library-only multi-keyword search and a
    crash-safe settings store with Windows, macOS, and XDG config paths.
  - `argus-ui-kit`: GPUI Kit design tokens, visual builders, typography, and
    native window materials.
- macOS support for the demo app: universal `Argus UI.app` bundle, vibrancy and
  transparent window materials, Core Text-appropriate font stack (PingFang SC
  fallback), and an ad-hoc signed `.app`/`.dmg` in every release.
- Per-platform material availability: unsupported modes (Mica on macOS) are
  detected at runtime and a cross-platform settings file degrades to a solid
  window instead of reaching a backend that cannot paint it.
- `Native desktop CI` workflow covering Windows x64, macOS, and the portable
  core library on Linux.
- `Release` workflow that builds Windows x64 and macOS universal packages,
  verifies the tag against the manifest version, and publishes the GitHub
  release.
- Versioned preference persistence with safe startup fallback, automatic saving,
  and a visible configuration-file location in Settings.
- Shared dense table header/row/empty-state builders reused by the services,
  startup, connections, and history pages.

### Changed

- Process views now really switch between category groups, a synthetic
  parent/child tree, and a flat sorted list; filtered children are promoted to
  top-level rows so no record disappears.
- Cross-page search matches every visible column with case-insensitive,
  multi-term AND semantics through the shared `argus-ui-core` matcher.
- Settings copy, material tags, and antialiasing notes describe the actual
  platform instead of assuming Windows.
- CI verifies committed formatting instead of rewriting sources, and the lock
  file is treated as authoritative.
- Removed non-functional controls ("run new task", "refresh every second",
  elevation lock); every reading is clearly labelled as synthetic demo data.

### Fixed

- Settings writes use a sibling temporary file plus a rename, so a failed save
  can no longer truncate the existing configuration.
- Application identifiers are validated before joining the config directory,
  preventing path traversal out of the per-user settings folder.
- Windows text on translucent windows keeps DirectWrite LCD coverage (vendored
  backend patch) instead of collapsing to a single-channel grayscale mask.
