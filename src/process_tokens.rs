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
            match &tokens[i] {
                token @ (Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power) => {
                    if i < 2 {
                        return Err("Need two operands for + = * - or ^".to_string());
                    }

                    let left = extract_complex(&tokens[i - 2])?;
                    let right = extract_complex(&tokens[i - 1])?;

                    let result = match token {
                        Token::Plus => left.add(right),
                        Token::Minus => left.sub(right),
                        Token::Divide => left.div(right),
                        Token::Multiply => left.mul(right),
                        Token::Power => left.pow(right.re),
                        _ => panic!("Impossible operator")
                    };

                    let result_token = Token::ComplexNumber(result.unwrap());

                    tokens.splice(i - 2..=i, [result_token]);

                    i = 0;
                },/* 
                Token::Power => {
                    if i < 1 {
                        return Err("Need to operands before ^".to_string());
                    }

                    let operand = extract_complex(&tokens[i - 1])?;

                    let result  operand.
                },*/
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
        case::subtract("{3 + i} - {1 - i}", "{2 + 2i}"),
        case::multiple_plus("{2} + {i} + {-2} + {-i} + {3 - 3i} + {-3 + 3i}", "{0}"),
        case::multiply("{i} * {i}", "{-1}"),
        case::multiple_more("{1 + i} * {3 - i}", "{4 + 2i}"),
        case::divide("{4 + 2i} / {3 - i}", "{1 + i}")
    )]
    fn test_processing(input: &str, expected: &str) {
        let tokenized = tokenize(input).unwrap();
        let mut shunted = shunting_yard(tokenized);

        let result = process(&mut shunted).unwrap();

        assert_eq!(expected, result.to_string());
    }

}