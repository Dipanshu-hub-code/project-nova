//! Checked physical ranges. This type does not map physical memory.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmioRange {
    base: u64,
    size: u64,
}

impl MmioRange {
    pub fn new(base: u64, size: u64) -> Option<Self> {
        if size == 0 || base.checked_add(size).is_none() {
            return None;
        }
        Some(Self { base, size })
    }

    pub const fn base(self) -> u64 {
        self.base
    }

    pub const fn size(self) -> u64 {
        self.size
    }

    pub fn contains(self, address: u64) -> bool {
        address >= self.base && address < self.base + self.size
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.base < other.base + other.size && other.base < self.base + self.size
    }
}
