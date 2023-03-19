#[macro_use]
mod parser;
mod lang;
mod token;
mod error;
mod run;
mod core;

use crate::lang::taco::Taco;

fn main() {
    let mut args = std::env::args();

    if args.len() > 2 {
        println!("Usage: taco [file]");
    } else if args.len() == 2 {
        Taco::run_file(args.nth(1).unwrap());
    } else {
        Taco::run_repl();
    }

    println!("Done!")
}
