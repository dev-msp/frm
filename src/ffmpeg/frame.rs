use super::cmd::*;
use super::path::{existing_path, non_existing_path, Error as PathError};
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
            Scale(Dim::W(480)),
            Format(FormatKind::JPEG),
            Output(Destination::Stdout),
        ]
        .into_iter()
        .map(|o| o.process_option())
        .flatten()
        .collect()
    }
}

impl<'a> Frame<'a> {
    pub fn new(input: &'a str, timecode: u32) -> Result<Self, ErrorKind> {
        existing_path(input)
            .and_then(|input_path| input_path.to_str().ok_or(PathError::PathNotUnicode))
            .map_err(ErrorKind::from)
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

    pub fn read(&mut self) -> Result<(), ErrorKind> {
        if self.has_data() {
            return Ok(());
        }
        let data = self.execute()?.stdout;
        self.data = Some(ImageData::new(data));
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write(mut self) -> Result<Vec<u8>, ErrorKind> {
        self.read()?;
        match self.data {
            Some(ImageData { data }) => Ok(data),
            None => panic!("write did not succeed"),
        }
    }

    #[allow(dead_code)]
    pub fn write_file(&mut self, output: String) -> Result<(), ErrorKind> {
        let path = non_existing_path(&output)?;
        self.read()?;
        match &self.data {
            Some(ImageData { data }) => proc::write_to_file(path, data).map_err(ErrorKind::from),
            None => panic!("write did not succeed"),
        }
    }
}
