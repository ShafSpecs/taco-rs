use std::fs::read;

use rustyline::{DefaultEditor, error::ReadlineError};

use crate::parser::run::run;

pub struct Taco {
    pub has_error: bool,
}

impl Taco {
    pub fn new() -> Taco {
        Taco { has_error: false }
    }

    pub fn run_file(path: String) {
        let bytes = read(path).unwrap();

        let source = String::from_utf8(bytes).unwrap();

        run(source);
        todo!("Remove the code splurging and handle the error better");
    }

    pub fn run_repl() {
        let mut rl = DefaultEditor::new().unwrap();

        // Used to create an infinite loop
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    run(line);
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("Error occured: {}", err);
                    break;
                }
            }
        }
    }
}
