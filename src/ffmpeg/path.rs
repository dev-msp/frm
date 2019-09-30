use super::ErrorKind;
use std::path::Path;

fn check_path<'a>(raw_str: &'a String) -> Result<&'a Path, ErrorKind> {
    let path = Path::new(raw_str.as_str());
    let path_str = path.to_str();
    if path_str.is_some() {
        Ok(path)
    } else {
        Err(ErrorKind::PathNotUnicode)
    }
}

pub fn existing_path<'a>(raw_str: &'a String) -> Result<&'a Path, ErrorKind> {
    let path = check_path(raw_str)?;
    if !path.exists() {
        Err(ErrorKind::FileDoesNotExist(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}

pub fn non_existing_path<'a>(raw_str: &'a String) -> Result<&'a Path, ErrorKind> {
    let path = check_path(raw_str)?;
    if path.exists() {
        Err(ErrorKind::FileAlreadyExists(raw_str.to_owned()))
    } else {
        Ok(path)
    }
}
