use std::io;

use axum::response::IntoResponse;
use hyper::StatusCode;
use thiserror::Error;

use crate::server;

use super::path::Error as PathError;
use super::proc::OutputError;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("argument error")]
    ArgumentError,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("path error: {0}")]
    Path(#[from] PathError),

    #[error("output error: {0}")]
    Output(#[from] OutputError),

    #[error("server error: {0}")]
    Server(#[from] server::Error),

    #[error("unhandled error: {0}")]
    Unhandled(String),
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "there was a problem").into_response()
    }
}
