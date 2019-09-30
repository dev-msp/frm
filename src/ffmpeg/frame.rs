use super::cmd::*;
use super::path;
use super::proc;
use super::ErrorKind;

pub struct ImageData {
    data: Vec<u8>,
}

impl ImageData {
    pub fn new(data: Vec<u8>) -> ImageData {
        ImageData { data }
    }
}

pub struct Frame<'a> {
    timecode: u32,
    input: &'a str,
    data: Option<ImageData>,
}

impl<'a> Command for Frame<'a> {
    fn build(&self) -> Vec<String> {
        use CommandOption::*;
        vec![
            LogLevel(Level::Error),
            Position(self.timecode),
            Input(String::from(self.input)),
            Frames(1),
            Format(FormatKind::JPEG),
            Output(Destination::Stdout),
        ]
        .into_iter()
        .map(|o| o.process_option())
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
                    data: None,
                })
            })
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    pub fn write(&mut self) -> Result<(), ErrorKind> {
        if self.has_data() {
            return Ok(());
        }
        let data = self.execute()?.stdout;
        self.data = Some(ImageData::new(data));
        Ok(())
    }

    pub fn write_file(&mut self, output: String) -> Result<(), ErrorKind> {
        let path = path::non_existing_path(&output)?;
        self.write()?;
        match &self.data {
            Some(ImageData { data }) => {
                proc::write_to_file(path, &data).map_err(|e| ErrorKind::from(e))
            }
            None => panic!("write did not succeed"),
        }
    }
}
