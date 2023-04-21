use super::HangmanGame;

use inquire::{length, required, validator::Validation, InquireError, Text};

#[derive(PartialEq)]
pub enum GuessOutcome {
    Correct(char),
    Incorrect,
    GameWin,
    GameLose,
}

pub trait Handler {
    fn get_guess(&self, prompt: &str) -> Result<char, InquireError>;
    fn handle_guess(
        &mut self,
        prompt: Option<&str>,
        max_incorrect_guess_count: u32,
    ) -> Result<GuessOutcome, InquireError>;
}

impl Handler for HangmanGame {
    /// Note this returns the character in uppercase.
    fn get_guess(&self, prompt: &str) -> Result<char, InquireError> {
        let uppercase_ascii_validator =
            |input: &str| match input.chars().all(|ch| ch.is_ascii_uppercase()) {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid(
                    "Your guess should only contain capital ASCII characters".into(),
                )),
            };

        let guess = Text::new(&prompt)
            .with_validator(required!("You have to type something."))
            .with_validator(length!(1))
            .with_validator(uppercase_ascii_validator)
            .prompt()?
            .parse()
            .expect(
                "Somehow Rust fucking could not convert an ASCII string of a single character to a `char`",
            );

        Ok(guess)
    }

    fn handle_guess(
        &mut self,
        prompt: Option<&str>,
        max_incorrect_guess_count: u32,
    ) -> Result<GuessOutcome, InquireError> {
        let default_prompt = "Enter a letter";
        let prompt = match prompt {
            Some(value) => value,
            None => &default_prompt,
        };

        let guess = self.get_guess(&prompt)?;

        if !self.word_to_guess.contains(guess) {
            self.incorrect_guess_count += 1;

            let has_exceeded_max_guess_count =
                self.incorrect_guess_count >= max_incorrect_guess_count;

            return match has_exceeded_max_guess_count {
                true => Ok(GuessOutcome::GameLose),
                false => Ok(GuessOutcome::Incorrect),
            };
        }

        self.letters_to_guess.insert(guess, true);

        return match self.all_correct() {
            true => Ok(GuessOutcome::GameWin),
            false => Ok(GuessOutcome::Correct(guess)),
        };
    }
}
