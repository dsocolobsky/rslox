use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // Compound tokens
    Equals,
    EqualsEquals,
    Bang,
    BangEquals,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Option<Value>,
    pub line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}[{}]", self.token_type, self.lexeme)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            value: None,
            line,
        }
    }

    pub fn new_string(string: String, line: usize) -> Token {
        Token {
            token_type: TokenType::String,
            lexeme: string.clone(),
            value: Some(Value::String(string.clone())),
            line,
        }
    }

    pub fn new_number(number: f64, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            lexeme: number.to_string(),
            value: Some(Value::Number(number)),
            line,
        }
    }

    pub fn new_identifier(identifier: String, line: usize) -> Token {
        let token_type = match identifier.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        Token {
            token_type,
            lexeme: identifier,
            value: None,
            line,
        }
    }
}
