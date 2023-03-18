use super::scanner::Scanner;

pub fn run(source: String) {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens();
  println!("{:#?}", tokens)
}