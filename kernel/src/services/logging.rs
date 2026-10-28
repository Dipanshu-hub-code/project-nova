//! Structured log record types. A persistent or serial log sink is not wired
//! into this service yet.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogRecord<'a> {
    pub level: LogLevel,
    pub message: &'a str,
}

pub const fn record(level: LogLevel, message: &str) -> LogRecord<'_> {
    LogRecord { level, message }
}
