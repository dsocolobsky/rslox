use std::fmt;

enum Value {
    String(String),
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // Compound tokens
    Equals, EqualsEquals, Bang, BangEquals,
    Greater, GreaterEqual, Less, LessEqual,
    // Literals
    Identifier, String, Number,
    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, Eof
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Value,
    pub line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}[{}]", self.token_type, self.lexeme)
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token::newString(String::from("EMPTY"), line)
    }

    pub fn newString(string: String, line: usize) -> Token {
        Token {
            token_type: TokenType::String,
            lexeme: string,
            value: Value::String(string.clone()),
            line
        }
    }

    pub fn newNumber(number: f64, line: usize) -> Token {
        Token {
            token_type: TokenType::Number,
            lexeme: number.to_string(),
            value: Value::Number(number),
            line
        }
    }
}