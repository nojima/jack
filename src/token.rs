use compact_str::CompactString;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    True,
    False,
    Null,
    Number(f64),
    String(String),
    Identifier(CompactString),

    If,
    Then,
    Else,
    Local,
    Function,

    Dot,
    Colon,
    Semicolon,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Eq,
    EqEq,
    Exclamation,
    NotEq,
    Ampersand,
    AndAnd,
    Pipe,
    OrOr,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
