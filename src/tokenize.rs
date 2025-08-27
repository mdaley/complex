use num_complex::Complex;
use crate::parse_complex::from_str;

#[derive(Debug)]
pub enum Token {
    OpenVector,
    CloseVector,
    Comma,
    ComplexNumber(Complex<f64>)
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut pos = 0;

    for c in input.chars() {
        pos += 1;

        match c {
            '[' => {
                tokens.push(Token::OpenVector);
                buffer.clear();
            },
            ']' => {
                if !buffer.is_empty() {
                    let c = from_str(buffer.as_str());
                    match c {
                        Ok(c) => {
                            tokens.push(Token::ComplexNumber(c));
                        },
                        Err(s) => {
                            println!("Error at line {}", pos);
                        }
                    }
                    //
                }
                buffer.clear();
                tokens.push(Token::CloseVector);
            },
            ',' => {
                tokens.push(Token::Comma);
                buffer.clear();
            },
            _ => {
                if !c.is_whitespace() {
                    buffer.push(c);
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tokenize_test() {
        let result = tokenize("[[1 + 2i]]");
        println!("result = {:?}", result.unwrap());
    }
}

