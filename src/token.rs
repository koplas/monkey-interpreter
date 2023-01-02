use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum TokenValue<'source> {
    Illegal,
    Eof,

    Indentifier(&'source str),
    Int(&'source str),

    Assign,
    Plus,

    Equal,
    NotEqual,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

impl<'source> TokenValue<'source> {
    pub fn identifier_from(ident: &'source str) -> Self {
        match ident {
            "let" => Self::Let,
            "fn" => Self::Function,
            _ => Self::Indentifier(ident),
        }
    }
}

impl fmt::Display for TokenValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use crate::token::TokenValue::*;
        let token = match self {
            Illegal => "Illegal",
            Eof => "EOF",
            Indentifier(..) => "Ident",
            Int(..) => "Int",
            Assign => "=",
            Plus => "+",
            Equal => "==",
            NotEqual => "!=",
            Comma => ",",
            Semicolon => ";",
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            Function => "Function",
            Let => "Let",
        };
        write!(f, "{}", token)
    }
}

pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: usize,
}
