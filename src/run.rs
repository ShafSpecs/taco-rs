use std::ops::Deref;

use crate::{token::scanner::Scanner, parser::parser::Parser, lang::taco::Taco, error::handling::report};

pub fn exec(source: String, lang: &mut Taco) {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens();
  let mut parser = Parser::new(tokens.clone());
  let expr = parser.parse();

  match expr {
    Ok(_) => (),
    Err(e) => {
      // `throw_error` should handle this. 
      // eprintln!("{}", e);
      lang.set_error(true);
      return;
    }
  }

  let err = lang.interpreter().interpret(expr.clone());

  match err {
      Ok(_) => {
        if lang.has_error {
          std::process::exit(65);
        };

        ()
      },
      Err(e) => {
          report(e.get_token().line as u8, format!(" at '{}'", e.get_token().lexeme).as_str(), e.get_message());
          lang.set_runtime_error(true);
          return;
      }
  }

  println!("{:?}\n\n", tokens.clone());
  println!("{:?}\n\n", expr.clone().unwrap());
  // println!("{:?}\n\n", lang.interpreter().environment.clone());
}