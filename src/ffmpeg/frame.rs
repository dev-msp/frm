use super::path;
use super::proc;
use super::ErrorKind;
use std::path::PathBuf;

pub struct Frame {
    timecode: u32,
    input: PathBuf,
}

impl Frame {
    pub fn new(input: &str, timecode: u32) -> Result<Self, ErrorKind> {
        path::existing_path(input)
            .map(|input_path| input_path.to_owned())
            .and_then(|input_path| {
                Ok(Frame {
                    timecode: timecode,
                    input: input_path,
                })
            })
    }

    pub fn write(&self, output: &str) -> Result<String, ErrorKind> {
        let path = path::non_existing_path(output)?;
        let timecode_str = self.timecode.to_string();
        let args = vec![
            "-loglevel",
            "error",
            "-ss",
            timecode_str.as_str(),
            "-i",
            self.input.to_str().ok_or(ErrorKind::PathNotUnicode)?,
            "-vframes",
            "1",
            path.to_str().ok_or(ErrorKind::PathNotUnicode)?,
        ];
        proc::run("ffmpeg", args)
            .and_then(|output| proc::dump(output))
            .map_err(|e| ErrorKind::Output(e))
    }
}
