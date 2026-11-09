//! Original semantic tokens derived from restrained Windows Fluent design principles.
use gpui_kit::{Hsla, rgb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}
impl Appearance {
    pub fn name(self) -> &'static str {
        match self {
            Self::Light => "浅色",
            Self::Dark => "深色",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
}
impl Density {
    pub fn row_height(self) -> f32 {
        match self {
            Self::Compact => 34.,
            Self::Comfortable => 43.,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Compact => "紧凑",
            Self::Comfortable => "舒适",
        }
    }
}
#[derive(Clone, Copy)]
pub struct Palette {
    pub background: Hsla,
    pub surface: Hsla,
    pub elevated: Hsla,
    pub control: Hsla,
    pub border: Hsla,
    pub border_strong: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub faint: Hsla,
    pub accent: Hsla,
    pub accent_ink: Hsla,
    pub accent_soft: Hsla,
    pub hover: Hsla,
    pub cpu: Hsla,
    pub memory: Hsla,
    pub disk: Hsla,
    pub network: Hsla,
    pub gpu: Hsla,
    pub success: Hsla,
    pub danger: Hsla,
}
fn hex(value: u32) -> Hsla { rgb(value).into() }
impl Palette {
    pub fn new(mode: Appearance) -> Self {
        match mode {
            Appearance::Light => Self {
                background: hex(0xf3f3f5),
                surface: hex(0xffffff),
                elevated: hex(0xfafafb),
                control: hex(0xf5f5f7),
                border: hex(0xe4e5e9),
                border_strong: hex(0xd3d5dc),
                text: hex(0x23252c),
                muted: hex(0x6b6f7b),
                faint: hex(0x9397a2),
                accent: hex(0x0067c0),
                accent_ink: hex(0xffffff),
                accent_soft: hex(0xe7f2fd),
                hover: hex(0xf0f2f5),
                cpu: hex(0x168cb3),
                memory: hex(0x7655da),
                disk: hex(0xc58c32),
                network: hex(0x249a68),
                gpu: hex(0xd65086),
                success: hex(0x249a68),
                danger: hex(0xc93838),
            },
            Appearance::Dark => Self {
                background: hex(0x202124),
                surface: hex(0x2a2b2f),
                elevated: hex(0x303137),
                control: hex(0x34363b),
                border: hex(0x41434a),
                border_strong: hex(0x585b64),
                text: hex(0xf1f2f5),
                muted: hex(0xb0b3bc),
                faint: hex(0x878b96),
                accent: hex(0x59baff),
                accent_ink: hex(0x09263b),
                accent_soft: hex(0x243c4d),
                hover: hex(0x37393f),
                cpu: hex(0x49c7e8),
                memory: hex(0xa695ff),
                disk: hex(0xf3bc67),
                network: hex(0x64d69f),
                gpu: hex(0xf686b4),
                success: hex(0x64d69f),
                danger: hex(0xff786f),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_rows_are_smaller() {
        assert!(Density::Compact.row_height() < Density::Comfortable.row_height());
    }
    #[test]
    fn appearance_names_are_distinct() {
        assert_ne!(Appearance::Light.name(), Appearance::Dark.name());
    }
}
