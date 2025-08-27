use std::io::{self, Write};
use complex::parse_complex::from_str;


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
            let c = from_str(&input);
            println!("Complex: {:?}", c);
        }
    }
}
