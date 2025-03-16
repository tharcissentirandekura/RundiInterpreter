
use crate::Tokenizer::*;

/**
 * (define x 10): => gives 
 * Tokens → [OpenParen, Symbol("define"), Symbol("x"), Number(10), CloseParen]
 * List([Symbol("define"), Symbol("x"), Number(10)])
 */
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Symbol(String),
    List(Vec<Expr>),
}

pub fn parse(tokens: &[Token]) -> Expr {
    let (expr, rest) = parse_tokens(tokens);
    print!("The rest is {:?}", rest);
    if !rest.is_empty() {
        panic!("Unexpected tokens after parsing:You passed many values");
    }
    expr
}

pub fn parse_tokens(tokens: &[Token]) -> (Expr, &[Token]) {
    if tokens.is_empty() {
        panic!("Unexpected end of input");
    }

    let (first, rest) = tokens.split_first().unwrap();

    match first {
        Token::Number(n) => (Expr::Number(*n), rest),
        Token::Symbol(s) => (Expr::Symbol(s.clone()), rest),
        Token::OpenParen => {
            let mut exprs = Vec::new();
            let mut tokens = rest;

            while !tokens.is_empty() {
                if let Token::CloseParen = tokens[0] {
                    return (Expr::List(exprs), &tokens[1..]); // Return collected expressions
                }
                let (expr, new_tokens) = parse_tokens(tokens);
                exprs.push(expr);
                tokens = new_tokens;
            }

            panic!("Missing closing parenthesis");
        }
        Token::CloseParen => panic!("Unexpected `)`"),
    }
}
