use crate::{token::scanner::Scanner, parser::parser::Parser};

pub fn exec(source: String) {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens();
  let mut parser = Parser::new(tokens.clone());
  let expr = parser.parse();

  println!("{:?}\n\n", tokens.clone());
  println!("{:?}", expr.unwrap())
}