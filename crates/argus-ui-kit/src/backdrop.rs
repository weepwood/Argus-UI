//! Native window materials for the shell.
//!
//! DWM paints the backdrop behind the window, so a translucent mode only reads
//! as glass when the UI keeps its own root layer transparent. The matching
//! surface colours live in `design.rs`.

use std::sync::OnceLock;

// Explicit imports only: `gpui_kit::*` also pulls in the library's `#[test]`
// attribute macro, which collides with the standard one during test expansion.
use gpui_kit::{
    App, Context, Div, IntoElement, Refineable as _, Render, Stateful, StyleRefinement,
    Styled as _, Window, WindowBackgroundAppearance,
    base::{Root, RootPlugin},
    div, transparent_black,
};

/// The material drawn behind the GPUI scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backdrop {
    /// Solid theme colour. Always available.
    Opaque,
    /// Desktop-tinted Mica, Windows 11 22H2 and later.
    Mica,
    /// Stronger desktop tint, Windows 11 22H2 and later.
    MicaAlt,
    /// Acrylic blur of whatever is behind the window, Windows 10 1809+.
    Acrylic,
    /// No system material: only the UI's translucent layers remain.
    Clear,
}

impl Backdrop {
    pub const ALL: [Self; 5] = [
        Self::Opaque,
        Self::Mica,
        Self::MicaAlt,
        Self::Acrylic,
        Self::Clear,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Mica => "mica",
            Self::MicaAlt => "mica-alt",
            Self::Acrylic => "acrylic",
            Self::Clear => "clear",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Opaque => "不透明",
            Self::Mica => "云母 Mica",
            Self::MicaAlt => "云母 Alt",
            Self::Acrylic => "亚克力毛玻璃",
            Self::Clear => "完全透明",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Opaque => "纯色实底，兼容所有 Windows 版本，可读性最好。",
            Self::Mica => "桌面壁纸色调的柔和半透明材质，内容层保持通透。",
            Self::MicaAlt => "更强的桌面着色，适合标签式与分栏布局。",
            Self::Acrylic => "窗口后方内容实时模糊的亚克力玻璃，层次最明显。",
            Self::Clear => "不启用系统背板，仅保留半透明内容层。",
        }
    }

    pub fn appearance(self) -> WindowBackgroundAppearance {
        match self {
            Self::Opaque => WindowBackgroundAppearance::Opaque,
            Self::Mica => WindowBackgroundAppearance::MicaBackdrop,
            Self::MicaAlt => WindowBackgroundAppearance::MicaAltBackdrop,
            Self::Acrylic => WindowBackgroundAppearance::Blurred,
            Self::Clear => WindowBackgroundAppearance::Transparent,
        }
    }

    pub fn is_translucent(self) -> bool {
        !matches!(self, Self::Opaque)
    }

    /// GPUI itself guards the DWM calls by OS build; keep the UI in sync with it.
    pub fn is_available(self) -> bool {
        match self {
            Self::Opaque => true,
            Self::Mica | Self::MicaAlt => windows_build() >= 22_621,
            Self::Acrylic | Self::Clear => windows_build() >= 17_763,
        }
    }

    /// Pick Mica on a capable system, otherwise fall back to a solid surface.
    /// `ARGUS_UI_OPAQUE=1` forces the fallback (remote sessions, screenshots, etc.).
    pub fn auto() -> Self {
        let forced = std::env::var_os("ARGUS_UI_OPAQUE").is_some();
        if !forced && Self::Mica.is_available() {
            Self::Mica
        } else {
            Self::Opaque
        }
    }
}

/// The component library paints an opaque theme colour onto the GPUI root by
/// default, which would hide any DWM material. Register a root plugin that
/// clears that fill; the shell's own root `div()` then owns the window colour
/// and keeps it translucent when a backdrop is active.
pub struct TranslucentSurface;

impl Render for TranslucentSurface {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl RootPlugin for TranslucentSurface {
    fn style(&self, surface: &mut Stateful<Div>, _window: &mut Window, cx: &mut App) {
        surface
            .style()
            .refine(&StyleRefinement::default().bg(transparent_black()));
        // The root is also the only style hook that cascades to every page and
        // floating layer, so the shared font stack lives here.
        crate::typography::apply(surface, cx);
    }
}

pub fn install_surface(cx: &mut App) {
    Root::register_plugin(cx, |_, _| TranslucentSurface);
}

/// Short line for the settings page: which materials this machine can use.
pub fn support_note() -> String {
    let build = windows_build();
    if build == 0 {
        "无法读取 Windows 版本，已回退到不透明材质".to_string()
    } else if build >= 22_621 {
        format!("Windows Build {build} · 支持 Mica / Mica Alt / 亚克力 / 透明")
    } else if build >= 17_763 {
        format!("Windows Build {build} · 支持亚克力 / 透明，Mica 需要 22H2")
    } else {
        format!("Windows Build {build} · 仅支持不透明材质")
    }
}

#[repr(C)]
struct OsVersionInfoW {
    size: u32,
    major: u32,
    minor: u32,
    build: u32,
    platform: u32,
    csd_version: [u16; 128],
}

#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(info: *mut OsVersionInfoW) -> i32;
}

/// `RtlGetVersion` reports the real build even when the manifest does not
/// declare modern Windows versions. Zero means detection failed.
pub fn windows_build() -> u32 {
    static BUILD: OnceLock<u32> = OnceLock::new();
    *BUILD.get_or_init(|| {
        let mut info: OsVersionInfoW = unsafe { std::mem::zeroed() };
        info.size = std::mem::size_of::<OsVersionInfoW>() as u32;
        if unsafe { RtlGetVersion(&mut info) } == 0 {
            info.build
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_the_real_windows_build() {
        let build = windows_build();
        assert!(
            build >= 17_763,
            "RtlGetVersion should report Windows 10 1809+, got {build}"
        );
    }

    #[test]
    fn capabilities_follow_the_reported_build() {
        let build = windows_build();
        assert!(Backdrop::Opaque.is_available());
        assert_eq!(Backdrop::Mica.is_available(), build >= 22_621);
        assert_eq!(Backdrop::MicaAlt.is_available(), build >= 22_621);
        assert_eq!(Backdrop::Acrylic.is_available(), build >= 17_763);
        assert_eq!(Backdrop::Clear.is_available(), build >= 17_763);
    }
}
