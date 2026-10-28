//! Bounds-safe software drawing primitives.

use super::color::Rgba;
use super::framebuffer::{Framebuffer, FramebufferError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub const fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: usize, py: usize) -> bool {
        px.checked_sub(self.x).is_some_and(|dx| dx < self.w)
            && py.checked_sub(self.y).is_some_and(|dy| dy < self.h)
    }
}

pub fn draw_rect(
    framebuffer: &mut Framebuffer,
    rect: &Rect,
    color: Rgba,
) -> Result<(), FramebufferError> {
    let info = framebuffer.info();
    let start_x = rect.x.min(info.width);
    let start_y = rect.y.min(info.height);
    let end_x = rect.x.saturating_add(rect.w).min(info.width);
    let end_y = rect.y.saturating_add(rect.h).min(info.height);

    if start_x >= end_x || start_y >= end_y {
        return Ok(());
    }

    framebuffer.fill_rect(start_x, start_y, end_x - start_x, end_y - start_y, color)
}
