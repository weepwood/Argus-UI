//! Semantic colors, density, and spacing for the Argus-inspired component kit.
use gpui_kit::{Hsla, rgb};

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
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

impl Palette {
    pub fn new(mode: Appearance) -> Self {
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
            },
        }
    }
}
