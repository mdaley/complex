use complex::tokenize::shunting_yard;
use complex::tokenize::tokenize;
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

            let shunted = shunting_yard(tokenized.unwrap());
            println!("Shunted = {:?}", shunted);


        }
    }
}
