use super::proc;
use super::ErrorKind;
use std::fmt;
use std::process::Output;

pub trait Command {
    fn build(&self) -> Vec<String>;
    fn execute(&self) -> Result<Output, ErrorKind> {
        let args = self.build();
        proc::run("ffmpeg", args).map_err(|e| ErrorKind::Output(e))
    }
}

pub enum Level {
    Error,
}

pub enum Destination {
    Stdout,
}

#[allow(dead_code)]
pub enum FormatKind {
    PNG,
    GIF,
    JPEG,
}

#[allow(dead_code)]
pub enum Dim {
    H(u32),
    W(u32),
}

pub enum CommandOption {
    LogLevel(Level),
    Position(u32),
    Input(String),
    Scale(Dim),
    Frames(u32),
    Format(FormatKind),
    Output(Destination),
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Error => write!(f, "error"),
        }
    }
}

impl fmt::Display for FormatKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatKind::PNG => write!(f, "apng"),
            FormatKind::GIF => write!(f, "gif"),
            FormatKind::JPEG => write!(f, "singlejpeg"),
        }
    }
}

impl CommandOption {
    pub fn process_option(self) -> Vec<String> {
        use CommandOption::*;
        let (k, v) = match self {
            LogLevel(level) => (Some("-loglevel"), format!("{}", level)),
            Position(secs) => (Some("-ss"), format!("{}", secs)),
            Input(p) => (Some("-i"), p),
            Frames(n) => (Some("-vframes"), format!("{}", n)),
            Scale(Dim::W(n)) => (Some("-vf"), format!("scale={}:-1", n)),
            Scale(Dim::H(n)) => (Some("-vf"), format!("scale=-1:{}", n)),
            Format(format) => (Some("-f"), format!("{}", format)),
            Output(Destination::Stdout) => (None, String::from("-")),
        };
        let mut out = Vec::new();
        if let Some(k) = k {
            out.push(String::from(k));
        }
        out.push(v);
        out
    }
}
