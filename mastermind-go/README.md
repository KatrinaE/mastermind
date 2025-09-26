# Mastermind (Go Implementation)

A command-line implementation of the classic Mastermind game written in Go.

## How to Run

From this directory:
```bash
go run main.go
```

## How to Play

1. The computer generates a secret code of 4 unique colors
2. You have 10 attempts to guess the code
3. Enter your guess as color initials separated by spaces: `Y B R P G W`
4. After each guess, you'll receive feedback:
   - **match**: Right color, wrong position
   - **matchPosition**: Right color in the right position
5. Win by getting 4 matchPosition!

## Available Colors

- Y: Yellow
- B: Blue
- R: Red
- P: Purple
- G: Green
- W: White

## Example

```
Please input the initials for your guesses: Y: yellow, B: blue, R: red, P: purple, G: green, W: white. You need to pick 4 colours.
Y B R P
Your 1 guess was: [Y B R P]
match: 1, matchPosition: 2
```