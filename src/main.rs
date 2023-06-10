mod ffmpeg;
mod server;
// mod span;

use ffmpeg::ErrorKind as FfmpegError;

type CommandResult<T = ()> = Result<T, FfmpegError>;

async fn handle_serve(matches: &clap::ArgMatches<'_>) -> CommandResult {
    match matches.value_of("INPUT") {
        Some(file_path) => {
            server::FrameServer::new(file_path.to_string())?
                .serve()
                .await;
            Ok(())
        }
        None => Err(FfmpegError::ArgumentError),
    }
}

#[tokio::main]
async fn main() -> CommandResult {
    use clap::{App, Arg, SubCommand};

    let input_arg = &Arg::with_name("INPUT")
        .required(true)
        .help("Sets the input file to use");

    let app_m = App::new("frm")
        .subcommand(SubCommand::with_name("serve").arg(input_arg))
        .get_matches();

    match app_m.subcommand() {
        ("serve", Some(sub_m)) => handle_serve(sub_m).await,
        _ => Err(FfmpegError::ArgumentError),
    }
}
