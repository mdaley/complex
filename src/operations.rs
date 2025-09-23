/*use crate::{complex::Complex, tokenize::Token};


pub trait Operation {
    fn execute(&self, args: &[Token]) -> Result<Token, String>;
}

struct AddComplex;

impl Operation for AddComplex {
    fn execute(&self, args: &[Token]) -> Result<Token, String> {
        match args {
            [Token::ComplexNumber(a), Token::ComplexNumber(b)] => {
                match a.add(*b) {
                    Some(c) => {
                        Ok(Token::ComplexNumber(c))
                    },
                    None => {
                        Err("could not add complex numbers".to_string())
                    }
                }
            },
            _ => {
                Err("operands are not complex numbers".to_string())
            }
        }
    }
}

pub fn get_operation(token: Token) -> Box<dyn Operation> {
    match token {
        Token::Plus => Box::new(AddComplex),
        _ => panic!("Arrgh")
    }
}*/