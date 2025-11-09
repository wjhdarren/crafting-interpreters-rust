use crate::lox::Lox;
use crate::token::Literal::Nil;
use crate::token::{Literal, Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ]),
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

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c = self.advance();
        match c {
            // single symbols
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
            // multiple symbols
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
                    self.add_token(TokenType::Less, Nil)
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
                if self.match_char('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Nil)
                }
            }

            // string
            '"' => self.string(lox),

            // empty space
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            _ => {
                // number
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    lox.error(self.line, "Unexpected character.")
                }
            }
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

    /// Hepler method of `scan_token` function. Look at current unconsumed character without consuming.
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peak_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            lox.error(self.line, "Unterminated string.");
            return;
        }

        // Consume closing "
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peak_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Literal::Number(number));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = String::from(&self.source[self.start..self.current]);
        let token_type = self
            .keywords
            .get(&text)
            .copied()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type, Nil);
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::Nil,
            self.line,
        ));
        self.tokens.clone()
    }
}
