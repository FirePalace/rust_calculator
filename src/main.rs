use regex::Regex;
use std::io;

fn main() {
    let mut quit: bool = false;
    let mut called_once: bool = false;

    let re_number = Regex::new(r"\d+(\.\d+)?").unwrap();
    let re_operator = Regex::new(r"[+\-*/]").unwrap();
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
            if input_is_valid(&input, &re_number, &re_operator) {
                println!("input is valid");

                let tokens = tokenize(&input.trim());
                let mut result = 0.0;
                let mut current_operator = '+';

                for token in tokens {
                    if re_number.is_match(&token) {
                        let number: f64 = token.parse().unwrap();

                        match current_operator {
                            '+' => result += number,
                            '-' => result -= number,
                            '*' => result *= number,
                            '/' => result /= number,
                            _ => panic!("Unknown operator: {}", current_operator),
                        }
                    } else if re_operator.is_match(&token) {
                        current_operator = token.chars().next().unwrap();
                    } else {
                        panic!("Unknown token: {}", token);
                    }
                }
                println!("Result: {}", result);
            } else {
                println!("input is not valid")
            }
        }
    }
}

fn input_is_valid(input: &String, re_number: &Regex, re_operator: &Regex) -> bool {
    //TODO
    let tokens = tokenize(&input.trim());
    for token in tokens {
        if !re_number.is_match(&token) && !re_operator.is_match(&token) {
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
