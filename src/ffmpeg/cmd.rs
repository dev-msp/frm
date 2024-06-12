// use super::filter::Filter;
use super::proc;
use super::proc::OutputError;
use std::fmt::{self, Debug};
use std::path::PathBuf;
use std::process::Output;

pub trait Command: Debug {
    fn build(&self) -> Vec<String>;
    #[tracing::instrument(skip_all)]
    fn execute(&self) -> Result<Output, OutputError> {
        let args = self.build();
        proc::run("ffmpeg", args)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Error,
}

#[derive(Debug, Clone)]
pub enum Destination {
    Stdout,
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub enum FormatKind {
    Png,
    Gif,
    Jpeg,
    Srt,
}

#[derive(Debug, Clone)]
pub enum Dim {
    H(u32),
    W(u32),
}

#[derive(Debug, Clone)]
pub enum CommandOption {
    Positional(String),
    Named(String, String),
    LogLevel(Level),
    Position(usize),
    Duration(usize),
    Input(String),
    Scale(Dim),
    Frames(usize),
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
            FormatKind::Png => write!(f, "apng"),
            FormatKind::Gif => write!(f, "gif"),
            FormatKind::Jpeg => write!(f, "singlejpeg"),
            FormatKind::Srt => write!(f, "srt"),
        }
    }
}

impl CommandOption {
    pub fn process_option(self) -> Vec<String> {
        use CommandOption::*;
        let (k, v) = match self {
            LogLevel(level) => (Some("-loglevel".into()), format!("{}", level)),
            Position(ms) => (Some("-ss".into()), format!("{:.3}", (ms as f64) / 1000.0)),
            Duration(ms) => (Some("-t".into()), format!("{:.3}", (ms as f64) / 1000.0)),
            Input(p) => (Some("-i".into()), p),
            Frames(n) => (Some("-vframes".into()), format!("{}", n)),
            // Filter(filter) => (Some("-vf".into()), filter.into()),
            Scale(Dim::W(n)) => (Some("-vf".into()), format!("scale={}:-1", n)),
            Scale(Dim::H(n)) => (Some("-vf".into()), format!("scale=-1:{}", n)),
            Format(format) => (Some("-f".into()), format!("{}", format)),
            Output(Destination::Stdout) => (None, String::from("-")),
            Output(Destination::Path(s)) => (None, s.to_string_lossy().to_string()),
            Positional(arg) => (None, arg),
            Named(name, arg) => (Some(name), arg),
        };
        let mut out = Vec::new();
        if let Some(k) = k {
            out.push(k);
        }
        out.push(v);
        out
    }
}
