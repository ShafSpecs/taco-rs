use std::os::windows::process;

use crate::{
    error::interpreter::throw_runtime_error, lang::taco::Taco, parser::parser::Parser,
    token::scanner::Scanner,
};

pub fn exec(source: String, lang: &mut Taco) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.clone());
    let expr = parser.parse();

    match expr {
        Ok(_) => (),
        Err(_e) => {
            lang.set_error(true);
            // std::process::exit(65);
            return;
        }
    }

    let err = lang.interpreter().interpret(expr.clone());

    match err {
        Ok(_) => {
            if lang.has_error {
                // std::process::exit(65);
                return;
            };

            ()
        }
        Err(e) => {
            throw_runtime_error(e);
            lang.set_runtime_error(true);
            // std::process::exit(70);
            return;
        }
    }

    // println!("{:?}\n\n", tokens.clone());
    // println!("{:?}\n\n", expr.clone().unwrap());
}
