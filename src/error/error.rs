// errors.rs
use std::fmt;

#[derive(Debug)]
pub enum KirundiError {
    UndefinedVariable(String),
    SyntaxError(String),
    RuntimeError(String),
}

impl fmt::Display for KirundiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KirundiError::UndefinedVariable(name) => {
                write!(f, "Ndababariwe, ariko sinzi ico ‘{}’ bivuga. Woba wibagiye kuyishiraho?", name)
            }
            KirundiError::SyntaxError(msg) => {
                write!(f, "Hari ikosa mu nyandiko: {}", msg)
            }
            KirundiError::RuntimeError(msg) => {
                write!(f, "Ikibazo igihe ushitseko: {}", msg)
            }
        }
    }
}
