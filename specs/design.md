# Ubumenyi Programming Language Design Document

## Overview

The Ubumenyi Programming Language is a domain-specific language designed for native Kirundi speakers. It implements a complete interpreter architecture with lexical analysis, parsing, abstract syntax tree (AST) generation, and evaluation phases. The language uses Kirundi keywords and follows natural Kirundi linguistic patterns while maintaining familiar programming concepts.

### Core Design Principles

1. **Native Language Integration**: All keywords, operators, and error messages use Kirundi terminology
2. **Familiar Programming Concepts**: Maps standard programming constructs to Kirundi equivalents
3. **Educational Focus**: Designed to teach programming concepts without language barriers
4. **Extensible Architecture**: Modular design allows for easy addition of new features

## Architecture

### High-Level Architecture

```
Source Code (Kirundi) → Lexer → Parser → AST → Evaluator → Result
                                    ↓
                              Environment (Variables/Functions)
```

### Component Overview

1. **Lexer/Tokenizer**: Converts Kirundi source code into tokens
2. **Parser**: Builds Abstract Syntax Tree from tokens
3. **AST**: Represents program structure in memory
4. **Evaluator**: Executes the AST with environment context
5. **Environment**: Manages variable and function scopes
6. **REPL**: Interactive programming environment
7. **Error Handler**: Provides Kirundi error messages

## Components and Interfaces

### 1. Lexer/Tokenizer Module

**Purpose**: Convert raw Kirundi source code into a stream of tokens

**Key Components**:
- `Token` struct with kind and value
- `TokenKind` enum for different token types
- `Lexer` struct for tokenization logic

**Kirundi Keywords Mapping**:
```rust
// Variable declaration
"reka" -> let/var
"ni" -> equals/is

// Control flow
"niba" -> if
"ariko" -> else
"kandi" -> and
"cyangwa" -> or
"ntabwo" -> not

// Loops
"kugeza" -> while
"kuri" -> for
"muri" -> in

// Functions
"igikorwa" -> function
"garuka" -> return

// Data types
"umubare" -> number
"ijambo" -> string
"ukuri" -> true
"ibinyoma" -> false
"ubusa" -> null/empty

// I/O operations
"andika" -> print/write
"soma" -> read/input
```

**Interface**:
```rust
pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self;
    pub fn next_token(&mut self) -> Token;
    pub fn tokenize(&mut self) -> Vec<Token>;
}
```

### 2. Parser Module

**Purpose**: Convert tokens into an Abstract Syntax Tree

**Key Components**:
- Recursive descent parser
- Precedence handling for operators
- Error recovery mechanisms

**Grammar Structure** (in EBNF-like notation):
```
program = statement*
statement = assignment | print_stmt | if_stmt | while_stmt | function_def
assignment = IDENTIFIER "ni" expression
print_stmt = "andika" "(" expression ")"
if_stmt = "niba" "(" expression ")" "{" statement* "}" ("ariko" "{" statement* "}")?
while_stmt = "kugeza" "(" expression ")" "{" statement* "}"
expression = logical_or
logical_or = logical_and (("cyangwa") logical_and)*
logical_and = equality (("kandi") equality)*
equality = comparison (("==" | "!=") comparison)*
comparison = term ((">" | ">=" | "<" | "<=") term)*
term = factor (("+" | "-") factor)*
factor = unary (("*" | "/") unary)*
unary = ("ntabwo" | "-") unary | primary
primary = NUMBER | STRING | IDENTIFIER | "(" expression ")"
```

**Interface**:
```rust
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self;
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError>;
}
```

### 3. AST Module

**Purpose**: Represent program structure in memory

**Node Types**:
```rust
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment { name: String, value: Expression },
    Print { expression: Expression },
    If { condition: Expression, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    While { condition: Expression, body: Vec<Statement> },
    FunctionDef { name: String, params: Vec<String>, body: Vec<Statement> },
    Return { value: Option<Expression> },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Value),
    Variable(String),
    Binary { left: Box<Expression>, operator: BinaryOp, right: Box<Expression> },
    Unary { operator: UnaryOp, operand: Box<Expression> },
    Call { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone)]
pub enum Value {
    Umubare(f64),      // Number
    Ijambo(String),    // String
    Ukuri(bool),       // Boolean
    Ubusa,             // Null/Empty
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Kongeramo,    // Add (+)
    Kuramo,       // Subtract (-)
    Gukuba,       // Multiply (*)
    Kugabanya,    // Divide (/)
    Kurenza,      // Greater than (>)
    Kurenzaho,    // Greater than or equal (>=)
    Kugabanuka,   // Less than (<)
    Kugabanukaho, // Less than or equal (<=)
    Bingana,      // Equal (==)
    Ntibingana,   // Not equal (!=)
    Kandi,        // And (&&)
    Cyangwa,      // Or (||)
}
```

### 4. Environment Module

**Purpose**: Manage variable and function scopes

**Features**:
- Lexical scoping with parent environments
- Variable storage and retrieval
- Function definition storage
- Scope chain traversal

**Interface**:
```rust
pub struct Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self;
    pub fn new_with_parent(parent: Environment) -> Self;
    pub fn define(&mut self, name: String, value: Value);
    pub fn get(&self, name: &str) -> Option<Value>;
    pub fn set(&mut self, name: String, value: Value) -> Result<(), RuntimeError>;
    pub fn define_function(&mut self, name: String, function: Function);
    pub fn get_function(&self, name: &str) -> Option<&Function>;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}
```

### 5. Evaluator Module

**Purpose**: Execute the AST and produce results

**Key Features**:
- Statement execution
- Expression evaluation
- Function calls with proper scoping
- Error handling with Kirundi messages

**Interface**:
```rust
pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self;
    pub fn evaluate_program(&mut self, statements: Vec<Statement>) -> Result<(), RuntimeError>;
    pub fn evaluate_statement(&mut self, stmt: Statement) -> Result<(), RuntimeError>;
    pub fn evaluate_expression(&mut self, expr: Expression) -> Result<Value, RuntimeError>;
}
```

### 6. REPL Module

**Purpose**: Provide interactive programming environment

**Features**:
- Kirundi welcome message
- Line-by-line evaluation
- Error display in Kirundi
- Command history
- Graceful exit with Kirundi commands

**Interface**:
```rust
pub struct Repl {
    evaluator: Evaluator,
    history: Vec<String>,
}

impl Repl {
    pub fn new() -> Self;
    pub fn run(&mut self);
    pub fn evaluate_line(&mut self, line: String) -> Result<Option<Value>, RuntimeError>;
    pub fn display_welcome(&self);
    pub fn display_prompt(&self);
}
```

## Data Models

### Core Value System

The language supports four primary data types, each with Kirundi names:

1. **Umubare** (Numbers): 64-bit floating point
2. **Ijambo** (Strings): UTF-8 encoded text
3. **Ukuri/Ibinyoma** (Booleans): True/False values
4. **Ubusa** (Null): Represents absence of value

### Variable Declaration Syntax

```kirundi
reka x ni 10          // let x = 10
reka izina ni "Jean"  // let name = "Jean"
reka ukuri_gusa ni ukuri  // let is_true = true
```

### Function Definition Syntax

```kirundi
igikorwa kubara(a, b) {
    garuka a + b
}

// Function call
reka igisumba ni kubara(5, 3)
andika(igisumba)  // prints 8
```

### Control Flow Syntax

```kirundi
// If statement
niba (x > 5) {
    andika("x ni munini")
} ariko {
    andika("x ni muto")
}

// While loop
reka i ni 0
kugeza (i < 10) {
    andika(i)
    reka i ni i + 1
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug)]
pub enum UbumenyiError {
    LexError(LexError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter { char: char, position: usize },
    UnterminatedString { position: usize },
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEndOfInput,
    InvalidSyntax { message: String },
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable { name: String },
    UndefinedFunction { name: String },
    TypeError { expected: String, found: String },
    DivisionByZero,
    InvalidOperation { operation: String, operands: String },
}
```

### Kirundi Error Messages

```rust
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable { name } => {
                write!(f, "Ikosa: Impinduka '{}' ntabwo yasobanuwe", name)
            }
            RuntimeError::UndefinedFunction { name } => {
                write!(f, "Ikosa: Igikorwa '{}' ntabwo kihari", name)
            }
            RuntimeError::TypeError { expected, found } => {
                write!(f, "Ikosa mu bwoko: Byari bitegerejwe '{}' ariko twabonye '{}'", expected, found)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Ikosa: Ntushobora kugabanya na zeru")
            }
            RuntimeError::InvalidOperation { operation, operands } => {
                write!(f, "Ikosa: Igikorwa '{}' ntabwo gishobora gukoreshwa na '{}'", operation, operands)
            }
        }
    }
}
```

## Testing Strategy

### Unit Testing Approach

1. **Lexer Tests**:
   - Token recognition for all Kirundi keywords
   - Number and string literal parsing
   - Operator tokenization
   - Error handling for invalid characters

2. **Parser Tests**:
   - Valid syntax parsing
   - Error recovery
   - Precedence handling
   - Complex expression parsing

3. **Evaluator Tests**:
   - Arithmetic operations
   - Variable assignment and retrieval
   - Function definition and calling
   - Control flow execution
   - Error propagation

4. **Integration Tests**:
   - Complete program execution
   - REPL functionality
   - Error message display
   - Multi-statement programs

### Test Data Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_assignment() {
        let source = "reka x ni 42";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate_program(statements).unwrap();
        
        assert_eq!(evaluator.environment.get("x"), Some(Value::Umubare(42.0)));
    }

    #[test]
    fn test_kirundi_arithmetic() {
        let source = "reka igisumba ni 10 + 5 * 2";
        // Test should evaluate to 20 (following standard precedence)
    }

    #[test]
    fn test_conditional_execution() {
        let source = r#"
            reka x ni 10
            niba (x > 5) {
                andika("munini")
            } ariko {
                andika("muto")
            }
        "#;
        // Should print "munini"
    }
}
```

### Performance Considerations

1. **Memory Management**: Use Rust's ownership system for safe memory handling
2. **Token Caching**: Cache frequently used tokens to reduce allocation
3. **AST Optimization**: Implement constant folding for compile-time optimizations
4. **Environment Efficiency**: Use efficient data structures for variable lookup

### Extensibility Design

1. **Keyword System**: Centralized keyword mapping for easy additions
2. **Operator Overloading**: Framework for adding new operators
3. **Built-in Functions**: Plugin system for standard library functions
4. **Module System**: Future support for importing external Kirundi modules

This design provides a solid foundation for implementing a complete programming language interpreter that serves the Kirundi-speaking community while maintaining professional software development standards.