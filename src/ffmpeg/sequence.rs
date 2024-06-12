use std::path::PathBuf;

use super::cmd::{Command, CommandOption};

#[derive(Debug, Clone)]
pub enum OutputKind {
    Text,
    Video,
    Audio,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    path: PathBuf,
    output: OutputKind,
    start: usize,
    end: usize,
}

impl Sequence {
    pub fn subtitles<P: Into<PathBuf>>(path: P, start: usize, end: usize) -> Self {
        Self {
            path: path.into(),
            output: OutputKind::Text,
            start,
            end,
        }
    }
}
impl Command for Sequence {
    fn build(&self) -> Vec<String> {
        use crate::ffmpeg::cmd::*;
        use CommandOption::*;

        match self.output {
            OutputKind::Text => {}
            OutputKind::Video => unimplemented!(),
            OutputKind::Audio => unimplemented!(),
        }

        vec![
            LogLevel(Level::Error),
            Position(self.start),
            Input(self.path.to_string_lossy().to_string()),
            Duration(self.end - self.start),
            Named("-c:s".into(), "copy".into()),
            Format(FormatKind::Srt),
            Output(Destination::Stdout),
        ]
        .into_iter()
        .flat_map(CommandOption::process_option)
        .collect()
    }
}
