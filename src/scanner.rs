use crate::token::{Token, TokenType};

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source[self.current]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line));
    }

    fn add_token_string(&mut self, string: String) {
        self.tokens.push(Token::newString(string, self.line));
    }

    fn add_token_number(&mut self, number: f64) {
        self.tokens.push(Token::newNumber(number, self.line));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peekIs(&self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.source[self.current] == expected
        }
    }

    fn scan_string(&mut self) -> String {
        self.scan_generic(TokenType::String)
    }

    fn scan_identifier(&mut self) -> String {
        self.scan_generic(TokenType::Identifier)
    }

    fn scan_number(&mut self) -> f64 {
        let lit = self.scan_generic(TokenType::Number);
        lit.parse::<f64>().unwrap()
    }

    fn scan_generic(&mut self, ttype: TokenType) -> String {
        let mut literal = String::from("");
        if ttype != TokenType::String { // We avoid 1st character if string since it's "
            literal.push(self.source[self.current]);
        }

        while self.current <= self.code.len() {
            let c = self.peek();
            if !c.is_ascii_alphanumeric() && !(ttype == TokenType::String && c.is_whitespace()) {
                if c == '"' && ttype == TokenType::String {
                    self.advance();
                }
                break;
            }
            literal.push(c);
            self.advance();
        }

        literal
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c.as_str() {
            '\n' => self.line = self.line + 1,
            c if c.trim().is_empty() => (),
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            "!" => {
                if self.peekIs('=') {
                    self.add_token(TokenType::BangEquals);
                    self.current = self.current + 1;
                } else {
                    self.add_token(TokenType::Bang);
                }
            },
            "!" => {
                if self.peekIs('=') {
                    self.add_token(TokenType::BangEquals);
                    self.current = self.current + 1;
                } else {
                    self.add_token(TokenType::Bang);
                }
            },
            "<" => {
                if self.peekIs('=') {
                    self.add_token(TokenType::LessEqual);
                    self.current = self.current + 1;
                } else {
                    self.add_token(TokenType::Less);
                }
            },
            ">" => {
                if self.peekIs('=') {
                    self.add_token(TokenType::GreaterEqual);
                    self.current = self.current + 1;
                } else {
                    self.add_token(TokenType::Greater);
                }
            },
            "/" => {
                if self.peekIs('/') { // Comment until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            "\"" => self.add_token_string(self.scan_string()),
            c if c.chars().nth(0).unwrap().is_ascii_digit() => self.add_token_number(self.scan_number()),
            _ => panic!("Unexpected character {} at line {}", c.as_str(), self.line),
        }
    }

    fn scan(&mut self, program: &str) {
        self.source = program.chars().collect::<Vec<char>>();
    }
}