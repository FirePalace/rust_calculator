#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mod,
    Mul,
    Div,
    Exp,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Token {
    Number(f64),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    BadToken(char),
    MismatchedParenthesies,
}

impl Calculator {
    pub fn sanitize_input(input: &str) -> String {
        let mut input_string = input.to_string();
        while let Some(index) = input_string.find("-(") {
            input_string.replace_range(index..index + 2, "-1*(");
        }
        while let Some(index) = input_string.find("(-") {
            input_string.replace_range(index..index + 2, "(0-");
        }

        input_string
    }
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr: &str = expr.as_ref();
        let chars: std::str::Chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parenthesies: Vec<char> = Vec::new();
        let mut negative_number: f64 = 0.0;

        let mut is_first_char: bool = true;
        let mut is_negative_number: bool = false;
        let mut is_first_number: bool = true;

        let mut is_after_decimal_point: bool = false;
        let mut decimal_point_count: i32 = 0;

        for c in chars {
            //Handle negative numbers at the beginning of the string
            if is_first_char && c == '-' {
                is_first_char = false;
                is_negative_number = true;
                continue;
            }
            is_first_char = false;
            if is_negative_number {
                if c == '.' {
                    is_after_decimal_point = true;
                    continue;
                }
                if !c.is_numeric() && c != '.' {
                    is_negative_number = false;
                    is_after_decimal_point = false;
                    decimal_point_count = 0;

                    negative_number = -negative_number;
                    tokens.push(Token::Number(negative_number));
                    match c {
                        '+' => tokens.push(Token::Op(Operator::Add)),
                        '-' => tokens.push(Token::Op(Operator::Sub)),
                        '%' => tokens.push(Token::Op(Operator::Mod)),
                        '*' => tokens.push(Token::Op(Operator::Mul)),
                        '/' => tokens.push(Token::Op(Operator::Div)),
                        '^' => tokens.push(Token::Op(Operator::Exp)),
                        _ => {}
                    }

                    continue;
                }
                if !is_after_decimal_point {
                    if is_first_number {
                        let digit: f64 = (c as u32 - 48) as f64;
                        negative_number = digit;
                        is_first_number = false;
                    } else {
                        let digit: f64 = (c as u32 - 48) as f64;
                        negative_number = negative_number * 10.0 + digit;
                    }
                } else {
                    let mut digit: f64 = (c as u32 - 48) as f64;
                    decimal_point_count += 1;
                    for _ in 0..decimal_point_count {
                        digit /= 10.0;
                    }
                    negative_number += digit;
                }

                continue;
            }

            //Handle everything else

            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        if !is_after_decimal_point {
                            *n = *n * 10.0 + (c as u32 - 48) as f64;
                        } else {
                            let mut digit = (c as u32 - 48) as f64;
                            decimal_point_count += 1;
                            for _ in 0..decimal_point_count {
                                digit /= 10.0;
                            }
                            *n += digit;
                        }
                    }
                    _ => {
                        let digit: f64 = (c as u32 - 48) as f64;
                        tokens.push(Token::Number(digit));
                    }
                },
                '^' => {
                    tokens.push(Token::Op(Operator::Exp));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '.' => {
                    is_after_decimal_point = true;
                }
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parenthesies.push(c);
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parenthesies.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParenthesies);
                        }
                    } else {
                        return Err(Error::MismatchedParenthesies);
                    }
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '+' => {
                    tokens.push(Token::Op(Operator::Add));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '-' => {
                    tokens.push(Token::Op(Operator::Sub));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '%' => {
                    tokens.push(Token::Op(Operator::Mod));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '*' => {
                    tokens.push(Token::Op(Operator::Mul));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '/' => {
                    tokens.push(Token::Op(Operator::Div));
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                ' ' => {
                    is_after_decimal_point = false;
                    decimal_point_count = 0;
                }
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }
        if !parenthesies.is_empty() {
            return Err(Error::MismatchedParenthesies);
        }

        let tokens2 = Calculator::fix_exponents(tokens);

        Ok(tokens2)
    }

    fn fix_exponents(mut tokens: Vec<Token>) -> Vec<Token> {
        let mut new_tokens = tokens.clone();

        let mut last_negative_number: f64 = 0.0;

        for (i, token) in new_tokens.iter_mut().enumerate() {
            if let Token::Number(n) = token {
                if *n <= 0.0 {
                    last_negative_number = *n;
                    continue;
                }
            }
            if let Token::Op(Operator::Exp) = token {
                if Token::Bracket(')') != tokens[i - 1] {
                    if last_negative_number != 0.0 {
                        tokens.insert(i - 1, Token::Bracket('('));
                        tokens.insert(i - 1, Token::Number(-1.0));
                        tokens.insert(i, Token::Op(Operator::Mul));
                        tokens.insert(i + 5, Token::Bracket(')'));
                    } else {
                        tokens.insert(i - 1, Token::Bracket('('));
                        tokens.insert(i + 3, Token::Bracket(')'));
                    }
                }
            }

            last_negative_number = 0.0;
        }

        tokens
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty()
                        && stack[stack.len() - 1] >= token
                        && matches!(stack[stack.len() - 1], Token::Op(_))
                    {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                _ => {}
            }
        }
        while let Some(element) = stack.pop() {
            queue.push(element);
        }
        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f64> {
        tokens.reverse();

        let mut stack: Vec<f64> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num),
                Token::Op(Operator::Add) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left + right)
                }
                Token::Op(Operator::Sub) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left - right)
                }
                Token::Op(Operator::Mul) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left * right)
                }
                Token::Op(Operator::Div) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left / right)
                }
                Token::Op(Operator::Exp) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left.powf(right))
                }
                Token::Op(Operator::Mod) => {
                    let right: f64 = stack.pop().unwrap();
                    let left: f64 = stack.pop().unwrap();
                    stack.push(left.rem_euclid(right))
                }
                _ => {}
            }
        }
        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Op(op) => write!(f, "{}", op),
            Token::Bracket(b) => write!(f, "{}", b),
        }
    }
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mod => write!(f, "%"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Exp => write!(f, "^"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Calculator, Error};

    #[test]
    fn basic() -> Result<(), Error> {
        test_template("1 +1-4     +54444", 54442.0)
    }
    #[test]
    fn negative_exp() -> Result<(), Error> {
        test_template("(-2)^2", 4.0)
    }
    #[test]
    fn negative_exp2() -> Result<(), Error> {
        test_template("-2^2", -4.0)
    }
    #[test]
    fn negative_exp3() -> Result<(), Error> {
        test_template("(-2*2)^2", 16.0)
    }
    #[test]
    fn modulo() -> Result<(), Error> {
        test_template("-4%23", 19.0)
    }
    fn test_template(input: &str, assertion: f64) -> Result<(), Error> {
        let input = Calculator::sanitize_input(input);
        let tokens: Result<Vec<super::Token>, Error> = Calculator::parse(input);
        let temp_tokens: &Result<Vec<crate::calculator::Token>, Error> = &tokens;

        println!("TOKENS:");
        if let Ok(tokens) = temp_tokens {
            for token in tokens {
                println!("{}", token);
            }
        }
        let expr: Vec<super::Token> = Calculator::expression(tokens?);
        println!("___________________");
        println!("RPN EXPRESSION:");

        for expre in &expr {
            println!("{}", expre);
        }

        if let Some(v) = Calculator::evaluate(expr) {
            assert_eq!(v, assertion);
        }
        Ok(())
    }
}
