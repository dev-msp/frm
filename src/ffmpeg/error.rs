use super::proc::OutputError;
use std::error::Error;

#[derive(Debug)]
pub enum ErrorKind {
    ArgumentError,
    PathNotUnicode,
    FileAlreadyExists(String),
    FileDoesNotExist(String),
    Output(OutputError),
}

impl From<OutputError> for ErrorKind {
    fn from(e: OutputError) -> Self {
        ErrorKind::Output(e)
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
