use std::fmt;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    FormatInvalid(String),
    FileError(String),
    ChartError(String),
    UnknownError(String),
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            Errcode::FormatInvalid(element) => write!(f, "FormatInvalid: {}", element),
            Errcode::FileError(element) => write!(f, "FileError: {}", element),
            Errcode::ChartError(element) => write!(f, "ChartError: {}", element),
            Errcode::UnknownError(element) => write!(f, "UnknownError: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}
