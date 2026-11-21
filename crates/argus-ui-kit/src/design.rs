//! Semantic colors, density, and spacing for the Argus-inspired component kit.
use gpui_kit::{Hsla, rgb};

use crate::backdrop::Backdrop;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
}

impl Density {
    pub fn row_height(self) -> f32 {
        match self {
            Self::Compact => 36.,
            Self::Comfortable => 42.,
        }
    }
}

/// How much of the DWM material is allowed behind text.
///
/// GPUI disables ClearType subpixel antialiasing whenever the window backdrop
/// is not `Opaque`, so translucent windows always get grayscale glyphs. Over a
/// detailed wallpaper, those half-covered edges read as jaggies. `Crisp` keeps
/// the material tint while putting a near-solid plate under every text run;
/// `Glass` maximises see-through; `Sharp` drops the material entirely so the
/// platform can use ClearType again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaterialClarity {
    /// Near-solid plates under text, material still tints the window.
    #[default]
    Crisp,
    /// Maximum transparency, closest to the reference glass look.
    Glass,
    /// Opaque window + ClearType: the sharpest glyphs, no system material.
    Sharp,
}

impl MaterialClarity {
    pub const ALL: [Self; 3] = [Self::Crisp, Self::Glass, Self::Sharp];

    pub fn id(self) -> &'static str {
        match self {
            Self::Crisp => "crisp",
            Self::Glass => "glass",
            Self::Sharp => "sharp",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Crisp => "清晰",
            Self::Glass => "通透",
            Self::Sharp => "锐利 ClearType",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Crisp => "默认。文字下方保留近实底托层，材质色调仍可见，灰度抗锯齿下最清楚。",
            Self::Glass => "最大化玻璃通透感，桌面纹理会透过文字底部，小字可能显毛边。",
            Self::Sharp => {
                "自动切换为不透明窗口，启用系统 ClearType 子像素渲染，字形最锐利（无毛玻璃）。"
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub background: Hsla,
    pub surface: Hsla,
    pub raised: Hsla,
    pub control: Hsla,
    pub border: Hsla,
    pub border_strong: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub faint: Hsla,
    pub accent: Hsla,
    pub accent_soft: Hsla,
    pub accent_ink: Hsla,
    pub hover: Hsla,
    pub cpu: Hsla,
    pub memory: Hsla,
    pub disk: Hsla,
    pub network: Hsla,
    pub gpu: Hsla,
    pub danger: Hsla,
    /// The opaque window colour of the current mode. Used to pre-compose text
    /// plates so layered translucent fills do not darken the composite.
    solid_background: Hsla,
    /// Whether a DWM material is active and how much clarity was requested.
    pub material: bool,
    pub clarity: MaterialClarity,
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

/// A surface colour with its own alpha, so it can sit over a DWM backdrop.
fn tint(value: u32, alpha: f32) -> Hsla {
    hex(value).alpha(alpha)
}

impl Palette {
    /// Solid surfaces for `Opaque`, translucent layered materials otherwise.
    pub fn new(mode: Appearance, backdrop: Backdrop, clarity: MaterialClarity) -> Self {
        let mut palette = Self::solid(mode);
        palette.material = backdrop.is_translucent();
        palette.clarity = clarity;
        if backdrop.is_translucent() {
            palette.apply_material(mode, backdrop, clarity);
        }
        palette
    }

    /// Fill for a surface that actually carries text.
    ///
    /// The reference pre-composes each control colour over the opaque window
    /// colour and then forces it opaque, because stacking translucent fills on
    /// a transparent window darkens the composite and lets wallpaper detail
    /// reach the glyph edges. We do the same in `Crisp`: `nominal` still sets
    /// the layer's visual weight, but the result is one opaque plate. Strokes,
    /// hovers and chart tints keep their original alpha.
    pub fn text_plate(&self, base: Hsla, nominal: f32) -> Hsla {
        if self.material && self.clarity == MaterialClarity::Crisp {
            self.solid_background
                .blend(base.opacity(nominal.clamp(0., 1.)))
                .alpha(1.)
        } else {
            base.opacity(nominal)
        }
    }

    fn solid(mode: Appearance) -> Self {
        match mode {
            Appearance::Light => Self {
                background: hex(0xeef3f6),
                surface: hex(0xf8fafb),
                raised: hex(0xffffff),
                control: hex(0xf4f6f8),
                border: hex(0xdce2e7),
                border_strong: hex(0xc8d0d8),
                text: hex(0x242a32),
                muted: hex(0x687483),
                faint: hex(0x929ba7),
                accent: hex(0x168bd2),
                accent_soft: hex(0xe1f2fc),
                accent_ink: hex(0xffffff),
                hover: hex(0xe8edf1),
                cpu: hex(0x159ed1),
                memory: hex(0x755be5),
                disk: hex(0xd78a00),
                network: hex(0x1ca65b),
                gpu: hex(0xe13d79),
                danger: hex(0xc84245),
                solid_background: hex(0xeef3f6),
                material: false,
                clarity: MaterialClarity::Crisp,
            },
            Appearance::Dark => Self {
                background: hex(0x1f2225),
                surface: hex(0x25292c),
                raised: hex(0x2d3236),
                control: hex(0x303539),
                border: hex(0x393f43),
                border_strong: hex(0x4a5258),
                text: hex(0xf1f3f5),
                muted: hex(0xaab2ba),
                faint: hex(0x7e8993),
                accent: hex(0x50c8f1),
                accent_soft: hex(0x183945),
                accent_ink: hex(0x102027),
                hover: hex(0x32383c),
                cpu: hex(0x43c9ed),
                memory: hex(0xa393ff),
                disk: hex(0xf0b64c),
                network: hex(0x56d39a),
                gpu: hex(0xf277a9),
                danger: hex(0xff7772),
                solid_background: hex(0x1f2225),
                material: false,
                clarity: MaterialClarity::Crisp,
            },
        }
    }

    /// Let the system material show through while keeping text readable.
    ///
    /// `apply_material` used to lower the alpha of every surface colour. The
    /// call sites then multiply it again (`p.raised.opacity(0.42)`), so a Mica
    /// panel ended up at roughly 0.30 alpha and glyph antialiasing blended
    /// straight into the wallpaper. `Crisp` keeps a thick root veil plus
    /// near-solid text plates; `Glass` preserves the original see-through look.
    fn apply_material(&mut self, mode: Appearance, backdrop: Backdrop, clarity: MaterialClarity) {
        let (root, surface, raised, control) = match (clarity, backdrop) {
            (MaterialClarity::Crisp, Backdrop::Mica | Backdrop::MicaAlt) => {
                (0.62, 0.92, 0.96, 0.90)
            }
            (MaterialClarity::Crisp, Backdrop::Acrylic) => (0.70, 0.94, 0.97, 0.92),
            (MaterialClarity::Crisp, Backdrop::Clear) => (0.78, 0.96, 0.98, 0.94),
            (MaterialClarity::Glass, Backdrop::Mica | Backdrop::MicaAlt) => {
                (0.06, 0.54, 0.72, 0.62)
            }
            (MaterialClarity::Glass, Backdrop::Acrylic) => (0.10, 0.60, 0.76, 0.68),
            (MaterialClarity::Glass, Backdrop::Clear) => (0.14, 0.66, 0.80, 0.72),
            (_, Backdrop::Opaque) | (MaterialClarity::Sharp, _) => return,
        };
        match mode {
            Appearance::Light => {
                self.background = tint(0xeef3f6, root);
                self.surface = tint(0xffffff, surface);
                self.raised = tint(0xffffff, raised);
                self.control = tint(0xffffff, control);
                self.border = tint(0x000000, 0.10);
                self.border_strong = tint(0x000000, 0.16);
                self.hover = tint(0x000000, 0.045);
                self.accent_soft = self.accent.alpha(0.14);
            }
            Appearance::Dark => {
                self.background = tint(0x1f2225, root);
                self.surface = tint(0x2d3236, surface);
                self.raised = tint(0x303539, raised);
                self.control = tint(0xffffff, 0.07);
                self.border = tint(0xffffff, 0.09);
                self.border_strong = tint(0xffffff, 0.15);
                self.hover = tint(0xffffff, 0.055);
                self.accent_soft = self.accent.alpha(0.16);
            }
        }
        // Grayscale antialiasing thins low-ink secondary labels. Pull them
        // closer to the primary text colour whenever clarity is preferred.
        if clarity == MaterialClarity::Crisp {
            match mode {
                Appearance::Light => {
                    self.muted = hex(0x58636f);
                    self.faint = hex(0x78838f);
                }
                Appearance::Dark => {
                    self.muted = hex(0xbbc3cb);
                    self.faint = hex(0x939da7);
                }
            }
        }
    }
}
