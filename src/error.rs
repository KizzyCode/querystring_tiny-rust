use ebacktrace::define_error;
use std::{
    result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::new($kind, format!($($arg)*)) })
}
/// Creates a new `ErrorImpl::InvalidValue` kind
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidValue, $($arg)*) });
}


/// The error kind
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// A value is invalid
    InvalidValue
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidValue => write!(f, "A value is invalid")
        }
    }
}


// Define our custom error type
define_error!(ErrorImpl);


/// A nice typealias for our custom error
pub type Error = ErrorImpl<ErrorKind>;
/// A nice typealias for a `Result` with our custom error
pub type Result<T = ()> = result::Result<T, ErrorImpl<ErrorKind>>;
