use crate::token::Token;
use crate::token::TokenType;

struct Scanner {
    code: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    current_char: char,
    line: usize,
}

impl Scanner {
    fn new(program: &str) -> Scanner {
        Scanner {
            code: program.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            current: 0,
            current_char: '\0',
            line: 1,
        }
    }

    fn peek(&self) -> char {
        if self.current >= self.code.len() {
            '\0'
        } else {
            self.code[self.current]
        }
    }

    fn next(&mut self) -> char {
        let nc = self.peek();
        self.current += 1;
        self.current_char = nc;
        nc
    }

    fn is_at_end(&self) -> bool {
        self.peek() == '\0'
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn scan(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.next().to_string();
        match c.as_str() {
            "\n" => self.line = self.line + 1,
            c if c.trim().is_empty() => (),
            "(" => self.add_token(TokenType::LeftParen, c),
            ")" => self.add_token(TokenType::RightParen, c),
            "{" => self.add_token(TokenType::LeftBrace, c),
            "}" => self.add_token(TokenType::RightBrace, c),
            "," => self.add_token(TokenType::Comma, c),
            "." => self.add_token(TokenType::Dot, c),
            "-" => self.add_token(TokenType::Minus, c),
            "+" => self.add_token(TokenType::Plus, c),
            ";" => self.add_token(TokenType::Semicolon, c),
            "*" => self.add_token(TokenType::Star, c),
            "!" => {
                if self.peek() == '=' {
                    self.next();
                    self.add_token(TokenType::BangEquals, "!=".to_string());
                } else {
                    self.add_token(TokenType::Bang, c);
                }
            }
            "=" => {
                if self.peek() == '=' {
                    self.next();
                    self.add_token(TokenType::EqualsEquals, "==".to_string());
                } else {
                    self.add_token(TokenType::Equals, c);
                }
            }
            "<" => {
                if self.peek() == '=' {
                    self.next();
                    self.add_token(TokenType::LessEqual, "<=".to_string());
                } else {
                    self.add_token(TokenType::Less, c);
                }
            }
            ">" => {
                if self.peek() == '=' {
                    self.next();
                    self.add_token(TokenType::GreaterEqual, ">=".to_string());
                } else {
                    self.add_token(TokenType::Greater, c);
                }
            }
            "/" => {
                if self.peek() == '/' {
                    // Comment until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, c);
                }
            }
            "\"" => self.scan_string(),
            c if is_alphabetic(c) => self.scan_identifier(),
            c if is_numeric(c) => self.scan_number(),
            _ => panic!("unrecognized char: '{}' at line {}", c, self.line),
        };
    }

    fn scan_string(&mut self) {
        let st = self.scan_generic(TokenType::String);
        self.tokens.push(Token::new_string(st, self.line));
    }

    fn scan_identifier(&mut self) {
        let id = self.scan_generic(TokenType::Identifier);
        self.tokens.push(Token::new_identifier(id, self.line));
    }

    fn scan_number(&mut self) {
        let lit = self.scan_generic(TokenType::Number);
        self.tokens
            .push(Token::new_number(lit.parse::<f64>().unwrap(), self.line));
    }

    fn scan_generic(&mut self, ttype: TokenType) -> String {
        let mut literal = String::from("");
        if ttype != TokenType::String {
            // We avoid 1st character if string since it's "
            literal.push(self.current_char);
        }

        while self.current <= self.code.len() {
            let c = self.peek();
            if !c.is_ascii_alphanumeric() && !(ttype == TokenType::String && c.is_whitespace()) {
                if c == '"' && ttype == TokenType::String {
                    self.next();
                }
                break;
            }
            literal.push(c);
            self.next();
        }

        literal
    }
}

fn is_alphabetic(c: &str) -> bool {
    c.chars().nth(0).unwrap().is_ascii_alphabetic()
}

fn is_numeric(c: &str) -> bool {
    c.chars().nth(0).unwrap().is_ascii_digit()
}

pub fn scan_program(program: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(program);
    scanner.scan()
}
