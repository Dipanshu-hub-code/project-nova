//! Minimal centered dock rendered as a bottom strip.

use crate::graphics::color::Rgba;
use crate::graphics::framebuffer::{Framebuffer, FramebufferError};

const DOCK_WIDTH: usize = 256;
const DOCK_HEIGHT: usize = 48;

pub fn render(framebuffer: &mut Framebuffer) -> Result<(), FramebufferError> {
    let info = framebuffer.info();
    let width = info.width.min(DOCK_WIDTH);
    let height = info.height.min(DOCK_HEIGHT);
    framebuffer.fill_rect(
        (info.width - width) / 2,
        info.height - height,
        width,
        height,
        Rgba::new(35, 42, 58, 255),
    )
}
