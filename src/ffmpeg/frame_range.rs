use std::fs::rename;
use std::path::PathBuf;

use tracing::info;

use super::cmd::*;
use super::path::existing_path;
use super::ErrorKind;

#[derive(Debug, Clone)]
pub struct FrameRange {
    origin: PathBuf,
    cache_root: PathBuf,
    start: usize,
    n: usize,
    step: usize,
}

#[derive(Debug, Clone)]
struct FrameAt {
    timecode: usize,
    origin: PathBuf,
    encoding: FormatKind,
    cache_root: PathBuf,
}

impl FrameAt {
    fn cache_path(&self) -> Destination {
        Destination::Path(self.cache_root.join(format!("{}.png", self.timecode)))
    }
}

impl From<FrameAt> for Vec<String> {
    fn from(f: FrameAt) -> Self {
        use CommandOption::*;
        let output = f.cache_path();
        vec![
            Position(f.timecode),
            Input(f.origin.to_string_lossy().to_string()),
            Frames(1),
            Scale(Dim::W(640)),
            Format(f.encoding),
            Output(output),
        ]
        .into_iter()
        .flat_map(|o| o.process_option())
        .collect()
    }
}

impl Command for FrameRange {
    #[tracing::instrument(skip_all)]
    fn build(&self) -> Vec<String> {
        use CommandOption::*;

        let total_duration = (self.n - 1) * self.step;

        let fps = {
            let fps = self.step as f32 / 1000.0;
            info!("fps: {fps}");
            if fps < 1.0 {
                format!("{}", (1.0 / fps) as usize)
            } else {
                format!("1/{}", fps as usize)
            }
        };
        info!("string fps: {fps}");

        let out: Vec<_> = vec![
            LogLevel(Level::Error),
            Position(self.start),
            Duration(total_duration),
            Input(self.origin.to_string_lossy().to_string()),
            // Frames(1),
            // Scale(Dim::W(640)),
            // Format(self.encoding.clone()),
            Named("-vf".into(), format!("fps={fps},scale=640:-1")),
            Output(Destination::Path(self.cache_root.join("%04d.png"))),
        ]
        .into_iter()
        .flat_map(|o| o.process_option())
        .collect();

        info!("{}", out.join(" "));

        out
    }
}

impl FrameRange {
    pub fn new(
        origin: &str,
        cache_root: &str,
        start: usize,
        n: usize,
        step: usize,
    ) -> Result<Self, ErrorKind> {
        Ok(FrameRange {
            origin: existing_path(origin)?.to_path_buf(),
            cache_root: existing_path(cache_root)?.to_path_buf(),
            start,
            n,
            step,
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn read(&mut self) -> Result<Vec<usize>, ErrorKind> {
        let mut output = Vec::new();
        match self.execute()?.status.code() {
            Some(0) => {}
            Some(code) => return Err(ErrorKind::Unhandled(format!("Non-zero exit code {code}"))),
            None => return Err(ErrorKind::Unhandled("Failed without exit code".into())),
        };

        for n in 0..self.n - 1 {
            let timecode = self.start + n * self.step;
            let pb = self.cache_root.join(format!("{:04}.png", n + 1));
            if !pb.exists() {
                return Err(ErrorKind::Unhandled(format!(
                    "expected all range files to exist, missing {}",
                    pb.to_string_lossy()
                )));
            }

            rename(
                pb.clone(),
                pb.parent()
                    .expect("path is neither blank nor /")
                    .to_path_buf()
                    .join(format!("{timecode}.png")),
            )?;

            output.push(timecode);
        }

        Ok(output)
    }
}
