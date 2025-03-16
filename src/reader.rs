use rustyline::DefaultEditor;
use crate::environment::Environment;
use crate::Tokenizer::tokenize;
use crate::parser::*;
pub fn repl() {
    let mut env = Environment::new();
    let mut rl = DefaultEditor::new().unwrap();
    
    loop {
        let input = rl.readline("miischeme> ");
        match input {
            Ok(line) => {
                let tokens = tokenize(&line);
                let ast = parse(&tokens); // You'd need to implement a `parse` function
                // let result = eval(&ast, &mut env);
                println!("{:?}", ast);
            }
            Err(_) => break,
        }
    }
}
