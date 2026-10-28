//! Small, deterministic layout values shared by host-rendered applications.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Insets {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Insets {
    pub const ZERO: Self = Self {
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    };

    pub const fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn content_size(self, outer: Size) -> Option<Size> {
        let horizontal = u32::from(self.left) + u32::from(self.right);
        let vertical = u32::from(self.top) + u32::from(self.bottom);
        Some(Size {
            width: outer.width.checked_sub(horizontal)?,
            height: outer.height.checked_sub(vertical)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackLayout {
    pub direction: StackDirection,
    pub gap: u16,
    pub padding: Insets,
}

impl StackLayout {
    pub const fn horizontal(gap: u16, padding: Insets) -> Self {
        Self {
            direction: StackDirection::Horizontal,
            gap,
            padding,
        }
    }

    pub const fn vertical(gap: u16, padding: Insets) -> Self {
        Self {
            direction: StackDirection::Vertical,
            gap,
            padding,
        }
    }

    pub fn content_size(self, outer: Size) -> Option<Size> {
        self.padding.content_size(outer)
    }
}
