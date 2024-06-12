mod ffmpeg;
mod server;
// mod span;

use std::{env::current_dir, fs, io};

use ffmpeg::ErrorKind as FfmpegError;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

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
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(write_to_log)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_max_level(Level::TRACE)
        .with_level(false)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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

const LOG_PATH: &str = "development.log";

fn write_to_log() -> impl io::Write {
    let pwd = current_dir().expect("couldn't get current dir");
    fs::File::options()
        .append(true)
        .open(pwd.join(LOG_PATH))
        .expect("failed to create file")
}
