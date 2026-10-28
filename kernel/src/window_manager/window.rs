//! Window bounds and basic title/content rendering.

use crate::graphics::color::Rgba;
use crate::graphics::framebuffer::{Framebuffer, FramebufferError};
use crate::graphics::renderer::{draw_rect, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub id: usize,
    pub title: &'static str,
    pub bounds: Rect,
    pub visible: bool,
}

impl Window {
    pub const fn new(
        id: usize,
        title: &'static str,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            id,
            title,
            bounds: Rect::new(x, y, width, height),
            visible: true,
        }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        focused: bool,
    ) -> Result<(), FramebufferError> {
        if !self.visible {
            return Ok(());
        }

        let title_height = self.bounds.h.min(24);
        draw_rect(
            framebuffer,
            &Rect::new(self.bounds.x, self.bounds.y, self.bounds.w, title_height),
            if focused { Rgba::BLUE } else { Rgba::GRAY },
        )?;

        if self.bounds.h > title_height {
            draw_rect(
                framebuffer,
                &Rect::new(
                    self.bounds.x,
                    self.bounds.y.saturating_add(title_height),
                    self.bounds.w,
                    self.bounds.h - title_height,
                ),
                Rgba::WHITE,
            )?;
        }
        Ok(())
    }
}
