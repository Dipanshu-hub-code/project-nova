//! Composes the desktop window stack into a framebuffer.

use crate::graphics::framebuffer::Framebuffer;

use super::{WindowManager, WindowManagerError};

pub fn compose(
    manager: &WindowManager,
    framebuffer: &mut Framebuffer,
) -> Result<(), WindowManagerError> {
    manager.render_windows(framebuffer)
}
