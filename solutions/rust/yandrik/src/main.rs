use std::io;
use std::io::Write;

use over_complicated_calculator::calculate;

fn main() {
    println!("Hi there! Welcome to OverComplicatedCalculator v1.0!");
    println!("To have OCC calculate something, just type it in!");
    loop {
        print!(" |> ");
        io::stdout().flush().expect("Couldn't flush output!");
        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Couldn't read line, please try again later!");
        match calculate(&expression.trim()) {
            Ok(res) => println!("ans: {}", res),
            Err(err) => println!("Expression couldn't be parsed: {:?}", err)
        }
    }
}
