use std::path::Path;
use std::str;

use errors::TokenizeError;
use tokens::{self, Token, TokenType};

pub fn tokenize<'a>(code: &'a str, filename: &'a Path) -> TokenIterator<'a> {
    TokenIterator {
        code,
        filename,
        pos: 0,
    }
}

pub type TokenResult<'a> = Result<Token<'a>, TokenizeError<'a>>;

#[derive(Debug)]
pub struct TokenIterator<'a> {
    code: &'a str,
    filename: &'a Path,
    pos: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn filter_comment(self) -> impl Iterator<Item=TokenResult<'a>> {
        self.filter(|r| r.as_ref().map(|t| !t.is_comment()).unwrap_or(true))
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
                                                           "Unexpected character.")));
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
                                                               "Unexpected character.")));
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
                                                                   "Unexpected character.")));
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
                                                                   e.description())));
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
