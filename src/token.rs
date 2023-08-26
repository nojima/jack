use std::fmt::{self, Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    True,
    False,
    Null,
    Number(f64),
    String(String),
    Identifier(String),

    If,
    Then,
    Else,

    Colon,
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
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
