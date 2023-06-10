use std::path::PathBuf;

use super::cmd::*;
use super::path::{existing_path, non_existing_path};
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
    timecode: u32,
    data: Option<ImageData>,
    path: PathBuf,
}

impl Command for Frame {
    fn build(&self) -> Vec<String> {
        use CommandOption::*;
        vec![
            LogLevel(Level::Error),
            Position(self.timecode),
            Input(self.path.to_string_lossy().to_string()),
            Frames(1),
            Scale(Dim::W(640)),
            Format(FormatKind::Png),
            Output(Destination::Stdout),
        ]
        .into_iter()
        .flat_map(|o| o.process_option())
        .collect()
    }
}

impl Frame {
    pub fn new(input: &str, timecode: u32) -> Result<Self, ErrorKind> {
        let path = existing_path(input)?.to_path_buf();
        Ok(Frame {
            timecode: timecode.max(1),
            data: None,
            path,
        })
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    pub fn read(&mut self) -> Result<(), ErrorKind> {
        if self.has_data() {
            // println!("Frame {} benefited from cached data", self.timecode);
            return Ok(());
        }

        // println!("Frame {} did not benefit from cached data", self.timecode);
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
