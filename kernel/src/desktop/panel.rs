//! Minimal top panel rendered as a solid strip.

use crate::graphics::color::Rgba;
use crate::graphics::framebuffer::{Framebuffer, FramebufferError};

const PANEL_HEIGHT: usize = 32;

pub fn render(framebuffer: &mut Framebuffer) -> Result<(), FramebufferError> {
    let info = framebuffer.info();
    framebuffer.fill_rect(
        0,
        0,
        info.width,
        info.height.min(PANEL_HEIGHT),
        Rgba::new(18, 24, 38, 255),
    )
}
