extern crate clap;
extern crate rayon;

mod ffmpeg;

use ffmpeg::ErrorKind as FfmpegError;

type CommandResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn handle_duration(matches: &clap::ArgMatches) -> CommandResult<f32> {
    match matches.value_of("INPUT") {
        Some(file_path) => Ok(ffmpeg::duration(&String::from(file_path))?),
        None => Err(Box::new(FfmpegError::ArgumentError)),
    }
}

fn handle_sample(matches: &clap::ArgMatches) -> CommandResult {
    use ffmpeg::sample;

    let input = matches.value_of("INPUT").map(|i| String::from(i)).unwrap();
    if let Some(frames) = matches.value_of("FRAMES") {
        let f = frames.parse()?;
        sample::sample_video(input, f)?;
        Ok(())
    } else {
        Err(Box::new(FfmpegError::ArgumentError))
    }
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
            SubCommand::with_name("sample")
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .help("Sets the input file to use"),
                )
                .arg(
                    Arg::with_name("FRAMES")
                        .short("-n")
                        .takes_value(true)
                        .default_value("10")
                        .help("count of frames to sample"),
                ),
        )
        .get_matches();

    match app_m.subcommand() {
        ("duration", Some(sub_m)) => {
            println!("{}", handle_duration(sub_m)?);
            Ok(())
        }
        ("sample", Some(sub_m)) => {
            println!("{}", handle_sample(sub_m).map(|_| "Good")?);
            Ok(())
        }
        _ => Err(Box::new(FfmpegError::ArgumentError)),
    }
}
