use crate::complex::Complex;

use crate::tokenize::Token;

pub fn process(tokens: &mut Vec<Token>) -> Result<Complex, String> {
    match process_recursively(tokens) {
        Ok(Token::ComplexNumber(c)) => {
            Ok(c)
        },
        Err(e) => {
            Err(e)
        },
        _ => {
            Err("recursive processing returned a non-complex number token".to_string())
        }
    }
}

fn process_recursively(tokens: &mut Vec<Token>) -> Result<Token, String> {
    if tokens.len() == 1 {
        match tokens.pop() {
            Some(token @ Token::ComplexNumber(_)) => {
                Ok(token)
            },
            _ => {
                Err("Single remaining token must be a complex number".to_string())
            }
        }
    } else {
        // evaluate tokens down to a final number
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i] {
                Token::Plus => {
                    if i < 2 {
                        return Err("Not enough operands before operator".to_string());
                    }

                    let left = extract_complex(&tokens[i - 2]);
                    let right = extract_complex(&tokens[i - 1]);

                    let result = left.unwrap().add(right.unwrap());

                    let result_token = Token::ComplexNumber(result.unwrap());

                    tokens.splice(i - 2..=i, [result_token]);

                    i = 0;
                },
                Token::Minus => {

                },
                _ => {
                    i += 1;
                }
            }
        }

        Ok(tokens.pop().unwrap())

        //Err("Not yet implemented".to_string())

    }
}

fn extract_complex(token: &Token) -> Result<Complex, String> {
    match token {
        Token::ComplexNumber(c) => Ok(*c),
        _ => Err("Token does not contain a complex number".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::tokenize::{shunting_yard, tokenize};

    #[rstest(
        input, expected,
        case::single_number("{2 + i}", "{2 + i}"),
        case::plus("{2} + {i}}", "{2 + i}"),
        case::double_plus("{2} + {i} + {2 + i}", "{4 + 2i}"),
        case::multiple_plus("{2} + {i} + {-2} + {-i} + {3 - 3i} + {-3 + 3i}", "{0}")
    )]
    fn test_processing(input: &str, expected: &str) {
        let tokenized = tokenize(input).unwrap();
        let mut shunted = shunting_yard(tokenized);

        let result = process(&mut shunted).unwrap();

        println!("result = {}", result);
    }

}