//! Fixed-capacity window manager suitable for a kernel without a heap.

pub mod compositor;
pub mod window;

use crate::graphics::color::Rgba;
use crate::graphics::framebuffer::{Framebuffer, FramebufferError};

use self::window::Window;

pub const MAX_WINDOWS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowManagerError {
    Full,
    Framebuffer(FramebufferError),
}

impl From<FramebufferError> for WindowManagerError {
    fn from(error: FramebufferError) -> Self {
        Self::Framebuffer(error)
    }
}

pub struct WindowManager {
    windows: [Option<Window>; MAX_WINDOWS],
    focused: Option<usize>,
    background_color: Rgba,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: [None; MAX_WINDOWS],
            focused: None,
            background_color: Rgba::DARK_BLUE,
        }
    }

    pub fn add_window(&mut self, window: Window) -> Result<(), WindowManagerError> {
        let slot = self
            .windows
            .iter()
            .position(Option::is_none)
            .ok_or(WindowManagerError::Full)?;
        self.windows[slot] = Some(window);
        self.focused = Some(slot);
        Ok(())
    }

    pub fn focus(&mut self, id: usize) -> bool {
        let Some(slot) = self
            .windows
            .iter()
            .position(|window| window.is_some_and(|window| window.id == id))
        else {
            return false;
        };
        self.focused = Some(slot);
        true
    }

    pub fn render(&self, framebuffer: &mut Framebuffer) -> Result<(), WindowManagerError> {
        let info = framebuffer.info();
        framebuffer.fill_rect(0, 0, info.width, info.height, self.background_color)?;

        self.render_windows(framebuffer)
    }

    pub fn render_windows(&self, framebuffer: &mut Framebuffer) -> Result<(), WindowManagerError> {
        for (slot, window) in self.windows.iter().enumerate() {
            if let Some(window) = window {
                window.render(framebuffer, self.focused == Some(slot))?;
            }
        }
        Ok(())
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}
