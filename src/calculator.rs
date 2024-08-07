#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
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
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parenthesies = Vec::new();

        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
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
        if parenthesies.len() > 0 {
            return Err(Error::MismatchedParenthesies);
        }
        return Ok(tokens);
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token && matches!(stack[stack.len()-1], Token::Op(_)) {
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
        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        return queue;
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
            return None;
        } else {
            return stack.pop();
        }
    }
}
