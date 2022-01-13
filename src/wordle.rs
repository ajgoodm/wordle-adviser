use std::collections::{HashMap, HashSet};
use std::{fmt, convert::TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref MATCH_OPTION_DISPLAY: HashMap<MatchOption, String> = vec![
        (MatchOption::Correct, "🟩".to_string()),
        (MatchOption::WrongPlace, "🟨".to_string()),
        (MatchOption::WrongLetter, "⬜".to_string())
    ].into_iter().collect();

    static ref ALL_WORDS: Vec<String> = BufReader::new(
        File::open(
            "src/data/five_letter_scrabble.txt"
        ).unwrap()
    ).lines().map(|line| line.unwrap()).collect();
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MatchOption {
    Correct,
    WrongPlace,
    WrongLetter
}


#[derive(Debug, Clone)]
pub struct BadInput;


impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "guesses must be exactly 5 characters!")
    }
}


pub struct Feedback {
    pattern: [MatchOption; 5],
}


impl Feedback {
    pub fn from_vec(v: Vec<MatchOption>) -> Result<Feedback, BadInput> {
        if v.len() != 5 {
            Err(BadInput)
        } else {
            Ok(
                Feedback {
                    pattern: v.try_into().unwrap(),
                }
            )
        }
    }
}


pub struct Guess {
    word: [char; 5],
    feedback: Option<Feedback>
}


impl Guess {
    pub fn from_string(s: String) -> Result<Guess, BadInput> {
        let word: Vec<char> = s.chars().collect();
        if word.len() != 5 {
            Err(BadInput)
        } else {
            Ok(
                Guess {
                    word: word.try_into().unwrap(),
                    feedback: None
                }
            )
        }
    }
}


pub struct Matcher {
    cannot_be: Vec<HashSet<char>>,
    must_be: Vec<Option<char>>,
    must_contain: HashSet<char>
}


impl Matcher {
    pub fn new() -> Matcher {
        Matcher {
            cannot_be: vec![HashSet::new(); 5],
            must_be: vec![None; 5],
            must_contain: HashSet::new()
        }
    }

    pub fn update(&mut self, guess: Guess) {
        match guess.feedback {
            Some(feedback) => {
                for idx in 0..5 {
                    match feedback.pattern[idx] {
                        MatchOption::Correct => self.must_be[idx] = Some(guess.word[idx]),
                        MatchOption::WrongPlace => {
                            let c = guess.word[idx];
                            self.cannot_be[idx].insert(c);
                            self.must_contain.insert(c);
                        },
                        MatchOption::WrongLetter => {
                            for set in self.cannot_be.iter_mut() {
                                set.insert(guess.word[idx]);
                            }
                        }
                    }
                }
            },
            None => ()
        }
    }

    pub fn is_match(&self, word: &str) -> bool {
        assert_eq!(word.len(), 5);
        for (idx, c) in word.chars().enumerate() {
            match self.must_be[idx] {
                Some(must_be_this_char) => {
                    if c != must_be_this_char {
                        return false
                    }
                },
                None => ()
            }

            if self.cannot_be[idx].contains(&c) {
                return false
            }
        }

        let word_chars = word.chars().collect::<HashSet<char>>();
        if !self.must_contain.is_subset(&word_chars) {
            return false
        }

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let guess = Guess::from_string("hello".to_string()).unwrap();
        assert_eq!(guess.word, ['h', 'e', 'l', 'l', 'o']);

        match Guess::from_string("too_long".to_string()) {
            Ok(_) => assert!(false, "from_string should have raised an error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_is_match() {
        let mut matcher = Matcher::new();
        let mut guess = Guess::from_string("HELLO".to_string()).unwrap();
        guess.feedback = Some(
            Feedback::from_vec(vec![
                MatchOption::WrongLetter,
                MatchOption::Correct,
                MatchOption::Correct,
                MatchOption::Correct,
                MatchOption::Correct,
            ]).unwrap()
        );

        matcher.update(guess);
        assert!(matcher.is_match("JELLO"));
        assert!(!matcher.is_match("HELLO"))
    }
}