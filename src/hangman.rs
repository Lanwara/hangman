use std::collections::{HashMap, HashSet};

pub mod game_handler;

#[derive(Clone)]
pub struct HangmanGame {
    pub word_to_guess: String,
    pub incorrect_guess_count: u32,
    letters_to_guess: HashMap<char, bool>,
}

impl HangmanGame {
    pub fn new(word: String) -> Self {
        let unique_characters: HashSet<char> = word.chars().collect();
        let letters_to_guess = unique_characters.iter().map(|&ch| (ch, false)).collect();

        Self {
            word_to_guess: word,
            incorrect_guess_count: 0,
            letters_to_guess,
        }
    }

    pub fn format_word(&self) -> String {
        let mut formatted_word = String::new();

        for ch in self.word_to_guess.chars() {
            if let Some(&true) = self.letters_to_guess.get(&ch) {
                formatted_word.push(ch);
            } else {
                formatted_word.push('_');
            }
        }
        formatted_word
    }

    pub fn all_correct(&self) -> bool {
        self.letters_to_guess.values().all(|&val| val)
    }
}
