//! RGBA colors used by the desktop renderer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const GRAY: Self = Self::new(200, 200, 200, 255);
    pub const BLUE: Self = Self::new(70, 130, 200, 255);
    pub const DARK_BLUE: Self = Self::new(30, 80, 140, 255);
    pub const RED: Self = Self::new(220, 50, 50, 255);
    pub const GREEN: Self = Self::new(80, 180, 80, 255);
}
