//! Host-buildable Nova application API facade.
//!
//! The current kernel has no syscall entry point, so system operations return
//! `NotSupported` instead of issuing unsafe instructions or faking success.

pub mod error;
pub mod fs;
pub mod log;
pub mod process;
pub mod time;

pub use error::NovaError;

pub const fn sdk_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::{fs, log, process, time, NovaError};

    #[test]
    fn public_system_services_fail_explicitly() {
        assert_eq!(log::write_log("hello"), Err(NovaError::NotSupported));
        assert_eq!(process::exit(0), Err(NovaError::NotSupported));
        assert_eq!(process::get_process_info(), Err(NovaError::NotSupported));
        assert_eq!(time::uptime_ms(), Err(NovaError::NotSupported));
        assert_eq!(fs::open("/home/user/file"), Err(NovaError::NotSupported));
    }

    #[test]
    fn sdk_reports_manifest_version() {
        assert_eq!(super::sdk_version(), "0.1.0");
    }
}
