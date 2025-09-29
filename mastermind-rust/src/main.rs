use rand::prelude::IndexedRandom;
use std::str::FromStr;
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

impl Color {
    const ALL: &[Color] = &[
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Purple,
        Color::White,
    ];
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

impl FromStr for Color {
    type Err = ColorSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Color::Red),
            "Green" => Ok(Color::Green),
            "Blue" => Ok(Color::Blue),
            "Yellow" => Ok(Color::Yellow),
            "Purple" => Ok(Color::Purple),
            "White" => Ok(Color::White),
            _ => Err(ColorSequenceError {
                message: format!("Invalid color: {}", s)
            }),
        }
    }
}

#[derive(Debug)]
struct ColorSequence {
    colors: Vec<Color>
}

impl FromIterator<Color> for ColorSequence {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        ColorSequence {
            colors: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ColorSequenceError {
    message: String
}

impl std::fmt::Display for ColorSequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl FromStr for ColorSequence {
    type Err = ColorSequenceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors: Result<Vec<Color>, ColorSequenceError> = s
            .split(" ")
            .map(|x| x.parse())
            .collect();

        let colors = colors?;
        Ok(ColorSequence { colors })
    }

}


struct Board {
    secret: ColorSequence,
    max_guesses: u8,
}

impl Board {
    fn generate_secret() -> ColorSequence {
        std::iter::repeat_with(
	    || *Color::ALL.choose(&mut rand::rng()).unwrap())
            .take(4)
            .collect()
    }
    
    fn validate_guess(&self, guess: ColorSequence) -> Feedback {
        let mut match_color = 0;
        let mut match_color_and_position = 0;

        for (i, color) in guess.colors.iter().enumerate() {
            if *color == self.secret.colors[i] {
                match_color_and_position += 1;
            } else if self.secret.colors.contains(color) {
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

impl std::fmt::Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	if self.is_win() {
	   return write!(f, "Congratulations! You guessed the secret code!");
	} else {
	    write!(f, "Feedback: {} color matches, {} color and position matches",
		   self.match_color, self.match_color_and_position)
	}

    }
}

impl Feedback {
    fn is_win(&self) -> bool {
	self.match_color_and_position == 4
    }
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
    let mut buffer;
    while attempts < b.max_guesses {
        println!("Attempt {} of {}", attempts + 1, b.max_guesses);
        println!("Enter a guess: ");
        buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;

        let guess_result: Result<Vec<Color>, _> = buffer
            .trim_end()
            .replace(",", "")
            .split(" ")
            .map(|s| s.parse::<Color>())
            .collect();

        let guess: ColorSequence = match guess_result {
            Ok(colors) => {
                if colors.len() != 4 {
                    println!("Please enter exactly 4 colors");
                    continue;
                }
                ColorSequence {
		    colors
		}
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("You guessed: {:?}", guess);

        let feedback = b.validate_guess(guess);
        println!("{}", feedback);
        if feedback.is_win() {
            return Ok(());
        }

	buffer.clear();
        attempts += 1;
    }
    println!("Computer wins");
    Ok(())
}
