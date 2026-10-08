use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse {
        line: usize,
        message: String,
    },
    InvalidMesh(String),
    InvalidParams(String),
    /// A numerical step could not complete (singular system, no convergence).
    Numerical(String),
    /// Whoever was watching the run asked for it to stop.
    Cancelled,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "i/o error: {e}"),
            Error::Parse { line, message } => write!(f, "parse error at line {line}: {message}"),
            Error::InvalidMesh(m) => write!(f, "invalid mesh: {m}"),
            Error::InvalidParams(m) => write!(f, "invalid parameters: {m}"),
            Error::Numerical(m) => write!(f, "numerical failure: {m}"),
            Error::Cancelled => write!(f, "stopped on request"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
