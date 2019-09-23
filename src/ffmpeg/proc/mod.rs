use std::io;
use std::process::{Command, Output};
use std::str::{FromStr, Utf8Error};

#[derive(Debug)]
pub enum OutputError {
    Io(String),
    NonzeroExit(String),
    SigTerm,
    Utf8(String),
    Parse(String),
}

impl From<io::Error> for OutputError {
    fn from(e: io::Error) -> Self {
        let message = format!("{}", e);
        OutputError::Io(message)
    }
}

impl From<Utf8Error> for OutputError {
    fn from(e: Utf8Error) -> Self {
        let message = format!("Invalid UTF-8 byte at index {}", e.valid_up_to());
        OutputError::Utf8(message)
    }
}

pub fn run(name: &str, args: Vec<&str>) -> Result<Output, OutputError> {
    use std::str::from_utf8;
    let output = match Command::new(name).args(args).output() {
        Ok(x) => x,
        Err(e) => return Err(OutputError::from(e)),
    };

    match output.status.code() {
        Some(0) => Ok(output),
        Some(code) => {
            if let Ok(text) = from_utf8(output.stderr.as_slice()) {
                let message = format!("exit {}: {}", code, String::from(text).trim());
                Err(OutputError::NonzeroExit(message))
            } else {
                Err(OutputError::NonzeroExit(String::new()))
            }
        }
        None => Err(OutputError::SigTerm),
    }
}

pub fn dump(output: Output) -> Result<String, OutputError> {
    use std::str::from_utf8;
    let text = from_utf8(output.stdout.as_slice())?;
    Ok(String::from(text))
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
            let message = format!("{} ({})", e.to_string(), text);
            Err(OutputError::Parse(message))
        }
    }
}
