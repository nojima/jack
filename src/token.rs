#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    True,
    False,
    Null,
    Number(f64),
    String(String),
    Identifier(String),

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
}
