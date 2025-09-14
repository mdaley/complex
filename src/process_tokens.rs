use crate::complex::Complex;

use crate::tokenize::Token;

/*pub fn process(tokens: &mut Vec<Token>) -> Result<Complex, String> {
    match processRecursively(tokens) {
        Ok(_) => {
            if tokens.len() == 1 {
                match tokens.pop() {
                    Some(Token::ComplexNumber(c)) => {
                        Ok(c)
                    }
                    _ => {
                        Err("Remaining token isn't a complex number".to_string())
                    }
                }
            } else {
                Err("there should only be one token remaining".to_string())
            }
        },
        Err(e) => {
            Err(e)
        }
    }
}*/

/*fn processRecursively(tokens: &mut Vec<Token>) -> Result<(), String> {
    for t in tokens {
        
    }
}*/