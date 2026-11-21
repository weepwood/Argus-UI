// Hide the console window in release Windows builds. The attribute is ignored
// by other targets, but it is still scoped so macOS builds stay untouched.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod data;
mod pages;
mod preferences;

// Compatibility aliases keep the demo's feature paths stable while exporting
// the actual reusable components and utilities from dedicated library crates.
pub use argus_ui_core::{search, settings_store};
pub use argus_ui_kit::{backdrop, components, design, typography};

use gpui_kit::assets::Assets;
use gpui_kit::*;

fn main() {
    let (preferences, store, config_warning) = preferences::Preferences::load_user();
    gpui_kit::application().with_assets(Assets).run(move |cx| {
        gpui_kit::init(cx);
        backdrop::install_surface(cx);
        let opts = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1600.), px(1000.)), cx)),
            window_min_size: Some(size(px(1150.), px(740.))),
            window_background: preferences.resolved_backdrop().appearance(),
            ..Default::default()
        };
        gpui_kit::open_window(opts, cx, move |window, cx| {
            window.set_window_title("Argus UI · Component Library");
            cx.new(|cx| app::Shell::new(preferences, store, config_warning, window, cx))
        })
        .expect("无法创建 Argus UI 窗口");
    });
}
