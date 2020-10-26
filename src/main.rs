use std::{env, process};
use minigrep::lib::{Config, ConfigParsingError};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        match err {
            ConfigParsingError::TwoArgumentsExpected => println!("Expected two arguments."),
            ConfigParsingError::WrongCaseInsensitiveFlagUsage(first_arg) => println!("Wrong usage of case sensitive flag. Expected '--case-sensitive', got: {}", first_arg)
        }

        process::exit(1)
    });
    
    match minigrep::run(config) {
        Ok(results) => println!("Matched results: {:?}", results),
        Err(error) => println!("Application error: {}", error)
    }
}
