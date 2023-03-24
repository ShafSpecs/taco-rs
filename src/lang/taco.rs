use std::fs::read;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{interpreter::interpreter::Interpreter, run::exec, parser::parser::Parser};

pub struct Taco {
    pub has_error: bool,
    pub has_runtime_error: bool,
    pub interpreter: Option<Interpreter>,
    pub parser: Option<Parser>
}

impl Taco {
    pub fn new() -> Taco {
        Taco {
            has_error: false,
            has_runtime_error: false,
            interpreter: None,
            parser: None
        }
    }

    pub fn interpreter(&mut self) -> &mut Interpreter {
        if self.interpreter.is_none() {
            self.interpreter = Some(Interpreter::new(self));
        }

        self.interpreter.as_mut().unwrap()
    }

    pub fn set_error (&mut self, value: bool) {
        self.has_error = value;
    }

    pub fn set_runtime_error (&mut self, value: bool) {
        self.has_runtime_error = value;
    }

    pub fn run_file(&mut self, path: String) {
        let bytes = read(path).unwrap();

        let source = String::from_utf8(bytes).unwrap();

        exec(source, self);

        if self.has_error {
            std::process::exit(65);
        }
        if self.has_runtime_error {
            std::process::exit(70);
        }

        todo!("Remove the code splurging (in `exec`) and handle the error better");
    }

    pub fn run_repl(&mut self) {
        let mut rl = DefaultEditor::new().unwrap();

        // Used to create an infinite loop
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    exec(line, self);
                    self.has_error = false;
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
