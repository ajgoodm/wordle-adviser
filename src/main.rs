pub mod wordle;

use std::io;

use wordle::Guess;


fn main() {
    println!("What's your first guess?");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line from stdin");
    Guess::from_string(guess);
}
