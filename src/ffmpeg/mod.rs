pub mod cmd;
mod error;
// mod filter;
pub mod frame;
pub mod frame_range;
mod path;
pub mod proc;
pub mod sequence;
pub mod subtitle;

use std::time::Duration;

pub use error::ErrorKind;

pub fn duration(path_str: &str) -> Result<Duration, ErrorKind> {
    let path = path::existing_path(path_str)?;

    let args = vec![
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        path.to_str().unwrap(),
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    let output = proc::run("ffprobe", args)?;
    match proc::parse_from_output::<f32>(output) {
        Ok(secs) if secs > 0_f32 => Ok(Duration::from_millis((secs * 1000.0).floor() as u64)),
        Ok(_) => Err(ErrorKind::Unhandled(
            "Weirdly small/negative duration".into(),
        )),

        Err(e) => Err(ErrorKind::Output(e)),
    }
}
