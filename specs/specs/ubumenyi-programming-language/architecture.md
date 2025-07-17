# Ubumenyi Programming Language - Architecture Overview

## High-Level System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Source Code   │───▶│     Lexer       │───▶│     Parser      │
│   (Kirundi)     │    │  (Tokenizer)    │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Output      │◀───│   Evaluator     │◀───│      AST        │
│   (Results)     │    │ (Interpreter)   │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │  Environment    │
                       │ (Variables &    │
                       │  Functions)     │
                       └─────────────────┘
```

## Module Structure

```
src/
├── main.rs                 # CLI entry point
├── lib.rs                  # Library exports
├── lexer/
│   ├── mod.rs             # Module exports
│   ├── token.rs           # Token definitions
│   └── lexer.rs           # Tokenization logic
├── parser/
│   ├── mod.rs             # Module exports
│   ├── ast.rs             # AST node definitions
│   └── parser.rs          # Parsing logic
├── evaluator/
│   ├── mod.rs             # Module exports
│   ├── value.rs           # Value types
│   ├── environment.rs     # Scope management
│   └── evaluator.rs       # Interpretation logic
├── error/
│   ├── mod.rs             # Module exports
│   └── error.rs           # Error types & messages
├── repl/
│   ├── mod.rs             # Module exports
│   └── repl.rs            # Interactive shell
└── tests/
    ├── mod.rs             # Test module
    ├── lexer_tests.rs     # Lexer unit tests
    ├── parser_tests.rs    # Parser unit tests
    ├── evaluator_tests.rs # Evaluator unit tests
    └── integration_tests.rs # End-to-end tests
```

## Data Flow Architecture

### 1. Input Processing Pipeline

```
Raw Kirundi Code
      │
      ▼
┌─────────────┐
│   Lexer     │ ──── Converts text to tokens
│             │      "reka x ni 10" → [REKA, IDENTIFIER("x"), NI, NUMBER(10)]
└─────────────┘
      │
      ▼
┌─────────────┐
│   Parser    │ ──── Builds AST from tokens
│             │      Tokens → Assignment { name: "x", value: Number(10) }
└─────────────┘
      │
      ▼
┌─────────────┐
│  Evaluator  │ ──── Executes AST nodes
│             │      AST → Store "x" = 10 in environment
└─────────────┘
```

### 2. Memory Management

```
┌─────────────────────────────────────────────────────────────┐
│                    Environment Stack                        │
├─────────────────────────────────────────────────────────────┤
│  Global Scope                                               │
│  ├── Variables: { "x": 10, "name": "Jean" }                │
│  └── Functions: { "kubara": Function {...} }               │
├─────────────────────────────────────────────────────────────┤
│  Function Scope (when calling kubara)                      │
│  ├── Parameters: { "a": 5, "b": 3 }                       │
│  ├── Local Variables: { "result": 8 }                     │
│  └── Parent: → Global Scope                               │
└─────────────────────────────────────────────────────────────┘
```

## Core Components Detail

### 1. Lexer Component

**Responsibility**: Convert raw text into tokens

```rust
// Input: "reka x ni 10 + 5"
// Output: [
//   Token { kind: REKA, value: "reka" },
//   Token { kind: IDENTIFIER, value: "x" },
//   Token { kind: NI, value: "ni" },
//   Token { kind: NUMBER, value: "10" },
//   Token { kind: PLUS, value: "+" },
//   Token { kind: NUMBER, value: "5" }
// ]
```

**Key Features**:
- Kirundi keyword recognition
- Number and string parsing
- Operator tokenization
- Position tracking for errors

### 2. Parser Component

**Responsibility**: Build Abstract Syntax Tree

```rust
// Input: Tokens from lexer
// Output: AST nodes like:
Statement::Assignment {
    name: "x",
    value: Expression::Binary {
        left: Box::new(Expression::Number(10)),
        operator: BinaryOp::Add,
        right: Box::new(Expression::Number(5))
    }
}
```

**Key Features**:
- Recursive descent parsing
- Operator precedence handling
- Error recovery
- Kirundi syntax rules

### 3. Evaluator Component

**Responsibility**: Execute AST and manage state

```rust
// Input: AST from parser
// Process: 
//   1. Evaluate expression: 10 + 5 = 15
//   2. Store in environment: x = 15
// Output: Updated environment state
```

**Key Features**:
- Tree-walking interpretation
- Environment management
- Function call handling
- Error propagation

### 4. Environment Component

**Responsibility**: Variable and function storage

```rust
Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    parent: Option<Box<Environment>>
}
```

**Key Features**:
- Lexical scoping
- Variable shadowing
- Function closure support
- Scope chain traversal

## Kirundi Language Mapping

### Token Mapping Table

| Kirundi | English | Token Type | Usage |
|---------|---------|------------|-------|
| `reka` | let/var | REKA | Variable declaration |
| `ni` | is/equals | NI | Assignment operator |
| `niba` | if | NIBA | Conditional statement |
| `ariko` | else | ARIKO | Else clause |
| `kugeza` | while | KUGEZA | While loop |
| `igikorwa` | function | IGIKORWA | Function definition |
| `garuka` | return | GARUKA | Return statement |
| `andika` | print | ANDIKA | Output function |
| `ukuri` | true | UKURI | Boolean true |
| `ibinyoma` | false | IBINYOMA | Boolean false |

### Expression Evaluation Flow

```
Expression: "10 + 5 * 2"
     │
     ▼
Parse Tree:
    +
   / \
  10  *
     / \
    5   2
     │
     ▼
Evaluation (left-to-right, respecting precedence):
1. Evaluate 5 * 2 = 10
2. Evaluate 10 + 10 = 20
3. Return Value::Number(20)
```

## Error Handling Architecture

### Error Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Lexer Error │    │Parser Error │    │Runtime Error│
│             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │  Error Handler  │
                  │  (Kirundi Msgs) │
                  └─────────────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │ User Interface  │
                  │ (REPL/Console)  │
                  └─────────────────┘
```

### Error Message Examples

```rust
// English: "Undefined variable 'x'"
// Kirundi: "Impinduka 'x' ntabwo yasobanuwe"

// English: "Cannot divide by zero"
// Kirundi: "Ntushobora kugabanya na zeru"

// English: "Expected ')' after expression"
// Kirundi: "Byari bitegerejwe ')' nyuma y'imvugo"
```

## REPL Architecture

### Interactive Loop

```
┌─────────────────────────────────────────────────────────────┐
│                    REPL Main Loop                           │
├─────────────────────────────────────────────────────────────┤
│  1. Display prompt: "ubumenyi> "                           │
│  2. Read user input                                         │
│  3. Tokenize input                                          │
│  4. Parse tokens to AST                                     │
│  5. Evaluate AST                                            │
│  6. Display result                                          │
│  7. Handle errors in Kirundi                               │
│  8. Repeat                                                  │
└─────────────────────────────────────────────────────────────┘
```

### State Persistence

```
REPL Session State:
├── Persistent Environment
│   ├── User-defined variables
│   ├── User-defined functions
│   └── Built-in functions
├── Command History
├── Current Line Buffer
└── Error State
```

## Performance Considerations

### Memory Usage

- **AST Nodes**: Use `Box<>` for recursive structures
- **Environment**: Use `HashMap` for O(1) variable lookup
- **Values**: Clone small values, reference large ones
- **Strings**: Use `String` for owned, `&str` for borrowed

### Execution Speed

- **Tree Walking**: Direct AST interpretation (simple but slower)
- **Future Optimization**: Could add bytecode compilation
- **Tail Call**: Optimize recursive functions
- **Constant Folding**: Evaluate constants at parse time

## Extension Points

### Adding New Features

1. **New Keywords**: Add to lexer keyword map
2. **New Operators**: Extend BinaryOp/UnaryOp enums
3. **New Data Types**: Extend Value enum
4. **New Statements**: Add to Statement enum and parser
5. **Built-in Functions**: Extend evaluator with native functions

### Future Enhancements

- **Module System**: Import/export Kirundi modules
- **Standard Library**: Built-in functions for common tasks
- **Debugging**: Step-through debugging in Kirundi
- **IDE Integration**: Syntax highlighting and completion
- **Compilation**: Bytecode or native code generation

This architecture provides a solid foundation for building a complete programming language interpreter that serves the Kirundi-speaking community while maintaining clean, maintainable code structure.