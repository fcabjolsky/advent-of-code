package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day6(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
    scanner.Scan()
	result := part1(scanner.Text())
	assert.Equal(t, 1816, result)
}

func TestPart2Day6(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
    scanner.Scan()
	result := part2(scanner.Text())
	assert.Equal(t, 2625, result)
}
