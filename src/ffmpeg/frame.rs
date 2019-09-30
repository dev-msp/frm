use super::cmd::*;
use super::path;
use super::proc;
use super::ErrorKind;

pub struct Frame<'a> {
    timecode: u32,
    input: &'a str,
}

impl<'a> Command for Frame<'a> {
    fn build(&self) -> Vec<String> {
        use CommandOption::*;
        let opts = vec![
            LogLevel(Level::Error),
            Position(self.timecode),
            Input(String::from(self.input)),
            Scale(Dim::W(240)),
            Frames(1),
            Format(FormatKind::JPEG),
            Output(Destination::Stdout),
        ]
        .into_iter();

        opts.map(|o| o.process_option())
            .flatten()
            .collect::<Vec<String>>()
    }
}

impl<'a> Frame<'a> {
    pub fn new(input: &'a String, timecode: u32) -> Result<Self, ErrorKind> {
        path::existing_path(input)
            .and_then(|input_path| input_path.to_str().ok_or(ErrorKind::PathNotUnicode))
            .and_then(|input_path| {
                Ok(Frame {
                    timecode: timecode,
                    input: input_path,
                })
            })
    }

    pub fn write(&self, output: String) -> Result<(), ErrorKind> {
        let path = path::non_existing_path(&output)?;
        let cmd_result = self.execute()?;
        proc::write_to_file(path, cmd_result).map_err(|e| ErrorKind::from(e))
    }
}
