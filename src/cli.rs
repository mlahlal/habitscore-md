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
}

pub fn parse_args() -> Result<Args, Errcode> {
    let opts = Args::parse_args_default_or_exit();

    setup_log(log::LevelFilter::Info);

    if !opts.path.exists() || !opts.path.is_file() {
        return Err(Errcode::ArgumentInvalid("path"));
    }

    Ok(opts)
}

fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
