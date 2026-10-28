//! Validated device resource descriptions; no resource is mapped or reserved.

use crate::hal::memory::MmioRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Mmio(MmioRange),
    PortRange { start: u16, end: u16 },
    Irq(u8),
}

impl Resource {
    pub fn port_range(start: u16, end: u16) -> Option<Self> {
        (start <= end).then_some(Self::PortRange { start, end })
    }

    pub fn overlaps(self, other: Self) -> bool {
        match (self, other) {
            (Self::Mmio(left), Self::Mmio(right)) => left.overlaps(right),
            (
                Self::PortRange {
                    start: left_start,
                    end: left_end,
                },
                Self::PortRange {
                    start: right_start,
                    end: right_end,
                },
            ) => left_start <= right_end && right_start <= left_end,
            (Self::Irq(left), Self::Irq(right)) => left == right,
            _ => false,
        }
    }
}
