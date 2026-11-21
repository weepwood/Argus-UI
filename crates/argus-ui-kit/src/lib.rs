//! Cross-platform visual component library for GPUI Kit.
//! All domain data, application routes and user actions belong to the host app.
//!
//! Windows uses DWM system materials (Mica, Mica Alt, Acrylic) and DirectWrite
//! text; macOS uses `NSVisualEffectView` vibrancy with Core Text. Availability
//! is resolved at runtime by `backdrop`, so a shared preferences file stays
//! portable between the two platforms.

pub mod backdrop;
pub mod components;
pub mod design;
pub mod typography;
