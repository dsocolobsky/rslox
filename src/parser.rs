use crate::token::{Token, TokenType, Value};

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Equality(EqualityExpression),
    Comparison(ComparisonExpression),
    Term(TermExpression),
    Factor(FactorExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Literal(LiteralExpression),
}

#[derive(Clone, PartialEq, Debug)]
pub enum LiteralExpression {
    Variable(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryExpression {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Box<Expr>) -> UnaryExpression {
        UnaryExpression {
            operator,
            right
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryExpression {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpression {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> BinaryExpression {
        BinaryExpression {
            left,
            operator,
            right
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FactorExpression {
}

#[derive(Clone, PartialEq, Debug)]
pub struct TermExpression {
}

#[derive(Clone, PartialEq, Debug)]
pub struct ComparisonExpression {
}

#[derive(Clone, PartialEq, Debug)]
pub struct EqualityExpression {
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    expressions: Vec<Expr>,
    current: usize,
}

impl Parser<'_> {
    fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens,
            expressions: Vec::new(),
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current + 1 >= self.expressions.len()
    }

    fn previous(&self) -> Option<Token> {
        if self.current > 0 {
            Some(self.tokens[self.current - 1].clone())
        } else {
            None
        }
    }

    // Consume current token and return it
    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn current_token(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn add_expression(&mut self, expr: Expr) {
        self.expressions.push(expr);
    }

    fn add_primary_expression(&mut self, expr: LiteralExpression) {
        self.add_expression(Expr::Literal(expr));
    }

    fn add_variable_expression(&mut self, token: Token) {
        self.add_primary_expression(LiteralExpression::Variable(token.lexeme))
    }

    fn add_string_expression(&mut self, token: Token) {
        self.add_primary_expression(LiteralExpression::String(token.lexeme));
    }

    fn add_number_expression(&mut self, token: Token) {
        let number = match token.value {
            Some(Value::Number(number)) => number,
            _ => panic!("Token was supposed to contain number"),
        };
        self.add_primary_expression(LiteralExpression::Number(number));
    }

    fn add_unary_expression(&mut self, expr: UnaryExpression) {
        self.add_expression(Expr::Unary(expr));
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let expr = self.parse_term();

        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            match operator {
                Some(op) => {
                    let right = self.parse_term();
                    BinaryExpression::new(Box::new(expr.clone()), op.clone(), Box::new(right));
                }
                None => panic!("Operator is None but was supposed to be something")
            }

        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let expr = self.parse_factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            match operator {
                Some(op) => {
                    let right = self.parse_factor();
                    BinaryExpression::new(Box::new(expr.clone()), op.clone(), Box::new(right));
                }
                None => panic!("Operator is None but was supposed to be something")
            }

        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let expr = self.parse_unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            match operator {
                Some(op) => {
                    let right = self.parse_unary();
                    BinaryExpression::new(Box::new(expr.clone()), op.clone(), Box::new(right));
                }
                None => panic!("Operator is None but was supposed to be something")
            }
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            match operator {
                Some(op) => {
                    let right = self.parse_unary();
                    return Expr::Unary(UnaryExpression::new(op.clone(), Box::new(right)))
                }
                None => panic!("Operator is None but was supposed to be something")
            }


        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::True => Expr::Literal(LiteralExpression::Boolean(true)),
            TokenType::False => Expr::Literal(LiteralExpression::Boolean(false)),
            TokenType::Nil => Expr::Literal(LiteralExpression::Nil(String::from("nil"))),
            TokenType::Identifier => Expr::Literal(LiteralExpression::Variable(String::from("nil"))),
            TokenType::String => Expr::Literal(LiteralExpression::Nil(String::from("nil"))),
            TokenType::Number => Expr::Literal(LiteralExpression::Nil(String::from("nil"))),
            _ => todo!()
        }
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        if token_types.iter().any(|ty| self.peek_type_is(*ty)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek_type_is(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    fn parse(&mut self) -> Vec<Expr> {
        while self.current < self.tokens.len() {
            let token = self.current_token();
            match token.token_type {
                TokenType::Identifier => self.add_variable_expression(token),
                TokenType::String => self.add_string_expression(token),
                TokenType::Number => self.add_number_expression(token),
                TokenType::Bang => {
                    self.parse_unary();
                }
                _ => panic!("Parser: Unrecognized token {:?}", token),
            }
            self.current = self.current + 1;
        }
        self.expressions.clone()
    }
}

pub fn parse(tokens: &Vec<Token>) -> Vec<Expr> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
