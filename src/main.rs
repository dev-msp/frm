extern crate clap;
extern crate handlebars;
extern crate rayon;
#[macro_use]
extern crate warp;
#[macro_use]
extern crate serde_json;
extern crate serde;

mod ffmpeg;
mod server;

use ffmpeg::ErrorKind as FfmpegError;

type CommandResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn handle_serve(matches: &clap::ArgMatches) -> CommandResult {
    match matches.value_of("INPUT") {
        Some(file_path) => {
            server::FrameServer::new(file_path).serve();
            Ok(())
        }
        None => Err(Box::new(FfmpegError::ArgumentError)),
    }
}

fn main() -> CommandResult {
    use clap::{App, Arg, SubCommand};

    let ref input_arg = Arg::with_name("INPUT")
        .required(true)
        .help("Sets the input file to use");

    let app_m = App::new("frm")
        .subcommand(SubCommand::with_name("serve").arg(input_arg))
        .get_matches();

    match app_m.subcommand() {
        ("serve", Some(sub_m)) => handle_serve(sub_m),
        _ => Err(Box::new(FfmpegError::ArgumentError)),
    }
}
