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
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}
