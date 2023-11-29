package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day2(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	assert.Equal(t, 15632, day2(scanner))

}

func TestPart2Day2(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	assert.Equal(t, 14416, Day2_2(scanner))

}
