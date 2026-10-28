//! Bounded package metadata and planning primitives.
//!
//! Repository fetching, signature verification, persistent storage, and package
//! installation are unavailable until the kernel has the required subsystems.

pub mod database;
pub mod installer;
pub mod metadata;
pub mod repository;
pub mod resolver;
