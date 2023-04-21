mod hangman;
mod statics;

use rand::seq::SliceRandom;

use hangman::{
    handler::{GuessOutcome, Handler},
    HangmanGame,
};
use statics::{MAX_INCORRECT_GUESS_COUNT, WORDS};

fn main() {
    let random_word = WORDS
        .choose(&mut rand::thread_rng())
        .expect("Failed to fetch a random word.")
        .to_string();

    let mut game = HangmanGame::new(random_word);

    // println!("[DEBUG] Your word is: {}", &game.word_to_guess);
    println!("Your word is: {}", &game.format_word());

    loop {
        let prompt = format!(
            "({}/{}) Letter",
            game.incorrect_guess_count, MAX_INCORRECT_GUESS_COUNT
        );

        let outcome = game
            .handle_guess(Some(prompt.as_str()), MAX_INCORRECT_GUESS_COUNT)
            .expect("Could not handle user input.");

        match outcome {
            GuessOutcome::Correct(_) => {
                println!("Correct!   {}", game.format_word());
                continue;
            }
            GuessOutcome::Incorrect => {
                println!("Incorrect! {}", game.format_word());
                continue;
            }
            GuessOutcome::GameWin => {
                println!("Yay! The word was {}", game.word_to_guess);
                break;
            }
            GuessOutcome::GameLose => {
                println!("YOU LOSE! The word was {}", game.word_to_guess);
                break;
            }
        }
    }
}
