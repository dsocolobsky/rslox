use crate::parser::{parse, Expr, LiteralExpression, UnaryExpression};
use crate::scanner::scan_program;

#[test]
fn parse_an_identifier() {
    let tokens = scan_program("somevar");
    assert_eq!(tokens.len(), 1);
    let expressions = parse(&tokens);
    assert_eq!(expressions.len(), 1);
    assert_eq!(
        expressions[0],
        Expr::Literal(LiteralExpression::Variable(String::from("somevar")))
    );
}

#[test]
fn parse_a_number() {
    let tokens = scan_program("432");
    assert_eq!(tokens.len(), 1);
    let expressions = parse(&tokens);
    assert_eq!(expressions.len(), 1);
    assert_eq!(
        expressions[0],
        Expr::Literal(LiteralExpression::Number(432.0))
    );
}

#[test]
fn parse_a_string() {
    let tokens = scan_program("\"thing\"");
    assert_eq!(tokens.len(), 1);
    let expressions = parse(&tokens);
    assert_eq!(expressions.len(), 1);
    assert_eq!(
        expressions[0],
        Expr::Literal(LiteralExpression::String(String::from("thing")))
    );
}

#[test]
fn parse_simple_negation() {
    let tokens = scan_program("!somevar");
    assert_eq!(tokens.len(), 2);
    let expressions = parse(&tokens);
    assert_eq!(expressions.len(), 2);
    assert_eq!(
        expressions[0],
        Expr::Unary(UnaryExpression::new(tokens[0].clone(), Box::new(
            Expr::Literal(LiteralExpression::Variable(String::from("somevar"))
        )))
    ));
}