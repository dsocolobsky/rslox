use crate::scanner::scan_program;
use crate::token::Token;
use crate::token::TokenType;

fn tokens_to_literals(tokens: &Vec<Token>) -> Vec<String> {
    tokens.into_iter().map(|token| token.lexeme.clone()).collect()
}

#[test]
fn empty_string_produces_empty_list_of_tokens() {
    let tokens = scan_program("");
    assert_eq!(0, tokens.len());
}

#[test]
fn token_assignment() {
    let tokens = scan_program("=");
    assert_eq!(["="], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_plus() {
    let tokens = scan_program("+");
    assert_eq!(["+"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_minus() {
    let tokens = scan_program("-");
    assert_eq!(["-"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_asterisk() {
    let tokens = scan_program("*");
    assert_eq!(["*"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn token_equals() {
    let tokens = scan_program("==");
    assert_eq!(["=="], &tokens_to_literals(&tokens)[..]);
}

#[test]
#[should_panic]
fn invalid_token_should_raise_error() {
    let tokens = scan_program("=%");
    assert_eq!(0, tokens.len());
}

#[test]
fn several_tokens() {
    let tokens = scan_program(" + - == / ;; *");
    assert_eq!(["+", "-", "==", "/", ";", ";", "*"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal() {
    let tokens = scan_program("banana");
    assert_eq!(["banana"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal_and_token() {
    let tokens = scan_program("mango=");
    assert_eq!(["mango", "="], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn two_literals() {
    let tokens = scan_program("t omate");
    assert_eq!(["t", "omate"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn literal_number_and_string() {
    let tokens = scan_program(r#" banana 1337 "kiwi" "#);
    assert_eq!(["banana", "1337", "kiwi"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn complex_expression() {
    let tokens = scan_program("radio = pi*3 + 2;");
    assert_eq!(["radio", "=", "pi", "*", "3", "+", "2", ";"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn complex_expression_comparisons() {
    let tokens = scan_program("4 < 19>2 >= !1 <= 1!=10");
    assert_eq!(["4", "<", "19", ">", "2", ">=", "!", "1", "<=", "1", "!=", "10"],
               &tokens_to_literals(&tokens)[..]);
}

#[test]
fn parentheses_and_braces() {
    let tokens = scan_program("((2)) == !{{4}}");
    assert_eq!(["(", "(", "2", ")", ")", "==", "!", "{", "{", "4", "}", "}"],
               &tokens_to_literals(&tokens)[..]);
}

#[test]
fn single_letter_as_string() {
    let tokens = scan_program(r#""f""#);
    assert_eq!(["f"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn string_with_spaces() {
    let tokens = scan_program(r#""canada is a nice country""#);
    assert_eq!(["canada is a nice country"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn newlines() {
    let tokens = scan_program("3*\n2 + \n 3;");
    assert_eq!(["3", "*", "2", "+", "3", ";"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn newlines_token_saves_lines() {
    let tokens = scan_program("4\n3\n2 + 3");
    assert_eq!(1, tokens[0].line);
    assert_eq!(2, tokens[1].line);
    assert_eq!(3, tokens[2].line);
    assert_eq!(3, tokens[3].line);
    assert_eq!(3, tokens[4].line);
}

#[test]
fn comments() {
    let tokens = scan_program("4 // this is a comment \n 5;");
    assert_eq!(["4", "5", ";"], &tokens_to_literals(&tokens)[..]);
}

#[test]
fn keywords_1() {
    let tokens: Vec<Token> = scan_program("and class else false fun for if nil or print");
    assert_eq!(["and", "class", "else", "false", "fun", "for", "if", "nil", "or", "print"],
               &tokens_to_literals(&tokens)[..]
    );
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        [TokenType::And,
            TokenType::Class,
            TokenType::Else,
            TokenType::False,
            TokenType::Fun,
            TokenType::For,
            TokenType::If,
            TokenType::Nil,
            TokenType::Or,
            TokenType::Print,
        ],
        &token_types[..]
    );
}

#[test]
fn keywords_2() {
    let tokens: Vec<Token> = scan_program("return super this true var while");
    assert_eq!(["return", "super", "this", "true", "var", "while"],
               &tokens_to_literals(&tokens)[..]
    );
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        [TokenType::Return,
            TokenType::Super,
            TokenType::This,
            TokenType::True,
            TokenType::Var,
            TokenType::While,
        ],
        &token_types[..]
    );
}

// TODO this changes now with Lox
#[test]
fn sample_program() {
    let tokens = scan_program(
        r#"
            fn square(x) {
                return x * x;
            }

            fn main() {
                let y = square(4);
                while y >= 12 {
                    print("answer is " y);
                }
                return 0;
            }
            "#,
    );

    assert_eq!(["fn","square","(","x",")","{","return","x","*","x",";","}",
                   "fn","main","(",")","{","let","y","=","square","(","4",")",";",
                   "while","y",">=","12","{","print","(","answer is ","y",")",";","}",
                   "return","0",";","}"], &tokens_to_literals(&tokens)[..]);
}
