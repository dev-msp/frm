use super::proc;
use super::proc::OutputError;
use std::fmt;
use std::process::Output;

pub trait Command {
    fn build(&self) -> Vec<String>;
    fn execute(&self) -> Result<Output, OutputError> {
        let args = self.build();
        proc::run("ffmpeg", args)
    }
}

pub enum Level {
    Error,
}

pub enum Destination {
    Stdout,
}

pub enum FormatKind {
    Png,
    Gif,
    Jpeg,
}

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
            FormatKind::Png => write!(f, "apng"),
            FormatKind::Gif => write!(f, "gif"),
            FormatKind::Jpeg => write!(f, "singlejpeg"),
        }
    }
}

impl CommandOption {
    pub fn process_option(self) -> Vec<String> {
        use CommandOption::*;
        let (k, v) = match self {
            LogLevel(level) => (Some("-loglevel".into()), format!("{}", level)),
            Position(ms) => (Some("-ss".into()), format!("{:.3}", (ms as f64) / 1000.0)),
            Input(p) => (Some("-i".into()), p),
            Frames(n) => (Some("-vframes".into()), format!("{}", n)),
            Scale(Dim::W(n)) => (Some("-vf".into()), format!("scale={}:-1", n)),
            Scale(Dim::H(n)) => (Some("-vf".into()), format!("scale=-1:{}", n)),
            Format(format) => (Some("-f".into()), format!("{}", format)),
            Output(Destination::Stdout) => (None, String::from("-")),
        };
        let mut out = Vec::new();
        if let Some(k) = k {
            out.push(k);
        }
        out.push(v);
        out
    }
}
