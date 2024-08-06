use lazy_static::lazy_static;
use regex::Regex;
use std::io;

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
    static ref RE_OPERATOR: Regex = Regex::new(r"[+\-*/]").unwrap();
}

fn main() {
    let mut quit: bool = false;
    let mut called_once: bool = false;

    while !quit {
        if !called_once {
            println!("You can type q to quit the program");
            called_once = true;
        }

        println!("Please enter an expression:");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" || input.trim() == "Q" {
            println!("The Programm will now exit");
            quit = true;
        } else {
            if input_is_valid(&input) {
                println!("input is valid");

                let result: f64 = calculate_result(&input);

                println!("Result: {}", result);
            } else {
                println!("input is not valid")
            }
        }
    }
}

fn calculate_result(input: &String) -> f64 {
    let mut current_operator: char = '+';
    let tokens: Vec<String> = tokenize(&input.trim());
    let mut result: f64 = 0.0;
    for token in tokens {
        if RE_NUMBER.is_match(&token) {
            let number: f64 = token.parse().unwrap();

            match current_operator {
                '+' => result += number,
                '-' => result -= number,
                '*' => result *= number,
                '/' => result /= number,
                _ => panic!("Unknown operator: {}", current_operator),
            }
        } else if RE_OPERATOR.is_match(&token) {
            current_operator = token.chars().next().unwrap();
        } else {
            panic!("Unknown token: {}", token);
        }
    }
    return result;
}

fn input_is_valid(input: &String) -> bool {
    let tokens = tokenize(&input.trim());
    for token in tokens {
        if !RE_NUMBER.is_match(&token) && !RE_OPERATOR.is_match(&token) {
            return false;
        }
    }
    return true;
}
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            current_token.push(ch);
        } else if ch == ' ' {
            //Do nothing
        } else {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            tokens.push(ch.to_string());
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    return tokens;
}

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
