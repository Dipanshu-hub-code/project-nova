//! Bounded, caller-owned security event queue; not a protected or persistent log.

pub const LOG_CAPACITY: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityEvent {
    AuthenticationSucceeded,
    AuthenticationFailed,
    AccountStatusChanged,
    PermissionDenied,
    PackageVerificationFailed,
    BootIntegrityFailed,
    RecoveryRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    Full,
}

pub struct SecurityLog {
    events: [Option<SecurityEvent>; LOG_CAPACITY],
    head: usize,
    len: usize,
}

impl SecurityLog {
    pub const fn new() -> Self {
        Self {
            events: [None; LOG_CAPACITY],
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, event: SecurityEvent) -> Result<(), LogError> {
        if self.len == LOG_CAPACITY {
            return Err(LogError::Full);
        }
        let tail = (self.head + self.len) % LOG_CAPACITY;
        self.events[tail] = Some(event);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<SecurityEvent> {
        if self.len == 0 {
            return None;
        }
        let event = self.events[self.head].take();
        self.head = (self.head + 1) % LOG_CAPACITY;
        self.len -= 1;
        event
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for SecurityLog {
    fn default() -> Self {
        Self::new()
    }
}
