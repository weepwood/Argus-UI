//! Typography rules shared by every surface in the window.
//!
//! GPUI's `RootPlugin::style` is the one place whose text style cascades to
//! pages, dialogs and floating layers alike. Without an explicit CJK fallback
//! DirectWrite resolves Chinese glyphs to the Japanese "Yu Gothic UI" face on
//! an English system locale, and proportional digits make the process readings
//! shift horizontally every time they refresh. macOS uses the same structure
//! with PingFang SC and the San Francisco system face.

use std::sync::{Arc, OnceLock};

use gpui_kit::{
    App, Div, FontFallbacks, FontFeatures, SharedString, Stateful, Styled as _, font, relative,
};

/// Chinese first, then Latin faces. The order matters: the platform text
/// engine walks this list for glyphs the primary family does not contain.
#[cfg(target_os = "windows")]
const FALLBACK_FONTS: [&str; 4] = [
    "Microsoft YaHei UI",
    "Microsoft YaHei",
    "Segoe UI",
    "Arial Unicode MS",
];

/// macOS ships PingFang SC; Hiragino Sans GB covers older installs.
#[cfg(target_os = "macos")]
const FALLBACK_FONTS: [&str; 4] = [
    "PingFang SC",
    "Hiragino Sans GB",
    "Helvetica Neue",
    "Arial Unicode MS",
];

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const FALLBACK_FONTS: [&str; 1] = [".SystemUIFont"];

/// Windows 11's variable UI face, with a Win10-safe fallback.
#[cfg(target_os = "windows")]
const UI_FONTS: [&str; 3] = ["Segoe UI Variable", "Segoe UI", ".SystemUIFont"];

/// `.SystemUIFont` resolves to San Francisco in GPUI's macOS backend.
#[cfg(target_os = "macos")]
const UI_FONTS: [&str; 2] = [".SystemUIFont", "Helvetica Neue"];

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const UI_FONTS: [&str; 1] = [".SystemUIFont"];

pub fn fallbacks() -> FontFallbacks {
    FontFallbacks::from_fonts(FALLBACK_FONTS.iter().map(|name| (*name).into()).collect())
}

/// Tabular figures (`tnum`) keep numeric columns a fixed width.
pub fn tabular() -> FontFeatures {
    static FEATURES: OnceLock<FontFeatures> = OnceLock::new();
    FEATURES
        .get_or_init(|| FontFeatures(Arc::new(vec![("tnum".into(), 1)])))
        .clone()
}

/// Resolve the first font that actually exists on this machine. Specifying a
/// missing family would silently fall back to the platform default anyway, but
/// probing keeps the choice explicit and lets the CJK chain stay in our hands.
fn installed_ui_font(cx: &App) -> SharedString {
    static FONT: OnceLock<SharedString> = OnceLock::new();
    FONT.get_or_init(|| {
        let system = cx.text_system();
        for name in UI_FONTS {
            let id = system.resolve_font(&font(name));
            if let Some(resolved) = system.get_font_for_id(id)
                && resolved.family.as_ref() == name
            {
                return name.into();
            }
        }
        ".SystemUIFont".into()
    })
    .clone()
}

/// Apply the shared text style to the window root.
pub fn apply(surface: &mut Stateful<Div>, cx: &mut App) {
    let style = surface.text_style();
    style.font_family = Some(installed_ui_font(cx));
    style.font_fallbacks = Some(fallbacks());
    style.font_features = Some(tabular());
    // One body line height everywhere, matching the reference's 1.4 ratio.
    style.line_height = Some(relative(1.4));
}
