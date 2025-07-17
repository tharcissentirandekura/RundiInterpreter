# RundiInterpreter

RundiInterpreter is a simple interpreter written in Rust. It is designed to parse and evaluate expressions, providing a REPL (Read-Eval-Print Loop) for interactive use.

## Overview

This project consists of several modules that handle different aspects of the interpreter:

- **Tokenizer**: Breaks source code into tokens.
- **Parser**: Parses tokens into an abstract syntax tree (AST).
- **Evaluator**: Executes the AST.
- **Environment**: Manages variable scopes.
- **REPL**: Provides an interactive command-line interface.
- **Error Handling**: Defines and manages errors that occur during interpretation.
- **Abstract Syntax Tree (AST)**: Represents the structure of parsed expressions.

## Installation

To build and run the project, ensure you have Rust and Cargo installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Clone the repository:

```
git clone <repository-url>
cd RundiInterpreter
```

Then, build the project:

```
cargo build
```

## Usage

To run the interpreter, execute the following command:

```
cargo run
```

This will start the REPL, where you can enter expressions to be evaluated.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.