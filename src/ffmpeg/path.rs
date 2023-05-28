use std::path::Path;

#[derive(Debug)]
pub enum Error {
    PathNotUnicode,
    FileDoesNotExist(String),
    FileAlreadyExists(String),
}

use Error::*;

fn check_path<'a>(raw_str: &'a str) -> Result<&'a Path, Error> {
    let path = Path::new(raw_str);
    let path_str = path.to_str();
    if path_str.is_some() {
        Ok(path)
    } else {
        Err(PathNotUnicode)
    }
}

pub fn existing_path<'a>(raw_str: &'a str) -> Result<&'a Path, Error> {
    let path = check_path(raw_str)?;
    if !path.exists() {
        Err(FileDoesNotExist(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}

pub fn non_existing_path<'a>(raw_str: &'a String) -> Result<&'a Path, Error> {
    let path = check_path(raw_str)?;
    if path.exists() {
        Err(FileAlreadyExists(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}
