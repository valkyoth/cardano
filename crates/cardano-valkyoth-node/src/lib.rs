#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
