use complex::process_tokens;
use complex::tokenize::shunting_yard;
use complex::tokenize::tokenize;
use process_tokens::process;
use std::io::{self, Write};

fn main() {
    println!("Complex shell!");

    let mut running = true;

    while running {
        print!("c$ ");
        io::stdout().flush().expect("flushed");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("Input is {}", input);

        if input.trim().to_lowercase() == "quit" {
            println!("Bye...");
            running = false;
        } else {
            let tokenized = tokenize(&input);
            println!("Tokenized = {:?}", tokenized);

            let mut shunted = shunting_yard(tokenized.unwrap());
            println!("Shunted = {:?}", shunted);

            let r = process(&mut shunted);
            match r {
                Ok(c) => {
                    println!("Result = {}", c);
                },
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            }
        }
    }
}
