use std::str::Chars;
use std::str::FromStr;
use std::fmt::Display;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::token::Token;

pub type Pos = usize;
pub type Spanned<T> = (Pos, T, Pos);
pub type Result<T> = std::result::Result<T, LexicalError>;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub struct LexicalError {
    lo: Pos,
    hi: Pos,
    message: String,
}

impl LexicalError {
    pub fn new(lo: Pos, hi: Pos, message: String) -> Self {
        Self { lo, hi, message }
    }
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexicalError({}:{}): {}", self.lo, self.hi, self.message)
    }
}

pub struct Lexer<'input> {
    chars: Chars<'input>,
    input_len: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            chars: input.chars(),
            input_len: input.len(),
        }
    }
}

impl<'input> Lexer<'input> {
    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn move_to(&mut self, i: Pos) {
        let diff = i - self.pos();
        self.chars = self.chars.as_str()[diff..].chars();
    }

    fn pos(&self) -> Pos {
        self.input_len - self.chars.as_str().len()
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or('\0')
    }

    fn ok(&mut self, token: Token, span_lo: Pos, span_hi: Pos) -> Result<Spanned<Token>> {
        self.move_to(span_hi);
        Ok((span_lo, token, span_hi))
    }

    fn error(&mut self, span_lo: Pos, span_hi: Pos, message: impl Into<String>) -> Result<Spanned<Token>> {
        self.move_to(span_hi);
        Err(LexicalError::new(span_lo, span_hi, message.into()))
    }

    fn numeric_literal(&mut self, lo: Pos) -> Result<Spanned<Token>> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?x)
            ^-?                 # sign
            (0|[1-9][0-9]*)     # integer
            ([.][0-9]+|)        # fraction
            ([eE][-+]?[0-9]+|)  # exponent
        ").unwrap());

        let Some(m) = RE.find(self.chars.as_str()) else {
            return self.error(lo, lo+1, "invalid numeric literal");
        };
        let n = f64::from_str(m.as_str()).unwrap();
        self.ok(Token::Number(n), lo, lo + m.len())
    }

    fn identifier_or_reserved(&mut self, lo: Pos) -> Result<Spanned<Token>> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap());

        let Some(m) = RE.find(self.chars.as_str()) else {
            panic!("bug");
        };
        let hi = lo + m.len();
        match m.as_str() {
            "true" => self.ok(Token::True, lo, hi),
            "false" => self.ok(Token::False, lo, hi),
            "null" => self.ok(Token::Null, lo, hi),
            s => self.ok(Token::Identifier(s.to_owned()), lo, hi),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Spanned<Token>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.first();
            let pos = self.pos();
            return match ch {
                '\0' => return None,

                // skip whitespaces.
                ch if ch.is_ascii_whitespace() => {
                    self.bump();
                    continue;
                }

                ':' => Some(self.ok(Token::Colon, pos, pos+1)),
                ',' => Some(self.ok(Token::Comma, pos, pos+1)),
                '[' => Some(self.ok(Token::LBracket, pos, pos+1)),
                ']' => Some(self.ok(Token::RBracket, pos, pos+1)),
                '{' => Some(self.ok(Token::LBrace, pos, pos+1)),
                '}' => Some(self.ok(Token::RBrace, pos, pos+1)),

                ch if ch.is_ascii_digit() || (ch == '-' && self.second().is_ascii_digit()) => {
                    Some(self.numeric_literal(pos))
                }

                ch if ch.is_ascii_alphabetic() => {
                    Some(self.identifier_or_reserved(pos))
                }

                ch => {
                    return Some(self.error(pos, pos+1, format!("unexpected character: '{ch}'")));
                }
            }
        }
    }
}
