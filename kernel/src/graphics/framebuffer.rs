//! Checked access to a mapped, 32-bit framebuffer supplied by a display driver.

use core::ptr::{write_volatile, NonNull};

use super::color::Rgba;

const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FramebufferInfo {
    pub width: usize,
    pub height: usize,
    /// Number of pixels between the start of adjacent rows.
    pub stride: usize,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferError {
    InvalidLayout,
    OutOfBounds,
}

pub struct Framebuffer {
    info: FramebufferInfo,
    base: NonNull<u8>,
    byte_len: usize,
}

impl Framebuffer {
    /// Creates a framebuffer over a mapped 32-bit pixel buffer.
    ///
    /// # Safety
    ///
    /// `base` must point to a writable, mapped region of at least `byte_len`
    /// bytes that remains valid for the lifetime of this framebuffer. The
    /// caller must ensure exclusive access while drawing.
    pub unsafe fn new(
        base: *mut u8,
        byte_len: usize,
        info: FramebufferInfo,
    ) -> Result<Self, FramebufferError> {
        if info.width == 0
            || info.height == 0
            || info.stride < info.width
            || info.width.checked_mul(BYTES_PER_PIXEL).is_none()
            || info
                .stride
                .checked_mul(info.height)
                .and_then(|pixels| pixels.checked_mul(BYTES_PER_PIXEL))
                .filter(|required| *required <= byte_len)
                .is_none()
        {
            return Err(FramebufferError::InvalidLayout);
        }

        let base = NonNull::new(base).ok_or(FramebufferError::InvalidLayout)?;
        Ok(Self {
            info,
            base,
            byte_len,
        })
    }

    pub const fn info(&self) -> FramebufferInfo {
        self.info
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Rgba) -> Result<(), FramebufferError> {
        if x >= self.info.width || y >= self.info.height {
            return Err(FramebufferError::OutOfBounds);
        }

        let offset = y
            .checked_mul(self.info.stride)
            .and_then(|row| row.checked_add(x))
            .and_then(|pixel| pixel.checked_mul(BYTES_PER_PIXEL))
            .ok_or(FramebufferError::InvalidLayout)?;
        if offset
            .checked_add(BYTES_PER_PIXEL)
            .filter(|end| *end <= self.byte_len)
            .is_none()
        {
            return Err(FramebufferError::InvalidLayout);
        }

        let channels = match self.info.format {
            PixelFormat::Rgb => [color.r, color.g, color.b, color.a],
            PixelFormat::Bgr => [color.b, color.g, color.r, color.a],
        };
        for (index, channel) in channels.into_iter().enumerate() {
            unsafe {
                write_volatile(self.base.as_ptr().add(offset + index), channel);
            }
        }
        Ok(())
    }

    pub fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Rgba,
    ) -> Result<(), FramebufferError> {
        let end_x = x.checked_add(width).ok_or(FramebufferError::OutOfBounds)?;
        let end_y = y.checked_add(height).ok_or(FramebufferError::OutOfBounds)?;
        if end_x > self.info.width || end_y > self.info.height {
            return Err(FramebufferError::OutOfBounds);
        }

        let channels = match self.info.format {
            PixelFormat::Rgb => [color.r, color.g, color.b, color.a],
            PixelFormat::Bgr => [color.b, color.g, color.r, color.a],
        };
        let row_bytes = width
            .checked_mul(BYTES_PER_PIXEL)
            .ok_or(FramebufferError::InvalidLayout)?;
        let row_stride_bytes = self
            .info
            .stride
            .checked_mul(BYTES_PER_PIXEL)
            .ok_or(FramebufferError::InvalidLayout)?;
        let column_offset = x
            .checked_mul(BYTES_PER_PIXEL)
            .ok_or(FramebufferError::InvalidLayout)?;

        for row in y..end_y {
            let row_offset = row
                .checked_mul(row_stride_bytes)
                .and_then(|offset| offset.checked_add(column_offset))
                .ok_or(FramebufferError::InvalidLayout)?;
            row_offset
                .checked_add(row_bytes)
                .filter(|end| *end <= self.byte_len)
                .ok_or(FramebufferError::InvalidLayout)?;

            if width == 0 {
                continue;
            }

            let mut pixel = unsafe { self.base.as_ptr().add(row_offset) };
            for _ in x..end_x {
                // The rectangle bounds and checked row end keep these volatile
                // writes and pointer advances inside the mapped framebuffer.
                unsafe {
                    write_volatile(pixel, channels[0]);
                    write_volatile(pixel.add(1), channels[1]);
                    write_volatile(pixel.add(2), channels[2]);
                    write_volatile(pixel.add(3), channels[3]);
                    pixel = pixel.add(BYTES_PER_PIXEL);
                }
            }
        }
        Ok(())
    }
}
