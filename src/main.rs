use anyhow::{Context, Result};
use rlox::lox::Lox;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}
