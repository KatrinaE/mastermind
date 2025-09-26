package main

import (
	"bufio"
	"fmt"
	"io"
	"math/rand"
	"os"
	"strings"
)

var colours = [6]string{"yellow", "blue", "red", "purple", "green", "white"}
var colourMap = map[string]string{
	"Y": "yellow",
	"B": "blue",
	"R": "red",
	"P": "purple",
	"G": "green",
	"W": "white",
}

func getSecretColours() [4]string {
	var secret [4]string

	for {
		a := rand.Int63n(6)
		b := rand.Int63n(6)
		c := rand.Int63n(6)
		d := rand.Int63n(6)

		secret = [4]string{colours[a], colours[b], colours[c], colours[d]}

		ok := true
		for i := range secret {
			for j := range secret {
				if i == j {
					continue
				}

				if secret[i] == secret[j] {
					ok = false
				}
			}
		}

		if ok {
			break
		}
	}

	return secret
}

func readLine(reader *bufio.Reader) string {
	str, _, err := reader.ReadLine()
	if err == io.EOF {
		return ""
	}

	return strings.TrimRight(string(str), "\r\n")
}

func validateGuess(guess []string) error {
	if len(guess) > 4 || len(guess) < 4 {
		return fmt.Errorf("you should choose exactly 4 colours")
	}

	seen := make(map[string]int, 4)

	for _, g := range guess {
		seen[strings.ToUpper(g)]++
		if seen[strings.ToUpper(g)] > 1 {
			return fmt.Errorf("duplicate colour choices %s", guess)
		}
		_, ok := colourMap[strings.ToUpper(g)]
		if !ok {
			return fmt.Errorf("invalid choice %s", g)
		}
	}
	return nil
}

func checkGuess(secret [4]string, guess []string) (int, int) {
	var match, matchPosition int
	for i := range secret {
		for j := range guess {
			if secret[i] == colourMap[strings.ToUpper(guess[j])] {
				if i == j {
					matchPosition++
				} else {
					match++
				}
			}
		}
	}

	return match, matchPosition
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	secret := getSecretColours()

	for i := 0; i < 10; {
		fmt.Printf("Please input the initials for your guesses: Y: yellow, B: blue, R: red, P: purple, G: green, W: white. You need to pick 4 colours. \n")
		input := readLine(reader)

		guess := strings.Split(input, " ")

		err := validateGuess(guess)
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Printf("Your %d guess was: %v \n", i+1, guess)

		match, matchPosition := checkGuess(secret, guess)
		fmt.Printf("match: %d, matchPosition: %d\n", match, matchPosition)

		if matchPosition == 4 {
			fmt.Println("Player wins.")
			break
		}

		if i == 9 {
			fmt.Println("Computer wins")
		}
		i++
	}
}
