use rand::prelude::IndexedRandom;
use std::fmt::Debug;
use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    White,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Purple => write!(f, "Purple"),
            Color::White => write!(f, "White"),
        }
    }
}

type ColorSequence = Vec<Color>;

struct Board {
    secret: ColorSequence,
    max_guesses: u8,
}

impl Board {
    fn generate_secret() -> ColorSequence {
        let mut secret = Vec::new();
        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Purple,
            Color::White,
        ];
        for _ in 0..4 {
            secret.push(*colors.choose(&mut rand::rng()).unwrap());
        }
        secret
    }

    fn validate_guess(&self, guess: ColorSequence) -> Feedback {
        let mut match_color = 0;
        let mut match_color_and_position = 0;

        for (i, color) in guess.iter().enumerate() {
            if *color == self.secret[i] {
                match_color_and_position += 1;
            } else if self.secret.contains(color) {
                match_color += 1;
            }
        }

        Feedback {
            match_color,
            match_color_and_position,
        }
    }
}
struct Feedback {
    match_color: u8,
    match_color_and_position: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = Board {
        secret: Board::generate_secret(),
        max_guesses: 10,
    };
    println!("The secret code is: {:?}", b.secret);
    println!("The maximum number of guesses is: {}", b.max_guesses);
    println!(
        "The colors you may guess are: {:?}",
        [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Purple,
            Color::White
        ]
    );
    let mut attempts = 0;
    while attempts < b.max_guesses {
        println!("Attempt {} of {}", attempts + 1, b.max_guesses);
        println!("Enter a guess: ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;

        let guess_result =
            buffer
                .trim_end()
                .replace(",", "")
                .split(" ")
                .try_fold(Vec::new(), |mut acc, s| {
                    match s {
                        "Red" => acc.push(Color::Red),
                        "Green" => acc.push(Color::Green),
                        "Blue" => acc.push(Color::Blue),
                        "Yellow" => acc.push(Color::Yellow),
                        "Purple" => acc.push(Color::Purple),
                        "White" => acc.push(Color::White),
                        _ => return Err(format!("Invalid color: {}", s)),
                    }
                    Ok(acc)
                });

        let guess = match guess_result {
            Ok(colors) => {
                if colors.len() != 4 {
                    println!("Please enter exactly 4 colors");
                    continue;
                }
                colors
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("You guessed: {:?}", guess);

        let feedback = b.validate_guess(guess);

        if feedback.match_color_and_position == 4 {
            println!("Congratulations! You guessed the secret code!");
            return Ok(());
        }

        println!(
            "Feedback: {} color matches, {} color and position matches",
            feedback.match_color, feedback.match_color_and_position
        );

        attempts += 1;
    }
    println!("Computer wins");
    Ok(())
}
