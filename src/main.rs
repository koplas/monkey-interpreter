use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    println!("Monkey Interpreter");
    let stdin = std::io::stdin();
    for line in stdin.lines().flatten() {
        let lexer = Lexer::new(&line);
        for token in lexer {
            println!("{token:?}");
        }
    }
}
