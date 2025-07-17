// parser.rs
pub enum Expr {
    Assignment { name: String, value: Box<Expr> },
    Print { value: Box<Expr> },
    Literal(String),
    Variable(String),
}
