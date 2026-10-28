//! Lightweight design tokens shared by Nova UI applications.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub elevated_surface: Color,
    pub text: Color,
    pub secondary_text: Color,
    pub border: Color,
    pub accent: Color,
    pub focus: Color,
    pub danger: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub const fn palette(self) -> Palette {
        match self {
            Self::Light => Palette {
                background: Color(0xfff5f6fa),
                surface: Color(0xffffffff),
                elevated_surface: Color(0xfffdfdff),
                text: Color(0xff1d2330),
                secondary_text: Color(0xff596274),
                border: Color(0xffd9deea),
                accent: Color(0xff6557d2),
                focus: Color(0xff4334bb),
                danger: Color(0xffa12635),
            },
            Self::Dark => Palette {
                background: Color(0xff111521),
                surface: Color(0xff191f2c),
                elevated_surface: Color(0xff222a39),
                text: Color(0xfff4f6fb),
                secondary_text: Color(0xffc0c7d6),
                border: Color(0xff394254),
                accent: Color(0xffb5a7ff),
                focus: Color(0xffd0c7ff),
                danger: Color(0xffff9da7),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spacing;

impl Spacing {
    pub const XX_SMALL: u8 = 2;
    pub const X_SMALL: u8 = 4;
    pub const SMALL: u8 = 8;
    pub const MEDIUM: u8 = 12;
    pub const LARGE: u8 = 16;
    pub const X_LARGE: u8 = 24;
    pub const XX_LARGE: u8 = 32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CornerRadius;

impl CornerRadius {
    pub const SMALL: u8 = 6;
    pub const MEDIUM: u8 = 10;
    pub const LARGE: u8 = 16;
    pub const PILL: u8 = 255;
}
