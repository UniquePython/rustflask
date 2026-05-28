use std::fmt;

#[derive(Debug)]
pub enum RustFlaskError {
    /// Wrapper around std::io::Error
    Io(std::io::Error),

    /// The HTTP request was malformed
    BadRequest,

    /// The request body exceeded the allowed size
    RequestTooLarge,

    /// No matching route was found
    NotFound,
}

impl fmt::Display for RustFlaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustFlaskError::Io(err) => write!(f, "I/O error: {}", err),
            RustFlaskError::BadRequest => write!(f, "Malformed HTTP request"),
            RustFlaskError::RequestTooLarge => write!(f, "Request too large"),
            RustFlaskError::NotFound => write!(f, "No route matched"),
        }
    }
}

impl std::error::Error for RustFlaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RustFlaskError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RustFlaskError {
    fn from(err: std::io::Error) -> Self {
        RustFlaskError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, RustFlaskError>;
