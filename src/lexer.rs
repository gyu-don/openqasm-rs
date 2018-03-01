use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::path::Path;
use std::u8;
use tokens::{self, Token, TokenType};

pub fn tokenize<P: AsRef<Path> + Debug>(code: &str, filename: P) -> Vec<Token<P>> {
    // dummy
    vec![]
}

#[derive(Debug)]
struct SplitToken<'a, P: AsRef<Path> + Debug> {
    code: &'a str,
    filename: P,
    pos: usize,
    cols: usize,
    rows: usize,
}

#[derive(Debug)]
struct TokenizeError<P: AsRef<Path> + Debug> {
    pub filename: P,
    pub pos: usize,
    pub len: usize,
    pub cols: usize,
    pub rows: usize,
    errstr: String,
}
impl<P: AsRef<Path> + Debug> TokenizeError<P> {
    fn new(filename: P, pos: usize, len: usize, cols: usize, rows: usize, errmsg: &str) -> TokenizeError<P> {
        TokenizeError {
            filename, pos, len, cols, rows, errstr: format!("{}:{}:{} {}", filename.as_ref().to_string_lossy(), cols, rows, errmsg)
        }
    }
}
impl<P: AsRef<Path> + Debug> Display for TokenizeError<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errstr)
    }
}
impl<P: AsRef<Path> + Debug> Error for TokenizeError<P> {
    fn description(&self) -> &str {
        &self.errstr
    }
}

impl<'a, P: AsRef<Path> + Debug> SplitToken<'a, P> {
    fn new(code: &'a str, filename: P) -> SplitToken<'a, P> {
        SplitToken { code, filename, pos: 0, cols: 0, rows: 0 }
    }
}

impl<'a, P: AsRef<Path> + Debug> Iterator for SplitToken<'a, P> {
    type Item = Result<Token<P>, TokenizeError<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.code.len() { return None }
        enum LexerState {
            Whitespace,
            Identifier,
            Integer,
            Real,
            Comment,
            Punctuation,
        }

        let code = self.code.as_bytes();
        let mut state = LexerState::Whitespace;
        let mut cur = self.pos;
        while cur < self.code.len() {
            let ch = code[cur];
            match state {
                LexerState::Whitespace => {
                    if ch.is_ascii_whitespace() {
                        if ch == b'\n' { self.rows += 1; self.cols = 0; } else { self.cols += 1; }
                        self.pos += 1;
                    } else if ch.is_ascii_digit() {
                        state = LexerState::Integer;
                    } else if ch.is_ascii_alphabetic() {
                        state = LexerState::Identifier;
                    } else if ch.is_ascii_punctuation() {
                        state = LexerState::Punctuation;
                    } else {
                        return Some(Err(TokenizeError::new(self.filename, self.pos, 1, self.cols, self.rows, "Unexpected character.")));
                    }
                },
                LexerState::Punctuation => {
                    // TODO: 書く
                }
            }
            cur += 1;
        }
        //dummy
        None
    }
}
