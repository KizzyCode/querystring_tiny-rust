#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

#[macro_use]
pub mod error;
mod percentcoding;
mod querystring;

// Re-export the symbol
pub use crate::{percentcoding::PercentCoded, querystring::QueryString};
