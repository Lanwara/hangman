use super::HangmanGame;
use std::io;

#[derive(PartialEq)]
pub enum GuessOutcome {
    Correct(char),
    Incorrect,
    GameWin,
    GameLose,
}

pub trait Handler {
    fn get_guess(&self, prompt: Option<&str>) -> char;
    fn handle_guess(&mut self, max_incorrect_guess_count: u32) -> GuessOutcome;
}

impl Handler for HangmanGame {
    fn get_guess(&self, prompt: Option<&str>) -> char {
        let default_prompt = "Guess a letter below";

        let prompt = match prompt {
            Some(value) => value,
            None => &default_prompt,
        };

        loop {
            println!("{}", &prompt);

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read stdin.");

            guess = guess.trim().to_string().to_ascii_uppercase();
            match guess.chars().count() {
                0 => {
                    println!("You have not typed in anything. Try again.");
                    continue;
                }
                1 => {
                    // Safe to unwrap here because we KNOW it is 1 character from the match arm.
                    let ch = guess.chars().next().unwrap();
                    break ch;
                }
                _ => {
                    println!("You have typed in more than one character. Try again.");
                    continue;
                }
            }
        }
    }

    fn handle_guess(&mut self, max_incorrect_guess_count: u32) -> GuessOutcome {
        let guess = self.get_guess(Some("Enter a letter below."));

        if !self.word_to_guess.contains(guess) {
            self.incorrect_guess_count += 1;

            let has_exceeded_max_guess_count =
                self.incorrect_guess_count >= max_incorrect_guess_count;

            return match has_exceeded_max_guess_count {
                true => GuessOutcome::GameLose,
                false => GuessOutcome::Incorrect,
            };
        }

        self.letters_to_guess.insert(guess, true);

        match self.all_correct() {
            true => GuessOutcome::GameWin,
            false => GuessOutcome::Correct(guess),
        }
    }
}
