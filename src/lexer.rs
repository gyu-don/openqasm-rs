use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
use std::path::Path;
use std::str;
use tokens::{self, Token, TokenType};

pub fn tokenize<P: AsRef<Path> + Debug>(code: &str, filename: P) -> Vec<Token<P>> {
    // dummy
    vec![]
}

#[derive(Debug)]
struct SplitToken<'a, P: 'a + AsRef<Path> + Debug> {
    code: &'a str,
    filename: P,
    pos: usize,
}

#[derive(Debug)]
struct TokenizeError<'a, P: 'a + AsRef<Path> + Debug> {
    pub filename: P,
    pub pos: usize,
    pub len: usize,
    errstr: String,
    _phantom: PhantomData<&'a u8>,
}
impl<'a, P: 'a + AsRef<Path> + Debug> TokenizeError<'a, P> {
    fn new(filename: P, pos: usize, len: usize, errmsg: &str) -> TokenizeError<P> {
        TokenizeError {
            filename, pos, len, errstr: errmsg.to_owned(), _phantom: PhantomData
        }
    }
}
impl<'a, P: 'a + AsRef<Path> + Debug> Display for TokenizeError<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errstr)
    }
}
impl<'a, P: 'a + AsRef<Path> + Debug> Error for TokenizeError<'a, P> {
    fn description(&self) -> &str {
        &self.errstr
    }
}

impl<'a, P: 'a + AsRef<Path> + Debug> SplitToken<'a, P> {
    fn new(code: &'a str, filename: P) -> SplitToken<'a, P> {
        SplitToken { code, filename, pos: 0 }
    }
}

impl<'a, P: 'a + AsRef<Path> + Debug> Iterator for SplitToken<'a, P> {
    type Item = Result<Token<'a, P>, TokenizeError<'a, P>>;
    fn next(&mut self) -> Option<Self::Item> {
        enum LexerState {
            Whitespace,
            Identifier,
            Integer,
            Real,
            Comment,
            Punctuation,
        }

        let code = self.code.as_bytes();
        if self.pos >= code.len() { return None }
        let mut state = LexerState::Whitespace;
        let mut cur = self.pos;
        while cur < code.len() {
            let ch = code[cur];
            match state {
                LexerState::Whitespace => {
                    if ch.is_ascii_whitespace() {
                        self.pos += 1;
                    } else if ch.is_ascii_digit() {
                        state = LexerState::Integer;
                    } else if ch.is_ascii_alphabetic() || ch == b'_' {
                        state = LexerState::Identifier;
                    } else if ch.is_ascii_punctuation() {
                        state = LexerState::Punctuation;
                    } else {
                        return Some(Err(TokenizeError::new(self.filename, self.pos, 1, "Unexpected character.")));
                    }
                },
                LexerState::Punctuation => {
                    if code[self.pos] == b'=' {
                        assert_eq!(self.pos + 1, cur);
                        if ch == b'=' {
                            let tok = Token { token: TokenType::DoubleEqual, filename: Some(&self.filename), pos: self.pos, len: 2 };
                            self.pos += 2;
                            return Some(Ok(tok));
                        } else {
                            return Some(Err(TokenizeError::new(self.filename, self.pos, 1, "Unexpected character.")));
                        }
                    } else if code[self.pos] == b'-' {
                        assert_eq!(self.pos + 1, cur);
                        if code[cur] == b'>' {
                            let tok = Token { token: TokenType::Arrow, filename: Some(&self.filename), pos: self.pos, len: 2 };
                            self.pos += 2;
                            return Some(Ok(tok));
                        } else {
                            let tok = Token { token: TokenType::Minus, filename: Some(&self.filename), pos: self.pos, len: 1 };
                            self.pos += 1;
                            return Some(Ok(tok));
                        }
                    } else {
                        match tokens::match_keyword_exact(&code[self.pos..self.pos + 1]) {
                            Some(kwd) => {
                                let tok = Token { token: kwd, filename: Some(&self.filename), pos: self.pos, len: 1 };
                                self.pos += 1;
                                return Some(Ok(tok));
                            },
                            None => {
                                return Some(Err(TokenizeError::new(self.filename, self.pos, 1, "Unexpected character.")));
                            }
                        }
                    }
                    // TODO: 書く
                }
                LexerState::Identifier => {
                    if ch.is_ascii_digit() || ch.is_ascii_alphabetic() || ch == b'_' {
                        // nothing.
                    } else {
                        let toktype = tokens::match_keyword_exact(&code[self.pos..cur]).unwrap_or(TokenType::Identifier(String::from_utf8(code[self.pos..cur].to_owned()).unwrap()));
                        let tok = Token { token: toktype, filename: Some(&self.filename), pos: self.pos, len: cur - self.pos };
                        self.pos = cur + 1;
                        return Some(Ok(tok));
                    }
                }
                LexerState::Integer => {
                    if ch.is_ascii_digit() {
                        // nothing.
                    } else if ch == b'.' {
                        state = LexerState::Real;
                    } else {
                        let tok = Token { token: TokenType::UInt(str::from_utf8(&code[self.pos..cur]).unwrap().parse().unwrap()), filename: Some(&self.filename), pos: self.pos, len: cur - self.pos };
                        self.pos = cur + 1;
                        return Some(Ok(tok));
                    }
                }
                LexerState::Real => {
                    if ch.is_ascii_digit() {
                        // nothing.
                    } else {
                        let tok = Token { token: TokenType::Real(str::from_utf8(&code[self.pos..cur]).unwrap().parse().unwrap()), filename: Some(&self.filename), pos: self.pos, len: cur - self.pos };
                        self.pos = cur + 1;
                        return Some(Ok(tok));
                    }
                }
                LexerState::Comment => {
                    if ch != b'\n' {
                        // nothing.
                    } else {
                        let tok = Token { token: TokenType::Comment(code[self.pos..cur].to_owned()), filename: Some(&self.filename), pos: self.pos, len: cur - self.pos };
                        self.pos = cur + 1;
                        return Some(Ok(tok));
                    }
                }
            }
            cur += 1;
        }
        None
    }
}
