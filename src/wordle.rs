use std::{fmt, convert::TryInto};

enum Match {
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


pub struct Guess {
    word: [char; 5],
    feedback: Option<[Match; 5]>
}


impl Guess {
    pub fn from_string(s: String) -> Result<Guess, BadInput> {
        let word: Vec<char> = s.chars().collect();
        if word.len() != 5 {
            return Err(BadInput)
        } else {
            return Ok(
                Guess {
                    word: word.try_into().unwrap(),
                    feedback: None
                }
            )
        }
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
}