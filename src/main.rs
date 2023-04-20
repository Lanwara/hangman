use std::{
    collections::{HashMap, HashSet},
    io,
};

const WORD: &'static str = "YOU";
const MAX_INCORRECT_GUESSES: u32 = 10;

fn get_guess() -> char {
    loop {
        println!("Guess a letter below.");

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

fn all_correct(map: &HashMap<char, bool>) -> bool {
    map.values().all(|&value| value)
}

fn format_word(word_to_guess: &str, letters_map: &HashMap<char, bool>) -> String {
    let mut formatted_word = String::new();

    for ch in word_to_guess.chars() {
        if let Some(true) = letters_map.get(&ch) {
            formatted_word.push(ch)
        } else {
            formatted_word.push('_')
        }
    }

    formatted_word
}
fn main() {
    let mut incorrect_guess_count: u32 = 0;
    let word_to_guess = WORD.to_string();
    let unique_chars: HashSet<char> = word_to_guess.chars().collect();

    let mut letters_to_guess: HashMap<char, bool> =
        unique_chars.iter().map(|&ch| (ch, false)).collect();

    loop {
        let formatted_word = format_word(&word_to_guess, &letters_to_guess);
        println!("Your word is: {formatted_word}");

        let guess = get_guess();

        if !word_to_guess.contains(guess) {
            incorrect_guess_count += 1;

            if incorrect_guess_count >= MAX_INCORRECT_GUESSES {
                println!("YOU LOST! You have made {incorrect_guess_count} incorrect guesses.");
                break;
            }

            println!(
                "Wrong letter. You have made {}/{} incorrect guesses. Try again.",
                incorrect_guess_count, MAX_INCORRECT_GUESSES
            );
            continue;
        }

        // From here the word DOES contain the guessed letter.
        println!("Correct!");
        letters_to_guess.insert(guess, true);

        if all_correct(&letters_to_guess) {
            println!("YOU WIN! You've correctly guessed every letter in the word.");
            println!("You had {incorrect_guess_count} incorrect guesses.");

            break;
        }

        continue;
    }
}
