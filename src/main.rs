pub mod wordle;

use std::io;

use wordle::{BadInput, Guess, Matcher, ALL_WORDS};


fn main() -> Result<(), BadInput> {
    let mut remaining_words: Vec<&String> = ALL_WORDS.iter().collect();
    let mut matcher= Matcher::new();

    loop {
        println!("\nWhat word do you guess?");
        let guess_string = read_line_from_std_in().unwrap();
        let mut guess = Guess::from_string(guess_string)?;

        println!("What was the wordle feedback? - * -> correct, ~ -> wrong location, x -> wrong letter");
        let feedback_string = read_line_from_std_in().unwrap();
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
    *s = s [..].to_lowercase();
}


fn read_line_from_std_in() -> Result<String, io::Error> {
    let mut result = String::new();
    io::stdin().read_line(&mut result)?;
    clean_input_string(&mut result);
    Ok(result)
}