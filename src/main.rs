use std::process::ExitCode;

use calculator::{Calculator,Error};

mod calculator;

fn main() -> Result<ExitCode, Error> {
    println!("Type q to quit.");

    println!("");
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "q" || input.trim() =="Q"{
                    println!("The Programm will now Exit!");
                    return Ok(ExitCode::from(0));
                }
                let tokens = Calculator::parse(input.trim());
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr = Calculator::expression(tokens?);
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("{}", v);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::calculate_result;
    #[test]
    fn basic() {
        let input: String = String::from("1 +1-4     +54444");

        let result = calculate_result(&input);
        assert!(result == 54442.0);
    }
    #[test]
    #[ignore = "unimplemented"]
    fn point_before_line_calculation() {
        let input: String = String::from("2+1*3");

        let result: f64 = calculate_result(&input);
        assert!(result == 5.0);

        let input: String = String::from("2+1/4");

        let result: f64 = calculate_result(&input);
        assert!(result == 2.25);
    }
}
*/
