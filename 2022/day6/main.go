package main

import (
	"bufio"
	"fmt"
	"os"
)

func isUnique(input string) bool {
	if len(input) == 0 {
		return true
	}
	contador := map[rune]int{}
	for _, v := range input {
		_, exist := contador[v]
		if !exist {
			contador[v] = 0
		}
		contador[v]++
		if contador[v] > 1 {
			return false
		}
	}
	return true
}

func run(input string, lengthOfUnique int) int {
	start := 0
	for i := lengthOfUnique; i < len(input); i++ {
		if isUnique(input[start:i]) {
			return i
		}
		start++
	}
	return 0

}

func part1(input string) int {
    return run(input, 4)
}
func part2(input string) int {
    return run(input,14)
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	input := scanner.Text()
	part1 := part1(input)
	part2 := part2(input)

	fmt.Println(input[part1:part1+4], part1)
	fmt.Println(input[part2:part2+14], part2)
}

