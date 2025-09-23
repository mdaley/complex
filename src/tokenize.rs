use std::fmt;

use crate::complex::Complex;

use crate::parse_complex::from_str;

#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Divide,
    Multiply,
    Power,
    LeftParen,
    RightParen,
    Conjugate,
    Transpose,
    OpenVector,
    CloseVector,
    Comma,
    Dot,
    ComplexNumber(Complex),
    Function(String, String),
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Minus => 1,
            Token::Plus => 2,
            Token::Multiply | Token::Divide => 3,
            _ => 0,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        matches!(self, Token::Power)
    }

    pub fn to_symbol(&self) -> String {
        match self {
            Token::Plus => "+".to_owned(),
            Token::Minus => "-".to_owned(),
            Token::Multiply => "*".to_owned(),
            Token::Divide => "/".to_owned(),
            Token::Dot => ".".to_owned(),
            Token::Function(f, args) => format!("{}{}", f.to_owned(), args.to_owned()),
            _ => "#".to_owned(),
        }
    }

    /*fn operation(&self) -> fn(u32, u32) -> u32 {
    match op {
        Operation::Plus => |a, b| a + b,
        Operation::Minus => |a, b| a - b,
        Operation::Multiply => |a, b| a * b,
        Operation::Divide => |a, b| {
            if b == 0 {
                panic!("Division by zero");
            }
            a / b
        },
    }*/
}


    /*pub fn operator(&self, token: Vec<Token>) ->  Fn{
        match self {
            Token::Plus => {

        }
    }*/
//}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ComplexNumber(c) => {
                let magnitude = f.width().unwrap_or(12);
                let precision = f.precision().unwrap_or(6);
                write!(f, "{}", c.to_std_string(magnitude, precision))
            }
            Token::Function(ftn, args) => {
                write!(f, "{}({})", ftn, args)
            }
            _ => {
                write!(f, "{}", self.to_symbol())
            }
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut pos = 0;
    let mut capturing_complex = false;
    let mut capturing_function = false;
    let mut function = String::new();
    let mut capturing_function_args = false;
    let mut function_bracket_nesting: u8 = 0;

    for c in input.chars() {
        pos += 1;

        if capturing_complex {
            buffer.push(c);
            if c == '}' {
                match from_str(buffer.as_str()) {
                    Ok(complex) => {
                        tokens.push(Token::ComplexNumber(complex));
                        buffer.clear();
                    }
                    Err(s) => {
                        println!("Error at char {}: {}", pos, s);
                        break;
                    }
                }
                buffer.clear();
                capturing_complex = false;
            }
        } else if capturing_function {
            if c == '(' {
                function = buffer.clone();
                buffer.clear();
                capturing_function = false;
                capturing_function_args = true;
                function_bracket_nesting = 1;
            } else {
                buffer.push(c);
            }
        } else if capturing_function_args {
            if c == ')' {
                function_bracket_nesting -= 1;
            } else if c == '(' {
                function_bracket_nesting += 1;
            }

            if function_bracket_nesting > 0 {
                buffer.push(c);
            } else {
                tokens.push(Token::Function(function.clone(), buffer.clone()));
                capturing_function_args = false;
                buffer.clear();
            }
        } else {
            match c {
                '{' | '@' => {
                    capturing_complex = true;
                    buffer.clear();
                    buffer.push(c);
                }
                '+' => {
                    tokens.push(Token::Plus);
                }
                '-' => {
                    tokens.push(Token::Minus);
                }
                '*' => {
                    tokens.push(Token::Multiply);
                }
                '/' => {
                    tokens.push(Token::Divide);
                }
                ')' => {
                    tokens.push(Token::RightParen);
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                }
                '~' => {
                    tokens.push(Token::Conjugate);
                }
                '`' => {
                    tokens.push(Token::Transpose);
                }
                '[' => {
                    tokens.push(Token::OpenVector);
                }
                ']' => {
                    tokens.push(Token::CloseVector);
                }
                ',' => {
                    tokens.push(Token::Comma);
                }
                _ => {
                    if !c.is_whitespace() {
                        capturing_function = true;
                        buffer.clear();
                        buffer.push(c);
                    }
                }
            }
        }
    }

    if !buffer.is_empty() {
        let c = from_str(buffer.as_str());
        match c {
            Ok(c) => {
                tokens.push(Token::ComplexNumber(c));
            }
            Err(_) => {
                println!("Error at position {}", pos);
            }
        }
    }

    Ok(tokens)
}

pub fn shunting_yard(input: Vec<Token>) -> Vec<Token> {
    let mut operators: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();

    // TODO: Implement the rest of the algorithm from
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    for o1 in input {
        match o1 {
            Token::ComplexNumber(_) => {
                output.push(o1);
            }
            Token::Function(_, _) => {
                operators.push(o1);
            }
            Token::LeftParen => {
                operators.push(o1);
            }
            Token::Comma => loop {
                let o2_opt = operators.last();
                match o2_opt {
                    Some(o2) => match o2 {
                        Token::LeftParen => {
                            break;
                        }
                        _ => {
                            output.push(operators.pop().unwrap());
                        }
                    },
                    None => {
                        break;
                    }
                }
            },
            Token::RightParen => loop {
                let o2_opt = operators.last();
                match o2_opt {
                    Some(o2) => {
                        if !matches!(o2, Token::LeftParen) {
                            output.push(operators.pop().unwrap());
                        } else {
                            operators.pop();
                        }
                    }
                    None => {
                        break;
                    }
                }
            },
            _ => {
                loop {
                    let o2_opt = operators.last();
                    match o2_opt {
                        Some(o2) => {
                            if !matches!(o2, Token::LeftParen)
                                && (o2.precedence() > o1.precedence()
                                    || (o1.is_left_associative()
                                        && o2.precedence() == o1.precedence()))
                            {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }

                operators.push(o1);
            }
        }
    }

    while let Some(o) = operators.pop() {
        output.push(o);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn tokenize_test() {
        //let result = tokenize("{2 + 3i} + @{2, 1}\n");
        let result = tokenize("{1} + inv(pow(fn({2}, {3}, {4})))");
        println!("result = {:?}", result.unwrap());
    }

    #[rstest(
        input,
        expected,
        case::simple_plus("{1} + {2}", "{1} {2} +"),
        case::plus_then_times("{1} + {2} * {3}", "{1} {2} {3} * +"),
        case::times_then_plus("{1} * {2} + {3}", "{1} {2} * {3} +"),
        case::plus_then_times_then_minux("{1} + {2} * {3} - {4}", "{1} {2} {3} * + {4} -"),
        case::unary_function("z({1})", "z({1})"),
        case::binary_function("pow({1}, {10})", "pow({1}, {10})"),
        case::multi_function("sum({1}, {2}, {3}, {4})", "sum({1}, {2}, {3}, {4})")
    )]
    fn shunting_works(input: &str, expected: &str) {
        let tokens = tokenize(input).unwrap();

        println!("TOKENS = {:?}", tokens);

        let result = shunting_yard(tokens);

        println!("SHUNTED = {:?}", result);

        let result = result
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        assert_eq!(expected, result);
    }
}
