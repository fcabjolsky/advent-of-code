package main

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Day7(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
    nodes := processInput(scanner)
    result := part1(nodes)
    assert.Equal(t, 1845346, result)
}

func TestPart2Day7(t *testing.T) {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
    nodes := processInput(scanner)
    result := part2(nodes)
    assert.Equal(t, 3636703, result)

}
