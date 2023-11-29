package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func getMatch(firstComp string, secondComp string) rune {
	for _, letter := range firstComp {
		if strings.ContainsRune(secondComp, letter) {
			return letter
		}
	}
	return ' '
}

func getPointsForElement(element rune) int {
	if int(element) > 96 {
		return int(element) - 96
	}
	return (int(element) - 64) + 26
}
func part1(scanner *bufio.Scanner) int {
	score := 0
	for scanner.Scan() {
		line := scanner.Text()
		halfLineLen := len(line) / 2
		firstHalf := line[:halfLineLen]
		secondHalf := line[halfLineLen:]
		element := getMatch(firstHalf, secondHalf)
		score += getPointsForElement(element)
	}
    return score
}

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
    part1 := part1(scanner)

	file, err = os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()

	scanner = bufio.NewScanner(file)
    part2 := Part2(scanner)
    fmt.Println("part 1", part1)
    fmt.Println("part 2", part2)
}

