#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
extern crate std;

mod budget;

pub use budget::{
    DecodeBudget, DecodeBudgetError, DecodeBudgetErrorCategory, DecodeBudgetTracker, NestedBudget,
};

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
