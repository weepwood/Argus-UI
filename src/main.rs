#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Argus UI's first release targets Windows only. Build with the MSVC Windows toolchain.");

mod app;
mod components;
mod data;
mod design;
mod pages;

use gpui_kit::*;
use gpui_kit::assets::Assets;

fn main() {
    gpui_kit::application().with_assets(Assets).run(|cx| {
        gpui_kit::init(cx);
        let opts = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1240.), px(810.)), cx)),
            window_min_size: Some(size(px(880.), px(590.))),
            ..Default::default()
        };
        gpui_kit::open_window(opts, cx, |window, cx| {
            window.set_window_title("Argus UI · Native Starter");
            cx.new(|cx| app::Shell::new(window, cx))
        })
        .expect("无法创建 Argus UI 窗口");
    });
}
