//! Framebuffer rendering primitives. A hardware driver must supply a mapped
//! framebuffer before the desktop can be rendered.

pub mod color;
pub mod framebuffer;
pub mod renderer;

pub use framebuffer::{Framebuffer, FramebufferError, FramebufferInfo, PixelFormat};
