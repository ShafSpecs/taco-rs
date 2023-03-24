#[macro_use]
mod parser;
mod lang;
mod token;
mod error;
mod run;
mod core;
mod interpreter;
mod environment;
mod util;

use crate::lang::taco::Taco;

fn main() {
    let mut args = std::env::args();
    let mut lang = Taco::new();

    if args.len() > 2 {
        println!("Usage: taco [file]");
    } else if args.len() == 2 {
        lang.run_file(args.nth(1).unwrap());
    } else {
        lang.run_repl();
    }

    println!("Done!")
}
