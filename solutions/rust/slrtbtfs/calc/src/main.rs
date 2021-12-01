use std::process::exit;

use crate::parser::Parser;

mod lexer;
mod location;
mod parser;
mod syntax_tree;
#[cfg(test)]
mod test;

fn main() {
    let mut expressions: Vec<String> = vec![];

    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "--help" => print_help_and_exit(),
            _ => expressions.push(argument),
        }
    }

    if expressions.is_empty() {
        println!("No expressions provided.");
        print_help_and_exit();
    }

    for expression in expressions {
        let result = Parser::eval(&expression);
        match result {
            Ok(value) => println!("{} = {}", expression, value),
            Err(error) => println!("Error on input\n{}\n{}", expression, error),
        }
    }
}

/**
 * Helper for printing usage information.
 */
fn print_help_and_exit() {
    println!(
        "
    usage: calc [flags] [expressions]
    
    where possible flags are:
        --help           Print this help message."
    );
    exit(0);
}
