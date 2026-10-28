//! Process launching is intentionally unavailable until a verified user-mode
//! loader, scheduler, and isolation boundary are implemented.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchError {
    ProcessExecutionUnavailable,
}

pub fn launch(_app_id: u32) -> Result<(), LaunchError> {
    Err(LaunchError::ProcessExecutionUnavailable)
}

pub fn terminate(_app_id: u32) -> Result<(), LaunchError> {
    Err(LaunchError::ProcessExecutionUnavailable)
}
