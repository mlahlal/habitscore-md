use crate::errors::Errcode;

use std::path::PathBuf;
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "file path")]
    pub path: PathBuf,

    #[options(help = "show the tasks not done many times")]
    pub stat: bool,

    #[options(help = "debug mode")]
    pub debug: bool,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse_args_default_or_exit();

    setup_log(log::LevelFilter::Debug);

    if !args.path.exists() || !args.path.is_file() {
        return Err(Errcode::ArgumentInvalid("path"));
    }

    Ok(args)
}

fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
