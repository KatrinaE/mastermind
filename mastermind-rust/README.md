# Mastermind (Rust Implementation)

A command-line implementation of the classic Mastermind game written in Rust.

## How to Run

From this directory:
```bash
cargo run
```

## How to Play

1. The computer generates a secret code of 4 unique colors
2. You have 10 attempts to guess the code
3. Enter your guess as color names separated by spaces: `Red Blue Green Yellow`
4. After each guess, you'll receive feedback:
   - **Color matches**: Right color, wrong position
   - **Color and position matches**: Right color in the right position
5. Win by getting 4 color and position matches!

## Available Colors

- Red
- Green
- Blue
- Yellow
- Purple
- White

## Example

```
Enter a guess:
Red Blue Green Yellow
You guessed: [Red, Blue, Green, Yellow]
Feedback: 1 color matches, 2 color and position matches
```