use std::{error, fmt};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct TokenizeError {
    pub filename: Rc<Path>,
    pub pos: usize,
    pub len: usize,
    errstr: String,
}

impl TokenizeError {
    pub fn new(filename: Rc<Path>, pos: usize, len: usize, errstr: String) -> TokenizeError {
        TokenizeError {
            filename,
            pos,
            len,
            errstr,
        }
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errstr)
    }
}

impl error::Error for TokenizeError {
    fn description(&self) -> &str {
        &self.errstr
    }
}
