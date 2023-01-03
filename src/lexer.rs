use crate::token::{Token, TokenValue};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'source> {
    input: &'source str,
    iter: Peekable<CharIndices<'source>>,
    position: usize,
    char: char,
}

impl<'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        let mut lexer = Self {
            input,
            iter: input.char_indices().peekable(),
            position: 0,
            char: '\x00',
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if let Some((pos, char)) = self.iter.next() {
            self.char = char;
            self.position = pos;
        } else {
            self.char = '\x00';
        }
    }

    fn read_char_if_eq(&mut self, expected: char) -> bool {
        if let Some((pos, char)) = self.iter.next_if(|&(_, char)| char == expected) {
            self.char = char;
            self.position = pos;
            true
        } else {
            self.char = '\x00';
            false
        }
    }

    fn skip_whitespace(&mut self) {
        if !self.char.is_whitespace() {
            return;
        }
        // Todo refactor into read until
        for (pos, char) in self.iter.by_ref() {
            if !char.is_whitespace() {
                self.char = char;
                self.position = pos;
                break;
            }
        }
        if self.iter.peek().is_none() {
            self.char = '\x00';
        }
    }

    fn read_identifier(&mut self) -> &'source str {
        let start = self.position;
        // Todo rewrite loop refactor into read until
        while self.iter.next_if(|(_, c)| c.is_alphanumeric()).is_some() {
            self.position += 1;
        }
        &self.input[start..self.position + 1]
    }

    fn read_number(&mut self) -> &'source str {
        let start = self.position;
        // Todo rewrite loop, refactor into read until
        while self.iter.next_if(|(_, c)| c.is_ascii_digit()).is_some() {
            self.position += 1;
        }
        &self.input[start..self.position + 1]
    }

    fn read_number_token(&mut self) -> TokenValue<'source> {
        let start = self.position;
        let int = self.read_number();
        if !self.read_char_if_eq('.') {
            TokenValue::Int(int)
        } else {
            let _frac = self.read_number();
            TokenValue::Float(&self.input[start..self.position + 1])
        }
    }

    fn read_string(&mut self) -> TokenValue<'source> {
        self.read_char();
        let start = self.position;
        loop {
            if self.char == '"' || self.char == '\x00' {
                break;
            }
            self.read_char();
        }
        TokenValue::String(&self.input[start..self.position])
    }

    fn skip_line(&mut self) {
        while self.iter.next_if(|(_, c)| c != &'\n').is_some() {
            self.position += 1;
        }
        self.read_char();
    }

    fn next_token(&mut self) -> Token<'source> {
        self.skip_whitespace();

        if self.char == '/' && self.iter.peek().is_some_and(|(_, c)| c == &'/') {
            self.skip_line();
            self.skip_whitespace();
        }

        let token_value = match self.char {
            '=' => {
                if self.read_char_if_eq('=') {
                    TokenValue::Equal
                } else {
                    TokenValue::Assign
                }
            }
            '!' => {
                if self.read_char_if_eq('=') {
                    TokenValue::NotEqual
                } else {
                    TokenValue::Bang
                }
            }
            '+' => TokenValue::Plus,
            '-' => TokenValue::Minus,
            '*' => TokenValue::Astarisk,
            '/' => TokenValue::Slash,
            '<' => TokenValue::LessThan,
            '>' => TokenValue::GreaterThan,
            ',' => TokenValue::Comma,
            ';' => TokenValue::Semicolon,
            ':' => TokenValue::Colon,
            '(' => TokenValue::LParen,
            ')' => TokenValue::RParen,
            '{' => TokenValue::LBrace,
            '}' => TokenValue::RBrace,
            '[' => TokenValue::LBracket,
            ']' => TokenValue::RBracket,
            '"' => self.read_string(),
            '\x00' => TokenValue::Eof,

            char if char.is_alphabetic() => {
                let ident = self.read_identifier();
                TokenValue::identifier_from(ident)
            }

            char if char.is_ascii_digit() => self.read_number_token(),

            _ => TokenValue::Illegal,
        };
        self.read_char();

        if token_value == TokenValue::Illegal {
            println!("Illegal character: {:?}", self.char);
        }

        Token {
            value: token_value,
            pos: self.position,
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.value == TokenValue::Eof {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenValue};

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*0;
        2 < 10 > 7;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;

        "foobar";
        "foo bar";

        [1, 2];

        {"foo": "bar"};

        // comment
        let a = 1; // inline comment

        let b = 123.45;
        let c = 0.678;
        let d = 9.0;

        macro(x, y) { x + y; };
        "#;

        use crate::token::TokenValue::*;
        let expected_tokens: &[TokenValue] = &[
            Let,
            Identifier("five"),
            Assign,
            Int("5"),
            Semicolon,
            Let,
            Identifier("ten"),
            Assign,
            Int("10"),
            Semicolon,
            Let,
            Identifier("add"),
            Assign,
            Function,
            LParen,
            Identifier("x"),
            Comma,
            Identifier("y"),
            RParen,
            LBrace,
            Identifier("x"),
            Plus,
            Identifier("y"),
            Semicolon,
            RBrace,
            Semicolon,
            Let,
            Identifier("result"),
            Assign,
            Identifier("add"),
            LParen,
            Identifier("five"),
            Comma,
            Identifier("ten"),
            RParen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Astarisk,
            Int("0"),
            Semicolon,
            Int("2"),
            LessThan,
            Int("10"),
            GreaterThan,
            Int("7"),
            Semicolon,
            If,
            LParen,
            Int("5"),
            LessThan,
            Int("10"),
            RParen,
            LBrace,
            Return,
            True,
            Semicolon,
            RBrace,
            Else,
            LBrace,
            Return,
            False,
            Semicolon,
            RBrace,
            Int("10"),
            Equal,
            Int("10"),
            Semicolon,
            Int("10"),
            NotEqual,
            Int("9"),
            Semicolon,
            String("foobar"),
            Semicolon,
            String("foo bar"),
            Semicolon,
            LBracket,
            Int("1"),
            Comma,
            Int("2"),
            RBracket,
            Semicolon,
            LBrace,
            String("foo"),
            Colon,
            String("bar"),
            RBrace,
            Semicolon,
            Let,
            Identifier("a"),
            Assign,
            Int("1"),
            Semicolon,
            Let,
            Identifier("b"),
            Assign,
            Float("123.45"),
            Semicolon,
            Let,
            Identifier("c"),
            Assign,
            Float("0.678"),
            Semicolon,
            Let,
            Identifier("d"),
            Assign,
            Float("9.0"),
            Semicolon,
            Macro,
            LParen,
            Identifier("x"),
            Comma,
            Identifier("y"),
            RParen,
            LBrace,
            Identifier("x"),
            Plus,
            Identifier("y"),
            Semicolon,
            RBrace,
            Semicolon,
            Eof,
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(&token.value, expected_token);
        }
    }
}
