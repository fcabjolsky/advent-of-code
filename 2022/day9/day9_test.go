package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day9(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	result, _ := run(scanner)
	assert.Equal(t, 6181, result)
}

func TestPart2Day9(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	_, result := run(scanner)
	assert.Equal(t, 2386, result)
}
