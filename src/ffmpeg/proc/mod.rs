use std::io;
use std::path::Path;
use std::process::{Command, Output};
use std::str::{from_utf8, FromStr, Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("non-zero exit {0}: {1}")]
    NonzeroExit(i32, String),

    #[error("signal terminated")]
    SigTerm,

    #[error("utf-8 error: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("parse error: {0}")]
    Parse(String),
}

#[allow(dead_code)]
fn debug_cmd(name: &str, args: &[String]) {
    println!(
        "{} {}",
        name,
        args.iter()
            .fold(String::new(), |acc, x| format!("{} {}", acc, x))
    );
}

pub fn run(name: &str, args: Vec<String>) -> Result<Output, OutputError> {
    let output = match Command::new(name).args(args).output() {
        Ok(x) => x,
        Err(e) => return Err(OutputError::from(e)),
    };

    match output.status.code() {
        Some(0) => Ok(output),
        Some(code) => {
            let text = from_utf8(output.stderr.as_slice()).unwrap_or("");
            Err(OutputError::NonzeroExit(code, String::from(text.trim())))
        }
        None => Err(OutputError::SigTerm),
    }
}

pub fn dump(output: Output) -> Result<String, OutputError> {
    let text = from_utf8(output.stdout.as_slice())?;
    Ok(String::from(text))
}

pub fn write_to_file(path: &Path, output: &Vec<u8>) -> Result<(), OutputError> {
    use std::fs::File;
    use std::io::Write;

    let display = path.display();
    let bytes = output.as_slice();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write_all(bytes).map_err(OutputError::from)
}

pub fn parse_from_output<T>(output: Output) -> Result<T, OutputError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let resolved_output = dump(output)?;
    let text = resolved_output.as_str();

    match text.trim().parse::<T>() {
        Ok(v) => Ok(v),
        Err(e) => {
            let message = format!("{e} ({text})");
            Err(OutputError::Parse(message))
        }
    }
}
