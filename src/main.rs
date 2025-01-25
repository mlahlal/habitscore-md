use cli::parse_args;
use std::process::exit;
use file::HabitFile;

mod cli;
mod errors;
mod file;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            log::error!("Error while parsing arguments: \n\t{}", e);
            exit(e.get_retcode());
        }
    };

    let mut habitfile = HabitFile::new(args);

    match habitfile.read() {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error in reading file: \n\t{}", e);
            exit(e.get_retcode());
        }
    }
}
