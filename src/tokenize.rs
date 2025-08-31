use std::fmt;

use num_complex::Complex;

use crate::display_complex::ComplexDisplay;
use crate::parse_complex::from_str;

#[derive(Debug)]
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
    ComplexNumber(Complex<f64>)
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Minus => 1,
            Token::Plus => 2,
            Token::Multiply | Token::Divide => 2,
            _ => 0
        }
    }

    pub fn is_left_associative(&self) -> bool {
        matches!(self, Token::Power)
    }

    pub fn to_symbol(&self) -> char {
        match self {
            Token::Plus => '+',
            Token::Multiply => '*',
            _ => '#'
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ComplexNumber(c) => {
                write!(f, "{}", c.to_std_string())
            },
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

    for c in input.chars() {
        pos += 1;

        if capturing_complex {
            buffer.push(c);
            if c == '}' {
                match from_str(buffer.as_str()) {
                    Ok(complex) => {
                        tokens.push(Token::ComplexNumber(complex));
                        buffer.clear();
                    },
                    Err(s) => {
                        println!("Error at char {}: {}", pos, s);
                        break;
                    }
                }
                buffer.clear();
                capturing_complex = false;
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
                },
                '-' => {
                    tokens.push(Token::Minus);
                },
                '*' => {
                    tokens.push(Token::Multiply);
                },
                '/' => {
                    tokens.push(Token::Divide);
                },
                ')' => {
                    tokens.push(Token::LeftParen);
                },
                '(' => {
                    tokens.push(Token::RightParen);
                },
                '~' => {
                    tokens.push(Token::Conjugate);
                },
                '`' => {
                    tokens.push(Token::Transpose);
                },
                '[' => {
                    tokens.push(Token::OpenVector);
                },
                ']' => {                    
                    tokens.push(Token::CloseVector);
                },
                ',' => {
                    tokens.push(Token::Comma);
                },
                _ => {
                    if !c.is_whitespace() {
                        // hmm... probably will be a variable and a function...
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
            },
            Err(s) => {
                println!("Error at position {}", pos);
            }
        }
    }

    Ok(tokens)
}

pub fn shunting_yard(input: Vec<Token>) ->  Vec<Token> {
    let mut operators: Vec<Token> = Vec::new();
    let mut output: Vec<Token>  = Vec::new();


    // TODO: Implement the rest of the algorithm from 
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    for t in input {
        if let Token::ComplexNumber(_) = t {
            output.push(t);
        } else {
            let o1 = t;
            let o2 = operators.last();

            match o2 {
                Some(o2) => {
                    if o1.precedence() < o2.precedence() {
                        output.push(operators.pop().unwrap());
                        operators.push(o1);
                    } else {
                        operators.push(o1);
                    }
                },
                None => {
                    operators.push(o1);
                }
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
        let result = tokenize("{2 + 3i} + @{2, 1}\n");
        println!("result = {:?}", result.unwrap());
    }

     #[rstest(
        input, expected,
        case::simple_plus("{1} + {2}", "{1} {2} +"),
        case::plus_then_times("{1} + {2} * {3}", "{1} {2} {3} * +")
    )]
    fn shunting_works(input: &str, expected: &str) {
        let tokens = tokenize(input).unwrap();
        let result = shunting_yard(tokens);

        let result = result.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(" ");

        assert_eq!(expected, result);
    }

}

