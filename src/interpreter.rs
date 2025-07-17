// interpreter.rs
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, String>,
}

impl Interpreter {
    pub fn eval(&mut self, expr: Expr) -> Result<(), KirundiError> {
        // Match on expr and execute accordingly
        Ok(())
    }
}
