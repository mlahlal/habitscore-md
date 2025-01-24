use std::fmt;
use std::process::exit;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    FormatInvalid(u8),
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            exit(0);
        },
        Err(e) => {
            let retcode = e.get_retcode();
            exit(retcode);
        }
    }
}
