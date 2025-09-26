import random


def mastermind():
    pass


colors = ["red", "blue", "green", "yellow", "purple", "orange"]

LEN_SECRET = 4
MAX_GUESSES = 10


def generate_secret():
    return [random.choice(colors) for _ in range(LEN_SECRET)]


class Feedback:
    def __init__(self):
        self.match_positions = 0
        self.match_colors = 0


def get_guess():
    while True:
        guess = input("Enter your guess (4 colors): ").split()
        if len(guess) != LEN_SECRET:
            print("Invalid input. Please enter exactly 4 colors.")
        elif not all(color in colors for color in guess):
            print("Invalid input. Please enter valid colors.")
        else:
            return guess


def check(guess, secret):
    feedback = Feedback()
    for i in range(0, LEN_SECRET):
        if guess[i] == secret[i]:
            feedback.match_positions += 1
        elif guess[i] in secret:
            feedback.match_colors += 1
    return feedback


def main():
    secret = generate_secret()
    for _ in range(0, MAX_GUESSES):
        guess = get_guess()
        feedback = check(guess, secret)

        if feedback.match_positions == 4:
            print("Congratulations! You won!")
            break
        else:
            print(
                f"{feedback.match_positions} correct positions, {feedback.match_colors} correct colors"
            )

    print("Computer wins.")


if __name__ == "__main__":
    main()
