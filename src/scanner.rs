use crate::lox::Lox;
use crate::token::Literal::Nil;
use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    session: Lox,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            session: Lox::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        match c {
            '(' => self.add_token(TokenType::LeftParen, Nil),
            ')' => self.add_token(TokenType::RightParen, Nil),
            '{' => self.add_token(TokenType::LeftBrace, Nil),
            '}' => self.add_token(TokenType::RightBrace, Nil),
            ',' => self.add_token(TokenType::Comma, Nil),
            '.' => self.add_token(TokenType::Dot, Nil),
            '-' => self.add_token(TokenType::Minus, Nil),
            '+' => self.add_token(TokenType::Plus, Nil),
            ';' => self.add_token(TokenType::Semicolon, Nil),
            '*' => self.add_token(TokenType::Star, Nil),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, Nil)
                } else {
                    self.add_token(TokenType::Bang, Nil)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, Nil)
                } else {
                    self.add_token(TokenType::Equal, Nil)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, Nil)
                } else {
                    self.add_token(TokenType::Equal, Nil)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, Nil)
                } else {
                    self.add_token(TokenType::Greater, Nil)
                }
            }
            '/' => {
                if self.peek() != '\n' && !self.is_at_end() {
                    self.current += 1
                } else {
                    self.add_token(TokenType::Slash, Nil)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            _ => self.session.error(self.line, "Unexpected character."),
        }
    }

    /// Hepler method of `scan_token` function. Match next char with the given char.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    /// Hepler method of `scan_token` function.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::Nil,
            self.line,
        ));
    }
}
