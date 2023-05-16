// -----------------------------------------------------------------------

// standard libraries
use std::env::args;
use std::fs::read_to_string;
use std::io::{self, Write};

// project libraries
mod error;
mod literal;
mod scanner;
mod token;
mod token_type;

use error::LuxtError;
use scanner::Scanner;

// -----------------------------------------------------------------------
// this will be used for running an entire file using luxt

fn run_file(file_name: &str) -> io::Result<()> {
    let buffer = read_to_string(file_name)?;
    match run(buffer) {
        Ok(_) => println!("No error running file"),
        Err(luxt_err) => {
            luxt_err.report("where");
            std::process::exit(65);
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------
// this is for running individual lines using luxt

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;

        // only if there is the end of line character then break
        let mut input_buffer = String::new();
        if stdin.read_line(&mut input_buffer)? == 1 {
            break;
        }

        // note that here we dont want to exit since the user
        // is just running individual lines
        match run(input_buffer) {
            Ok(_) => println!("No error running line"),
            Err(luxt_err) => luxt_err.report("where"),
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------
// where all the meat of the program will go
fn run(source: String) -> Result<(), LuxtError> {
    // println!("source: {:?}", source);
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

// -----------------------------------------------------------------------

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);

    if args.len() > 2 {
        println!("Usage: luxt [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt().expect("Error while running luxt promt");
    }

    Ok(())
}
