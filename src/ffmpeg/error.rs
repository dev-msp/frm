use thiserror::Error;

use super::path::Error as PathError;
use super::proc::OutputError;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("argument error")]
    ArgumentError,

    #[error("path error: {0}")]
    Path(#[from] PathError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),
}
