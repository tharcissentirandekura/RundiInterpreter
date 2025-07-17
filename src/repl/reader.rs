// This file contains the implementation of the REPL, including functions for reading user input and evaluating expressions.

use std::io::{self, Write};

pub struct REPL;

impl REPL {
    pub fn new() -> Self {
        REPL
    }

    pub fn start(&self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input");
                continue;
            }

            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue; // Skip empty input
            }

            // Here you would typically evaluate the input
            // For now, we just print it back
            println!("You entered: {}", trimmed_input);
        }
    }
}