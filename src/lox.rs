use anyhow::Result;
use std::io::{self, Write};
use std::{fs, process};

#[derive(Debug, Clone)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[Line {line}] Error {location}: {message}");
        self.had_error = true;
    }

    pub fn run_file(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.run_source(content);

        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut line = String::new();
            let bytes_read = io::stdin().read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            if !line.trim().is_empty() {
                self.run_source(line);
                self.had_error = false; // Reset in REPL
            }
        }
        println!("Bye bye from rlox 🤓");
        Ok(())
    }

    pub fn run_source(&mut self, source: String) {
        println!("{source}");
    }
}
