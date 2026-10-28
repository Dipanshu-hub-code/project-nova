//! A missing timer is reported as unavailable rather than a fabricated uptime.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    TimerUnavailable,
}

pub fn uptime_ms() -> Result<u64, TimeError> {
    Err(TimeError::TimerUnavailable)
}
