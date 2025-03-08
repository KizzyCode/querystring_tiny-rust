#![doc = include_str!("../README.md")]

#[macro_use]
pub mod error;
mod percentcoding;
mod querystring;

// Re-export the symbol
pub use crate::percentcoding::PercentCoded;
pub use crate::querystring::QueryString;
