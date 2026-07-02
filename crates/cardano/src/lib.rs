#![no_std]
#![forbid(unsafe_code)]
//! no_std-first Cardano protocol building blocks for Rust.
//!
//! Most users should start with the repository README and release plan:
//!
//! - <https://github.com/valkyoth/cardano>
//! - <https://github.com/valkyoth/cardano/blob/main/docs/RELEASE_PLAN.md>

pub use cardano_valkyoth_primitives as primitives;

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
