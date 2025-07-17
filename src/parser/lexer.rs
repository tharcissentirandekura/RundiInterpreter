// lexer.rs
#[derive(Debug, PartialEq)]
pub enum Token {
    Soma,
    Andika,
    Bingana,
    Nimba,
    Mugihe,
    Kiretse,
    Identifier(String),
    StringLiteral(String),
    Equal,
    Eof,
    // etc.
}
