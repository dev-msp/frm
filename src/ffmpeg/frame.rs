use std::io;
use std::path::PathBuf;

use super::cmd::*;
use super::path::non_existing_path;
use super::proc;
use super::ErrorKind;

#[derive(Debug, Clone)]
pub struct ImageData {
    data: Vec<u8>,
}

impl ImageData {
    pub fn new(data: Vec<u8>) -> ImageData {
        ImageData { data }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    timecode: usize,
    data: Option<ImageData>,
    origin: PathBuf,
    encoding: FormatKind,
}

impl Command for Frame {
    fn build(&self) -> Vec<String> {
        use CommandOption::*;
        vec![
            LogLevel(Level::Error),
            Position(self.timecode),
            Input(self.origin.to_string_lossy().to_string()),
            Frames(1),
            Scale(Dim::W(640)),
            Format(self.encoding.clone()),
            Output(Destination::Stdout),
        ]
        .into_iter()
        .flat_map(|o| o.process_option())
        .collect()
    }
}

impl Frame {
    pub fn new<I: Into<PathBuf>>(input: I, timecode: usize) -> Result<Self, ErrorKind> {
        let path = input.into();
        if path.exists() {
            Ok(Frame {
                timecode: timecode.max(1),
                data: None,
                origin: path,
                encoding: FormatKind::Png,
            })
        } else {
            Err(ErrorKind::Io(io::ErrorKind::NotFound.into()))
        }
    }

    pub fn timecode(&self) -> usize {
        self.timecode
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(ImageData::new(data))
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
    pub fn write(&mut self) -> Result<Vec<u8>, ErrorKind> {
        self.read()?;
        match self.data.clone() {
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
