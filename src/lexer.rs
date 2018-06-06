use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::{Path, PathBuf};
use std::str;

use errors::TokenizeError;
use tokens::{self, Token, TokenType};

pub type TokenResult<'a> = Result<Token<'a>, TokenizeError<'a>>;

pub struct RecursiveTokenizer<'a> {
    codes: HashMap<PathBuf, String>,
    stack: Vec<Box<Iterator<Item=TokenResult<'a>>>>
}

impl<'a> RecursiveTokenizer<'a> {
    pub fn new(filename: &Path) -> RecursiveTokenizer {
        let mut rtok = RecursiveTokenizer::get_empty();
        if let Err(_) = rtok.load(filename.to_path_buf()) {
        }
        rtok
    }

    fn get_empty() -> RecursiveTokenizer<'a> {
        RecursiveTokenizer {
            codes: HashMap::new(),
            stack: Vec::new()
        }
    }

    fn load(&mut self, filename: PathBuf) -> io::Result<()> {
        if self.codes.contains_key(&filename) {
            return Ok(());
        }

        let f = File::open(&filename)?;
        let mut f = io::BufReader::new(f);
        let mut code = String::new();
        f.read_to_string(&mut code)?;
        self.codes.insert(filename, code);
        self.stack.push(Box::new(TokenIterator::from_owned(&self.codes[&filename], &self.codes.entry(filename).key())));
        Ok(())
    }
}

pub fn tokenize<'a>(code: &'a str, filename: &'a Path) -> TokenIterator<'a> {
    TokenIterator {
        code,
        filename,
        pos: 0,
    }
}

#[derive(Debug)]
pub struct TokenIterator<'a> {
    code: &'a str,
    filename: &'a Path,
    pos: usize,
}

pub fn filter_comment<'a>(tokenresults: impl Iterator<Item=TokenResult<'a>>) -> impl Iterator<Item=TokenResult<'a>> {
    tokenresults.filter(|r| r.as_ref().map(|t| !t.is_comment()).unwrap_or(true))
}

impl<'a> TokenIterator<'a> {
    pub fn new(code: &'a str, filename: &'a Path) -> TokenIterator<'a> {
        tokenize(code, filename)
    }

    pub fn from_owned(code: &'a String, filename: &'a PathBuf) -> TokenIterator<'a> {
        TokenIterator {
            code: code.as_ref(),
            filename: filename.as_ref(),
            pos: 0
        }
    }

    pub fn filter_comment(self) -> impl Iterator<Item=TokenResult<'a>> {
        filter_comment(self)
    }

    fn get_token(&mut self) -> Option<TokenResult<'a>> {
        #[derive(Debug)]
        enum LexerState {
            Whitespace,
            Identifier,
            Integer,
            Real,
            StringLiteral,
            Comment,
            Punctuation,
        }

        let code = self.code.as_bytes();
        if self.pos >= code.len() {
            return None;
        }
        let mut state = LexerState::Whitespace;
        let mut cur = self.pos;
        while cur < code.len() {
            let ch = code[cur];
            // Debug
            // println!("pos: {}, cur: {}, state: {:?}, ch: {}",
            //          self.pos,
            //          cur,
            //          state,
            //          char::from(ch));
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
                        let pos = self.pos;
                        self.pos += 1;
                        return Some(Err(TokenizeError::new(self.filename,
                                                           pos,
                                                           1,
                                                           "Unexpected character.".to_owned())));
                    }
                }
                LexerState::Punctuation => {
                    if code[self.pos] == b'=' {
                        assert_eq!(self.pos + 1, cur);
                        if ch == b'=' {
                            let tok = Token {
                                token: TokenType::DoubleEqual,
                                filename: Some(self.filename.as_ref()),
                                pos: self.pos,
                                len: 2,
                            };
                            self.pos += 2;
                            return Some(Ok(tok));
                        } else {
                            let pos = self.pos;
                            self.pos += 1;
                            return Some(Err(TokenizeError::new(self.filename,
                                                               pos,
                                                               1,
                                                               "Unexpected character.".to_owned())));
                        }
                    } else if code[self.pos] == b'-' {
                        assert_eq!(self.pos + 1, cur);
                        if code[cur] == b'>' {
                            let tok = Token {
                                token: TokenType::Arrow,
                                filename: Some(self.filename.as_ref()),
                                pos: self.pos,
                                len: 2,
                            };
                            self.pos += 2;
                            return Some(Ok(tok));
                        } else {
                            let tok = Token {
                                token: TokenType::Minus,
                                filename: Some(self.filename.as_ref()),
                                pos: self.pos,
                                len: 1,
                            };
                            self.pos += 1;
                            return Some(Ok(tok));
                        }
                    } else if code[self.pos] == b'/' {
                        assert_eq!(self.pos + 1, cur);
                        if code[cur] == b'/' {
                            state = LexerState::Comment;
                            self.pos += 2;
                            cur = self.pos;
                            continue;
                        } else {
                            let tok = Token {
                                token: TokenType::Devide,
                                filename: Some(self.filename.as_ref()),
                                pos: self.pos,
                                len: 1,
                            };
                            self.pos += 1;
                            return Some(Ok(tok));
                        }
                    } else if code[self.pos] == b'"' {
                        state = LexerState::StringLiteral;
                        self.pos += 1;
                        cur = self.pos;
                        continue;
                    } else {
                        match tokens::match_keyword_exact(&code[self.pos..self.pos + 1]) {
                            Some(kwd) => {
                                let tok = Token {
                                    token: kwd,
                                    filename: Some(self.filename.as_ref()),
                                    pos: self.pos,
                                    len: 1,
                                };
                                self.pos += 1;
                                return Some(Ok(tok));
                            }
                            None => {
                                let pos = self.pos;
                                self.pos += 1;
                                return Some(Err(TokenizeError::new(self.filename,
                                                                   pos,
                                                                   1,
                                                                   "Unexpected character.".to_owned())));
                            }
                        }
                    }
                    // TODO: 書く
                }
                LexerState::Identifier => {
                    if ch.is_ascii_digit() || ch.is_ascii_alphabetic() || ch == b'_' {
                        // nothing.
                    } else {
                        let toktype = tokens::match_keyword_exact(&code[self.pos..cur])
                            .unwrap_or(TokenType::Identifier(String::from_utf8(code[self.pos..
                                                                               cur]
                                    .to_owned())
                                .unwrap()));
                        let tok = Token {
                            token: toktype,
                            filename: Some(self.filename.as_ref()),
                            pos: self.pos,
                            len: cur - self.pos,
                        };
                        self.pos = cur;
                        return Some(Ok(tok));
                    }
                }
                LexerState::Integer => {
                    if ch.is_ascii_digit() {
                        // nothing.
                    } else if ch == b'.' {
                        state = LexerState::Real;
                    } else {
                        let tok = Token {
                            token: TokenType::UInt(str::from_utf8(&code[self.pos..cur])
                                .unwrap()
                                .parse()
                                .unwrap()),
                            filename: Some(self.filename.as_ref()),
                            pos: self.pos,
                            len: cur - self.pos,
                        };
                        self.pos = cur;
                        return Some(Ok(tok));
                    }
                }
                LexerState::Real => {
                    if ch.is_ascii_digit() {
                        // nothing.
                    } else {
                        let tok = Token {
                            token: TokenType::Real(str::from_utf8(&code[self.pos..cur])
                                .unwrap()
                                .parse()
                                .unwrap()),
                            filename: Some(self.filename.as_ref()),
                            pos: self.pos,
                            len: cur - self.pos,
                        };
                        self.pos = cur;
                        return Some(Ok(tok));
                    }
                }
                LexerState::StringLiteral => {
                    if ch != b'"' {
                        // nothing.
                    } else {
                        match String::from_utf8(code[self.pos..cur].to_owned()) {
                            Ok(s) => {
                                let tok = Token {
                                    token: TokenType::StringLiteral(s),
                                    filename: Some(self.filename.as_ref()),
                                    pos: self.pos,
                                    len: cur - self.pos,
                                };
                                self.pos = cur + 1;
                                return Some(Ok(tok));
                            }
                            Err(e) => {
                                return Some(Err(TokenizeError::new(self.filename,
                                                                   self.pos,
                                                                   1,
                                                                   e.description().to_owned())));
                            }
                        }
                    }
                }
                LexerState::Comment => {
                    if ch != b'\n' {
                        // nothing.
                    } else {
                        let tok = Token {
                            token: TokenType::Comment(code[self.pos..cur].to_owned()),
                            filename: Some(self.filename.as_ref()),
                            pos: self.pos,
                            len: cur - self.pos,
                        };
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

impl<'a> Iterator for TokenIterator<'a> {
    type Item = TokenResult<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_token()
    }
}
