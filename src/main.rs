use cli::parse_args;
use std::process::exit;
use file::HabitFile;

mod cli;
mod errors;
mod file;
mod chart;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            log::error!("{}", e);
            exit(e.get_retcode());
        }
    };

    let mut habitfile = HabitFile::new(args);

    match habitfile.read() {
        Ok(_) => (),
        Err(e) => {
            log::error!("\n{}\n", e);
            exit(e.get_retcode());
        }
    }
}
