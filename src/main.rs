#![feature(is_some_and)]

use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    println!("Monkey Interpreter");
    let stdin = std::io::stdin();
    for line in stdin.lines() {
        if let Ok(input) = line {
            let lexer = Lexer::new(&input);
            for token in lexer {
                println!("{token:?}");
            }
        }
    }
}
