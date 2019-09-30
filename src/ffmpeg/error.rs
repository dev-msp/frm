use super::proc::OutputError;
use std::error::Error;
use super::path::Error as PathError;

#[derive(Debug)]
pub enum ErrorKind {
    ArgumentError,
    Path(PathError),
    Output(OutputError),
}

impl From<OutputError> for ErrorKind {
    fn from(e: OutputError) -> Self {
        ErrorKind::Output(e)
    }
}

impl From<PathError> for ErrorKind {
    fn from(e: PathError) -> Self {
        ErrorKind::Path(e)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
