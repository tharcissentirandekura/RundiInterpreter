use std::collections::HashMap;
use crate::parser::Expr;

#[derive(Debug)]
pub struct Environment{
    vars:HashMap<String,Expr>,
}

impl Environment{
    pub fn new() -> Self{
        Environment{vars:HashMap::new()} //create new hashmap for environments
    }
}