pub mod cmd;
mod error;
pub mod frame;
mod path;
pub mod proc;
pub mod sample;

pub use error::ErrorKind;

pub fn duration(path_str: &String) -> Result<f32, ErrorKind> {
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
    .map(|s| String::from(s))
    .collect::<Vec<_>>();

    let output = proc::run("ffprobe", args)?;
    match proc::parse_from_output(output) {
        Ok(duration) => Ok(duration),
        Err(e) => Err(ErrorKind::Output(e)),
    }
}
