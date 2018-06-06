use std::{error, fmt};
use std::path::Path;

#[derive(Debug)]
pub struct TokenizeError<'a> {
    pub filename: &'a Path,
    pub pos: usize,
    pub len: usize,
    errstr: String,
}

impl<'a> TokenizeError<'a> {
    pub fn new(filename: &'a Path, pos: usize, len: usize, errstr: String) -> TokenizeError<'a> {
        TokenizeError {
            filename,
            pos,
            len,
            errstr,
        }
    }
}

impl<'a> fmt::Display for TokenizeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errstr)
    }
}

impl<'a> error::Error for TokenizeError<'a> {
    fn description(&self) -> &str {
        &self.errstr
    }
}
