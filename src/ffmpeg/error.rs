use thiserror::Error;
use warp::reject::Reject;

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

    #[error("unhandled error: {0}")]
    Unhandled(String),
}

impl Reject for ErrorKind {}
