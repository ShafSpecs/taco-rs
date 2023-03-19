use crate::token::scanner::Scanner;

pub fn exec(source: String) {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens();
  println!("{:#?}", tokens)
}