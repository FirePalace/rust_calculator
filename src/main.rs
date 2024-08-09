use std::process::ExitCode;

use calculator::{Calculator, Error};

mod calculator;

fn main() -> Result<ExitCode, Error> {
    println!("Type q to quit.");

    loop {
        let mut input: String = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "q" || input.trim() == "Q" {
                    println!("The programm will now exit!");
                    return Ok(ExitCode::from(0));
                }
                let tokens: Result<Vec<calculator::Token>, Error> = Calculator::parse(input.trim());
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr: Vec<calculator::Token> = Calculator::expression(tokens?);
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("{}", v);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
