package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func _isOverlaped(lower1 int, upper1 int, lower2 int, upper2 int) bool {
	for i := lower1; i <= upper1; i++ {
		if i < lower2 || i > upper2 {
			return false
		}
	}
	return true
}

func _isOverlaped2(lower1 int, upper1 int, lower2 int, upper2 int) bool {
	for i := lower1; i <= upper1; i++ {
		if i >= lower2 && i <= upper2 {
			return true
		}
	}
	return false
}

func isOverlaped(range1 string, range2 string, check func(int, int, int, int) bool) bool {
	range1Splitted := strings.Split(range1, "-")
	range2Splitted := strings.Split(range2, "-")
	lower1, _ := strconv.Atoi(range1Splitted[0])
	upper1, _ := strconv.Atoi(range1Splitted[1])

	lower2, _ := strconv.Atoi(range2Splitted[0])
	upper2, _ := strconv.Atoi(range2Splitted[1])

	return check(lower1, upper1, lower2, upper2) || check(lower2, upper2, lower1, upper1)
}

func part1(firstRange string, secondRange string) bool {
	return isOverlaped(firstRange, secondRange, _isOverlaped)
}
func part2(firstRange string, secondRange string) bool {
	return isOverlaped(firstRange, secondRange, _isOverlaped2)
}

func run(scanner *bufio.Scanner) (int, int) {
	totalFullOverlaped := 0
	totalOverlaped := 0
	for scanner.Scan() {
		line := scanner.Text()
		splitted := strings.Split(line, ",")
		firstRange := splitted[0]
		secondRange := splitted[1]
		if part1(firstRange, secondRange) {
			totalFullOverlaped++
		}
		if part2(firstRange, secondRange) {
			totalOverlaped++
		}
	}
    return totalFullOverlaped, totalOverlaped
}

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
    totalFullOverlaped, totalOverlaped := run(scanner)

	fmt.Println("full: ", totalFullOverlaped)
	fmt.Println("partial: ", totalOverlaped)

}
