use std::io::{self, Write};
use complex::tokenize::shunting_yard;
use complex::{parse_complex::from_str, tokenize::tokenize};
use complex::display_complex::ComplexDisplay;
use num_complex::Complex;


fn main() {
    println!("Complex shell!");

    let mut running = true;

    while running {
        print!("c$ ");
        io::stdout().flush().expect("flushed");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");

        println!("Input is {}", input);

        if input.trim().to_lowercase() == "quit" {
            println!("Bye...");
            running = false;
        } else {
            let tokenized = tokenize(&input);
            println!("Tokenized = {:?}", tokenized);

            let shunted = shunting_yard(tokenized.unwrap());
            println!("Shunted = {:?}", shunted);

            //let c = from_str(&input).unwrap();
            //println!("Complex, std: {}", c.to_std_string());
            //println!("Complex, plr: {}", c.to_polar_string());
        }
    }
}

