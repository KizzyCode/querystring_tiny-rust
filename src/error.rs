//! Implements the crate's error type

/// An error
#[derive(Debug, Error)]
pub enum Error {
    /// The HTTP header is invalid
    #[error("invalid HTTP header")]
    QueryString,
    /// An in-/out-error
    #[error("in/out error")]
    PercentEncoding,
}
