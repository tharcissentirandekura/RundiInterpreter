#[derive(Debug, PartialEq)]
pub enum Token{
    Number(i64),
    Symbol(String),
    OpenParen,
    CloseParen,
}

pub fn tokenize(input:&str) -> Vec<Token>{
    let mut tokens = Vec::new();
    for word in input.split_whitespace(){
        match word{
            "(" => tokens.push(Token::OpenParen),
            ")" => tokens.push(Token::CloseParen),
            _ if word.chars().all(char::is_numeric) => tokens.push(Token::Number(word.parse().unwrap())),
            _ => tokens.push(Token::Symbol(word.to_string()))
        }
    }
    tokens //return tokens
}