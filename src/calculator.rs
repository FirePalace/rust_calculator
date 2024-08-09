#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(i64),
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
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr: &str = expr.as_ref();
        let chars: std::str::Chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parenthesies: Vec<char> = Vec::new();
        let mut negative_number: i64 = 0;

        let mut is_first_char: bool = true;
        let mut is_negative_number: bool = false;
        let mut is_first_number: bool = true;

        for c in chars {
            //Handle negative numbers at the beginning of the string
            if is_first_char && c == '-' {
                is_first_char = false;
                is_negative_number = true;
                continue;
            }
            is_first_char = false;
            if is_negative_number {
                if !c.is_numeric() {
                    is_negative_number = false;

                    negative_number = -negative_number;
                    tokens.push(Token::Number(negative_number));
                    match c {
                        '+' => tokens.push(Token::Op(Operator::Add)),
                        '-' => tokens.push(Token::Op(Operator::Sub)),
                        '*' => tokens.push(Token::Op(Operator::Mul)),
                        '/' => tokens.push(Token::Op(Operator::Div)),
                        _ => {}
                    }

                    continue;
                }
                if is_first_number {
                    let digit: i64 = c as i64 - 48;
                    negative_number = digit;
                    is_first_number = false;
                } else {
                    let digit = c as i64 - 48;
                    negative_number = negative_number * 10 + digit;
                }
                continue;
            }

            //Handle everything else

            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as i64 - 48);
                    }
                    _ => {
                        let digit = c as i64 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parenthesies.push(c);
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
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }
        if !parenthesies.is_empty() {
            return Err(Error::MismatchedParenthesies);
        }
        Ok(tokens)
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

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right)
                }
                Token::Op(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right)
                }
                Token::Op(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right)
                }
                Token::Op(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right)
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

#[cfg(test)]
mod tests {
    use crate::{Calculator, Error};

    #[test]
    fn basic() -> Result<(), Error> {
        let tokens = Calculator::parse("1 +1-4     +54444");
        let expr = Calculator::expression(tokens?);

        if let Some(v) = Calculator::evaluate(expr) {
            assert_eq!(v, 54442.0);
        }

        Ok(())
    }
    #[test]
    #[ignore = "unimplemented"]
    fn point_before_line_calculation() {}
}
