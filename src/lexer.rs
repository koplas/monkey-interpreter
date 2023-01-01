use crate::token::{Token, TokenValue};
use std::str::CharIndices;
use std::iter::Peekable;

pub struct Lexer<'source> {
    input: &'source str,
    iter: Peekable<CharIndices<'source>>,
    position: usize,
    char: char,
    error: bool,
}

impl<'source> Lexer<'source> {
    fn new(input: &'source str) -> Self {
        let mut lexer = Self {
            input,
            iter: input.char_indices().peekable(),
            position: 0,
            char: '\x00',
            error: false,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if let Some((pos, char)) = self.iter.next() {
            self.char = char;
            self.position = pos;
        } else {
            self.char = '\x00'
        }
    }

    fn skip_whitespace(&mut self) {
        if !self.char.is_whitespace() {
            return;
        }
        while let Some((pos, char)) = self.iter.next() {
            if !char.is_whitespace() {
                self.char = char;
                self.position = pos;
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token<'source> {
        self.skip_whitespace();

        let token_value = match self.char {
            '=' => TokenValue::Assign,
            '+' => TokenValue::Plus,
            ',' => TokenValue::Comma,
            ';' => TokenValue::Semicolon,
            '(' => TokenValue::LParen,
            ')' => TokenValue::RParen,
            '{' => TokenValue::LBrace,
            '}' => TokenValue::RBrace,
            '\x00' => TokenValue::Eof,

            _ => TokenValue::Illegal,
        };
        self.read_char();

        Token{ value: token_value, pos: self.position}
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

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
        let input = "=+(){},;";

        use crate::token::TokenValue::*;
        let expected_tokens: [TokenValue; 8] = [
            Assign, Plus, LParen, RParen, LBrace, RBrace, Comma, Semicolon,
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.value, expected_token);
        }
    }
}
