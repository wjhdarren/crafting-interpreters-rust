use anyhow::{Context, Result};
use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            process::exit(64); // EX_USAGE from sysexits.h
        }
    }
}

fn run_file(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    run_source(content)
}

fn run_prompt() -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        if let Err(e) = run_source(line) {
            eprintln!("Error: {e}");
        }
    }
    Ok(())
}

fn run_source(source: String) -> Result<()> {
    println!("{source}");
    Ok(())
}
