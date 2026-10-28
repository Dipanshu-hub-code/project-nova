//! Basic desktop composition. A mapped framebuffer is required for display.

pub mod dock;
pub mod panel;

use crate::graphics::color::Rgba;
use crate::graphics::framebuffer::{Framebuffer, FramebufferError};
use crate::window_manager::{compositor, WindowManager, WindowManagerError};

pub struct Desktop {
    wallpaper_color: Rgba,
}

impl Desktop {
    pub const fn new() -> Self {
        Self {
            wallpaper_color: Rgba::new(20, 20, 30, 255),
        }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        window_manager: &WindowManager,
    ) -> Result<(), DesktopError> {
        let info = framebuffer.info();
        framebuffer.fill_rect(0, 0, info.width, info.height, self.wallpaper_color)?;
        compositor::compose(window_manager, framebuffer)?;
        panel::render(framebuffer)?;
        dock::render(framebuffer)?;
        Ok(())
    }
}

impl Default for Desktop {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopError {
    Framebuffer(FramebufferError),
    WindowManager(WindowManagerError),
}

impl From<FramebufferError> for DesktopError {
    fn from(error: FramebufferError) -> Self {
        Self::Framebuffer(error)
    }
}

impl From<WindowManagerError> for DesktopError {
    fn from(error: WindowManagerError) -> Self {
        Self::WindowManager(error)
    }
}
