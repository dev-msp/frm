use std::path::Path;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path not unicode")]
    PathNotUnicode,

    #[error("file does not exist: \"{0}\"")]
    FileDoesNotExist(String),

    #[error("file already exists: \"{0}\"")]
    FileAlreadyExists(String),
}

use thiserror::Error;
use Error::*;

fn check_path(raw_str: &str) -> Result<&Path, Error> {
    let path = Path::new(raw_str);
    let path_str = path.to_str();
    if path_str.is_some() {
        Ok(path)
    } else {
        Err(PathNotUnicode)
    }
}

pub fn existing_path(raw_str: &str) -> Result<&Path, Error> {
    let path = check_path(raw_str)?;
    if !path.exists() {
        Err(FileDoesNotExist(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}

pub fn non_existing_path(raw_str: &String) -> Result<&Path, Error> {
    let path = check_path(raw_str)?;
    if path.exists() {
        Err(FileAlreadyExists(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}
