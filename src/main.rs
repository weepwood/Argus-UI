#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![recursion_limit = "512"]

#[cfg(not(target_os = "windows"))]
compile_error!(
    "Argus UI's first release targets Windows only. Build with the MSVC Windows toolchain."
);

mod app;
mod backdrop;
mod components;
mod data;
mod design;
mod pages;
mod typography;

use gpui_kit::assets::Assets;
use gpui_kit::*;

fn main() {
    gpui_kit::application().with_assets(Assets).run(|cx| {
        gpui_kit::init(cx);
        backdrop::install_surface(cx);
        let opts = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1600.), px(1000.)), cx)),
            window_min_size: Some(size(px(1150.), px(740.))),
            window_background: backdrop::Backdrop::auto().appearance(),
            ..Default::default()
        };
        gpui_kit::open_window(opts, cx, |window, cx| {
            window.set_window_title("Argus UI · Component Library");
            cx.new(|cx| app::Shell::new(window, cx))
        })
        .expect("无法创建 Argus UI 窗口");
    });
}
