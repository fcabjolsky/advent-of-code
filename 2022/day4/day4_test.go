package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day4(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	full, _ := run(scanner)
	assert.Equal(t, 580, full)

}

func TestPart2Day4(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	_, partial := run(scanner)
	assert.Equal(t, 895, partial)
}
