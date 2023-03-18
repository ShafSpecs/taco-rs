pub fn report(line: u8, where_: &str, message: &str) {
  eprintln!("[line {}] Error {}: {}", line, where_, message);
}

pub fn error(line: u8, message: &str) {
  report(line, "", message);
}