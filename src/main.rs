mod hangman;

use hangman::game_handler::{GuessOutcome, Handler};
use hangman::HangmanGame;

const WORD: &'static str = "YOU";
const MAX_INCORRECT_GUESSES: u32 = 10;

#[allow(dead_code)]
fn main() {
    let mut game = HangmanGame::new(WORD.to_string());

    loop {
        println!("Your word is: {}", &game.format_word());

        let outcome = game.handle_guess(MAX_INCORRECT_GUESSES);

        match outcome {
            GuessOutcome::Correct(_) => {
                println!("Correct");
                continue;
            }
            GuessOutcome::Incorrect => {
                println!("Incorrect");
                continue;
            }
            GuessOutcome::GameWin => {
                println!("WINNER!");
                break;
            }
            GuessOutcome::GameLose => {
                println!("LOSE");
                break;
            }
        }
    }
}
