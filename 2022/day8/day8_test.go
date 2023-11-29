package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day8(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	inputMap := processInput(scanner)
	result, _ := part1And2(inputMap)
    assert.Equal(t, 1801, result)
}

func TestPart2Day8(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	inputMap := processInput(scanner)
	_, result := part1And2(inputMap)
    assert.Equal(t, 209880, result)
}
