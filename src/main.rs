extern crate clap;
extern crate rayon;
extern crate warp;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate serde;

mod ffmpeg;
mod server;

use ffmpeg::ErrorKind as FfmpegError;

type CommandResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn handle_serve(_: &clap::ArgMatches) -> CommandResult {
    server::serve();
    Ok(())
}

fn handle_duration(matches: &clap::ArgMatches) -> CommandResult<f32> {
    match matches.value_of("INPUT") {
        Some(file_path) => Ok(ffmpeg::duration(&String::from(file_path))?),
        None => Err(Box::new(FfmpegError::ArgumentError)),
    }
}

fn handle_sample(matches: &clap::ArgMatches) -> CommandResult {
    use ffmpeg::sample;

    let input = matches.value_of("INPUT").map(|i| String::from(i)).unwrap();
    let n: u32 = matches
        .value_of("FRAMES")
        .map(|s| s.parse())
        .expect("required arg FRAMES missing")
        .map_err(|_| FfmpegError::ArgumentError)?;

    let start: u32 = matches
        .value_of("START")
        .map(|s| s.parse())
        .expect("required arg START missing")
        .map_err(|_| FfmpegError::ArgumentError)?;

    let end: Option<u32> = matches.value_of("END").map(|s| s.parse()).transpose()?;

    sample::sample_video(&input, &sample::SampleWindow { start, end, n })?
        .into_iter()
        .map(|(k, mut v)| v.write_file(format!("output_{}.jpeg", k)))
        .fold(Ok(()), |acc, next| acc.and(next))?;

    Ok(())
}

fn main() -> CommandResult {
    use clap::{App, Arg, SubCommand};
    let app_m = App::new("frm")
        .subcommand(
            SubCommand::with_name("serve")
        )
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
                    Arg::with_name("START")
                        .short("-s")
                        .takes_value(true)
                        .default_value("0")
                        .help("timecode at which to start collecting frames"),
                )
                .arg(
                    Arg::with_name("END")
                        .short("-e")
                        .takes_value(true)
                        .help("timecode at which to stop collecting frames"),
                )
                .arg(
                    Arg::with_name("FRAMES")
                        .short("-n")
                        .takes_value(true)
                        .default_value("10")
                        .help("timecode at which to start collecting frames"),
                ),
        )
        .get_matches();

    match app_m.subcommand() {
        ("serve", Some(sub_m)) => { handle_serve(sub_m) },
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
