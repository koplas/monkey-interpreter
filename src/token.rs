use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum TokenValue<'source> {
    Illegal,
    Eof,

    Identifier(&'source str),
    Int(&'source str),
    Float(&'source str),
    String(&'source str),

    Bang,
    Assign,
    Plus,
    Minus,
    Astarisk,
    Slash,

    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Macro,
}

impl<'source> TokenValue<'source> {
    pub fn identifier_from(ident: &'source str) -> Self {
        match ident {
            "let" => Self::Let,
            "fn" => Self::Function,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            "macro" => Self::Macro,
            _ => Self::Identifier(ident),
        }
    }
}

impl fmt::Display for TokenValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use crate::token::TokenValue::*;
        let token = match self {
            Illegal => "Illegal",
            Eof => "EOF",
            Identifier(..) => "Ident",
            Int(..) => "Int",
            Float(..) => "Float",
            String(..) => "String",
            Bang => "!",
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Astarisk => "*",
            Slash => "/",
            LessThan => "<",
            GreaterThan => ">",
            Equal => "==",
            NotEqual => "!=",
            Comma => ",",
            Semicolon => ";",
            Colon => ":",
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            LBracket => "[",
            RBracket => "]",
            Function => "Function",
            Let => "Let",
            True => "true",
            False => "false",
            If => "if",
            Else => "else",
            Return => "return",
            Macro => "macro",
        };
        write!(f, "{token}")
    }
}

#[derive(Debug)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: usize,
}
