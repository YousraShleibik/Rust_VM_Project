#![allow(dead_code)]

use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenSemicolon,
    TokenMinus, TokenPlus,
    TokenSlash, TokenStar,
    TokenNot, TokenNotEqual,
    TokenEqual, TokenEqualEqual,
    TokenLess, TokenLessEqual,
    TokenGreater, TokenGreaterEqual,
    TokenIdentifier,
    TokenString,
    TokenNumber,
    TokenTrue, TokenFalse,
    TokenAnd, TokenOr,
    TokenIf, TokenElse,
    TokenClass, TokenSuper, TokenThis,
    TokenFun,
    TokenVar,
    TokenReturn,
    TokenFor,
    TokenWhile,
    TokenNil,
    TokenPrint,
    TokenError,
    TokenEof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Vec<u8>, 
    pub line: usize,
}

#[derive(Debug)]
pub struct Scanner {
    source: Vec<u8>,
    start: usize,
    current: usize,
    line: usize,
}

#[inline]
fn is_digit(c: u8) -> bool { c.is_ascii_digit() }

#[inline]
fn is_alpha(c: u8) -> bool { c.is_ascii_alphabetic() || c == b'_' }

impl Scanner {
    pub fn init_scanner(source_code: &str) -> Self {
        Self {
            source: source_code.as_bytes().to_vec(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // peek() -> current char or '\0'
    #[inline]
    fn peek(&self) -> u8 {
        if self.is_at_end() { 0 } else { self.source[self.current] }
    }

    // peek_next() -> next char or '\0'
    #[inline]
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() { 0 } else { self.source[self.current + 1] }
    }

    // advance() -> current char, then move forward
    #[inline]
    fn advance(&mut self) -> u8 {
        let c = self.peek();
        self.current = self.current.saturating_add(1);
        c
    }

    // skip_next_character(): move forward one char
    #[inline]
    fn skip_next_character(&mut self) {
        if !self.is_at_end() { self.current += 1; }
    }

    // skip whitespace (+ // comments)
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                b' ' | b'\r' | b'\t' => { self.advance(); }
                b'\n' => { self.advance(); self.line += 1; }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() { self.advance(); }
                }
                _ => return,
            }
        }
    }

    #[inline]
    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }
        self.current += 1;
        true
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        let slice = &self.source[self.start..self.current];
        Token {
            token_type: ttype,
            value: slice.to_vec(),
            line: self.line,
        }
    }

    fn error_token(&self, msg: &str) -> Token {
        Token {
            token_type: TokenType::TokenError,
            value: msg.as_bytes().to_vec(),
            line: self.line,
        }
    }

    fn get_literal_string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != b'"' {
            if self.peek() == b'\n' { self.line += 1; }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated String Literal");
        }
        self.advance(); 
        self.make_token(TokenType::TokenString)
    }

    fn get_literal_number(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance(); // '.'
            while is_digit(self.peek()) { self.advance(); }
        }
        self.make_token(TokenType::TokenNumber)
    }

    fn get_identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        let ttype = match str::from_utf8(lexeme).unwrap_or("") {
            "and" => TokenType::TokenAnd,
            "class" => TokenType::TokenClass,
            "else" => TokenType::TokenElse,
            "false" => TokenType::TokenFalse,
            "for" => TokenType::TokenFor,
            "fun" => TokenType::TokenFun,
            "if" => TokenType::TokenIf,
            "nil" => TokenType::TokenNil,
            "or" => TokenType::TokenOr,
            "print" => TokenType::TokenPrint,
            "return" => TokenType::TokenReturn,
            "super" => TokenType::TokenSuper,
            "this" => TokenType::TokenThis,
            "true" => TokenType::TokenTrue,
            "var" => TokenType::TokenVar,
            "while" => TokenType::TokenWhile,
            _ => TokenType::TokenIdentifier,
        };
        self.make_token(ttype)
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();

        if is_alpha(c) { return self.get_identifier(); }
        if is_digit(c) { return self.get_literal_number(); }
        if c == b'"'   { return self.get_literal_string(); }

        match c {
            b'(' => self.make_token(TokenType::TokenLeftParen),
            b')' => self.make_token(TokenType::TokenRightParen),
            b'{' => self.make_token(TokenType::TokenLeftBrace),
            b'}' => self.make_token(TokenType::TokenRightBrace),
            b',' => self.make_token(TokenType::TokenComma),
            b'.' => self.make_token(TokenType::TokenDot),
            b';' => self.make_token(TokenType::TokenSemicolon),
            b'-' => self.make_token(TokenType::TokenMinus),
            b'+' => self.make_token(TokenType::TokenPlus),
            b'*' => self.make_token(TokenType::TokenStar),
            b'/' => self.make_token(TokenType::TokenSlash),

            b'!' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenNotEqual) }
                else { self.make_token(TokenType::TokenNot) }
            }
            b'=' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenEqualEqual) }
                else { self.make_token(TokenType::TokenEqual) }
            }
            b'<' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenLessEqual) }
                else { self.make_token(TokenType::TokenLess) }
            }
            b'>' => {
                if self.match_next(b'=') { self.make_token(TokenType::TokenGreaterEqual) }
                else { self.make_token(TokenType::TokenGreater) }
            }

            _ => self.error_token("Unknown character."),
        }
    }
}
