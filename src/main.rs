extern crate clap;

mod ffmpeg;

use ffmpeg::ErrorKind as FfmpegError;

type CommandResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn handle_duration(matches: &clap::ArgMatches) -> CommandResult<f32> {
    match matches.value_of("INPUT") {
        Some(file_path) => Ok(ffmpeg::duration(file_path)?),
        None => Err(Box::new(FfmpegError::ArgumentError)),
    }
}

fn handle_frame(matches: &clap::ArgMatches) -> CommandResult<String> {
    use ffmpeg::frame::Frame;

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    let timecode_str = matches.value_of("TIMECODE").unwrap();

    let timecode = match timecode_str.parse() {
        Ok(timecode) => timecode,
        Err(e) => return Err(Box::new(e)),
    };

    let s = Frame::new(input, timecode)?.write(output)?;
    Ok(s.clone())
}

fn main() -> CommandResult {
    use clap::{App, Arg, SubCommand};
    let app_m = App::new("frm")
        .subcommand(
            SubCommand::with_name("duration").arg(
                Arg::with_name("INPUT")
                    .required(true)
                    .help("Sets the input file to use"),
            ),
        )
        .subcommand(
            SubCommand::with_name("frame")
                .arg(
                    Arg::with_name("TIMECODE")
                        .short("-t")
                        .takes_value(true)
                        .required(true)
                        .help("the timecode to seek to in INPUT"),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .help("Sets the input file to use"),
                )
                .arg(
                    Arg::with_name("OUTPUT")
                        .short("-o")
                        .takes_value(true)
                        .required(true)
                        .help("Sets the output file to write to"),
                ),
        )
        .get_matches();

    match app_m.subcommand() {
        ("duration", Some(sub_m)) => {
            println!("{}", handle_duration(sub_m)?);
            Ok(())
        }
        ("frame", Some(sub_m)) => {
            println!("{}", handle_frame(sub_m)?);
            Ok(())
        }
        _ => Err(Box::new(FfmpegError::ArgumentError)),
    }
}
