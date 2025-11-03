#![allow(dead_code)]
use crate::Number;
use std::collections::HashMap;

use crate::{
    opcode_to_u8, Chunk, OpCode, Scanner, Token, TokenType, Value,
};


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    PrecNone        = 0,
    PrecAssignment  = 1, // =
    PrecOr          = 2, // or
    PrecAnd         = 3, // and
    PrecEquality    = 4, // == !=
    PrecComparison  = 5, // < > <= >=
    PrecTerm        = 6, // + -
    PrecFactor      = 7, // * /
    PrecUnary       = 8, // ! -
    PrecCall        = 9, // . ()
    PrecPrimary     = 10,
}

impl Precedence {
    pub fn from_integer(i: u32) -> Precedence {
        match i {
            0  => Precedence::PrecNone,
            1  => Precedence::PrecAssignment,
            2  => Precedence::PrecOr,
            3  => Precedence::PrecAnd,
            4  => Precedence::PrecEquality,
            5  => Precedence::PrecComparison,
            6  => Precedence::PrecTerm,
            7  => Precedence::PrecFactor,
            8  => Precedence::PrecUnary,
            9  => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _  => Precedence::PrecPrimary, 
        }
    }

    #[inline]
    pub fn next(self) -> Precedence {
        Precedence::from_integer((self as u32).saturating_add(1))
    }
}

/// The type of parse functions stored in the Pratt table.
pub type ParseFn = Option<fn(&mut Compiler)>;

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn init_parse_rule(prefix: ParseFn, infix: ParseFn, precedence: Precedence) -> Self {
        Self { prefix, infix, precedence }
    }
}

pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub parse_rules: HashMap<TokenType, ParseRule>,
}

impl Parser {
    pub fn init_parser() -> Self {
        Self {
            current: Token { token_type: TokenType::TokenError, value: Vec::new(), line: 0 },
            previous: Token { token_type: TokenType::TokenError, value: Vec::new(), line: 0 },
            had_error: false,
            panic_mode: false,
            parse_rules: Parser::pratt_table(),
        }
    }

    pub fn get_rule(&self, t: TokenType) -> ParseRule {
        *self.parse_rules.get(&t).expect("missing Pratt rule for token")
    }

    pub fn pratt_table() -> HashMap<TokenType, ParseRule> {
        // Helper shortcuts
        fn none() -> ParseFn { None }
        fn pre(f: fn(&mut Compiler)) -> ParseFn { Some(f) }
        fn inf(f: fn(&mut Compiler)) -> ParseFn { Some(f) }

        let mut m = HashMap::<TokenType, ParseRule>::new();

        // (Key) -> {prefix, infix, precedence}
        m.insert(TokenType::TokenLeftParen,   ParseRule::init_parse_rule(pre(Compiler::grouping), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenRightParen,  ParseRule::init_parse_rule(none(), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenLeftBrace,   ParseRule::init_parse_rule(none(), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenRightBrace,  ParseRule::init_parse_rule(none(), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenComma,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenDot,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecNone));

        m.insert(TokenType::TokenMinus,       ParseRule::init_parse_rule(pre(Compiler::unary),   inf(Compiler::binary), Precedence::PrecTerm));
        m.insert(TokenType::TokenPlus,        ParseRule::init_parse_rule(none(),                 inf(Compiler::binary), Precedence::PrecTerm));
        m.insert(TokenType::TokenSemicolon,   ParseRule::init_parse_rule(none(), none(), Precedence::PrecTerm));

        m.insert(TokenType::TokenSlash,       ParseRule::init_parse_rule(none(),                 inf(Compiler::binary), Precedence::PrecFactor));
        m.insert(TokenType::TokenStar,        ParseRule::init_parse_rule(none(),                 inf(Compiler::binary), Precedence::PrecFactor));

        m.insert(TokenType::TokenNot,         ParseRule::init_parse_rule(pre(Compiler::unary), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenNotEqual,    ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecEquality));
        m.insert(TokenType::TokenEqual,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenEqualEqual,  ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecEquality));
        m.insert(TokenType::TokenGreater,      ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecComparison));
        m.insert(TokenType::TokenGreaterEqual, ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecComparison));
        m.insert(TokenType::TokenLess,         ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecComparison));
        m.insert(TokenType::TokenLessEqual,    ParseRule::init_parse_rule(none(), inf(Compiler::binary), Precedence::PrecComparison));

        m.insert(TokenType::TokenIdentifier,  ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenString,      ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenNumber,      ParseRule::init_parse_rule(pre(Compiler::number),  none(),               Precedence::PrecFactor));

        m.insert(TokenType::TokenAnd,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenClass,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenElse,        ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenFalse, ParseRule::init_parse_rule(pre(Compiler::literal), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenFor,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenFun,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenIf,          ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenNil,   ParseRule::init_parse_rule(pre(Compiler::literal), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenOr,          ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenPrint,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenReturn,      ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenSuper,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenThis,        ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenTrue,  ParseRule::init_parse_rule(pre(Compiler::literal), none(), Precedence::PrecNone));
        m.insert(TokenType::TokenVar,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenWhile,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenError,       ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));
        m.insert(TokenType::TokenEof,         ParseRule::init_parse_rule(none(), none(), Precedence::PrecFactor));

        m
    }
}

/// The compiler holds the bytecode chunk, scanner, and parser.
pub struct Compiler {
    chunk: Chunk,
    scanner: Scanner,
    pub parser: Parser,
}

impl Compiler {
    pub fn init_compiler() -> Self {
        Self {
            chunk: Chunk::init_chunk(),
            scanner: Scanner::init_scanner(""),
            parser: Parser::init_parser(),
        }
    }

    /// Move the compiled chunk out 
    pub fn get_chunk(self) -> Chunk {
        self.chunk
    }

    /// Compile a Lox source string to bytecode; returns success/failure.
    pub fn compile(&mut self, source_code: &str) -> bool {
        self.scanner = Scanner::init_scanner(source_code);

        self.advance();

        self.expression();

        self.consume(TokenType::TokenEof, "Expect end of expression.");

        self.end_compiler();

        !self.parser.had_error
    }

    /// Move parser to next non-error token, reporting errors from scanner.
    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            let tok = self.scanner.scan_token();
            self.parser.current = tok.clone();

            if tok.token_type != TokenType::TokenError {
                break;
            }
            let msg = String::from_utf8(tok.value.clone()).unwrap_or_else(|_| "Scan error".into());
            self.error_at_current(&msg);
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // parse right operand at higher precedence
        let rule = self.parser.get_rule(operator_type);
        let higher = rule.precedence.next();
        self.parse_precedence(higher);

        match operator_type {
            // arithmetic
            TokenType::TokenPlus    => self.emit_byte(opcode_to_u8(OpCode::OpAdd)),
            TokenType::TokenMinus   => self.emit_byte(opcode_to_u8(OpCode::OpSubtract)),
            TokenType::TokenStar    => self.emit_byte(opcode_to_u8(OpCode::OpMultiply)),
            TokenType::TokenSlash   => self.emit_byte(opcode_to_u8(OpCode::OpDivide)),

            // equality
            TokenType::TokenEqualEqual => self.emit_byte(opcode_to_u8(OpCode::OpEqual)),
            TokenType::TokenNotEqual   => {
                self.emit_byte(opcode_to_u8(OpCode::OpEqual));
                self.emit_byte(opcode_to_u8(OpCode::OpNot));
            }

            // comparisons
            TokenType::TokenGreater => self.emit_byte(opcode_to_u8(OpCode::OpGreater)),
            TokenType::TokenLess    => self.emit_byte(opcode_to_u8(OpCode::OpLess)),
            TokenType::TokenGreaterEqual => {
                self.emit_byte(opcode_to_u8(OpCode::OpLess));
                self.emit_byte(opcode_to_u8(OpCode::OpNot));
            }
            TokenType::TokenLessEqual => {
                self.emit_byte(opcode_to_u8(OpCode::OpGreater));
                self.emit_byte(opcode_to_u8(OpCode::OpNot));
            }

            _ => {}
        }
    }


    fn consume(&mut self, ttype: TokenType, message: &str) {
        if self.parser.current.token_type == ttype {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // compile operand
        self.parse_precedence(Precedence::PrecUnary);

        match operator_type {
            TokenType::TokenMinus => self.emit_byte(opcode_to_u8(OpCode::OpNegate)),
            TokenType::TokenNot   => self.emit_byte(opcode_to_u8(OpCode::OpNot)),
            _ => {}
        }
    }

    fn number(&mut self) {
        let lexeme = match String::from_utf8(self.parser.previous.value.clone()) {
            Ok(s) => s,
            Err(_) => { self.error("Invalid UTF-8 in numeric literal."); return; }
        };
        match lexeme.parse::<Number>() {
            Ok(n) => self.emit_constant(Value::ValNumber(n)),
            Err(_) => self.error("Invalid integer literal."),
        }
    }

    fn literal(&mut self) {
    match self.parser.previous.token_type {
        TokenType::TokenNil   => self.emit_byte(opcode_to_u8(OpCode::OpNil)),
        TokenType::TokenTrue  => self.emit_byte(opcode_to_u8(OpCode::OpTrue)),
        TokenType::TokenFalse => self.emit_byte(opcode_to_u8(OpCode::OpFalse)),
        _ => self.error("Unexpected literal."),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.parser.get_rule(self.parser.previous.token_type).prefix;
        match prefix_rule {
            Some(prefix_fn) => prefix_fn(self),
            None => {
                self.error("Expected expression.");
                return;
            }
        }

        loop {
            let current_rule = self.parser.get_rule(self.parser.current.token_type);
            if precedence > current_rule.precedence {
                break;
            }
            if let Some(infix_fn) = current_rule.infix {
                self.advance();
                infix_fn(self);
            } else {
                break;
            }
        }
    }


    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.line as u8;
        self.chunk.write_to_chunk(byte, line);
    }

    fn emit_bytes(&mut self, a: u8, b: u8) {
        self.emit_byte(a);
        self.emit_byte(b);
    }

    fn emit_return(&mut self) {
        self.emit_byte(opcode_to_u8(OpCode::OpReturn));
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        self.chunk.add_constant(value)
    }

    fn emit_constant(&mut self, value: Value) {
        let const_idx = self.make_constant(value);
        self.emit_bytes(opcode_to_u8(OpCode::OpConstant), const_idx);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        
    }

    pub fn error_at_current(&mut self, message: &str) {
        let tok = self.parser.current.clone();
        self.error_at(tok, message);
    }

    pub fn error(&mut self, message: &str) {
        let tok = self.parser.previous.clone();
        self.error_at(tok, message);
    }

    pub fn error_at(&mut self, token: Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        println!("[line {}] Error", token.line);

        if token.token_type == TokenType::TokenEof {
            println!(" at end");
        } else if token.token_type == TokenType::TokenError {
        } else {
            let text = String::from_utf8(token.value.clone()).unwrap_or_default();
            println!(" at '{}'", text);
        }
        println!(": {}\n", message);
        self.parser.had_error = true;
    }
}
