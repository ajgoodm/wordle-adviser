pub mod wordle;

use std::io;

use wordle::{BadInput, Guess, Matcher, ALL_WORDS};


fn main() -> Result<(), BadInput> {
    let mut remaining_words: Vec<&String> = ALL_WORDS.iter().collect();
    let mut matcher= Matcher::new();

    loop {
        let mut guess_string = String::new();
        println!("What word do you guess?");
        io::stdin().read_line(&mut guess_string).expect("Failed to read line from stdin");
        clean_input_string(&mut guess_string);
        let mut guess = Guess::from_string(guess_string)?;

        let mut feedback_string = String::new();
        println!("What was the wordle feedback? - * -> correct, ~ -> wrong location, x -> wrong letter");
        io::stdin().read_line(&mut feedback_string).expect("Failed to read line from stdin");
        clean_input_string(&mut feedback_string);
        guess.update_with_feedback(feedback_string)?;
        matcher.update(guess);

        remaining_words = remaining_words.into_iter().filter(|w| matcher.is_match(w)).collect();
        for w in remaining_words.iter() {
            println!("{}", w);
        }
    }

}


fn clean_input_string(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
    *s =s[..].to_lowercase();
}