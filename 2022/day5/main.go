package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CrateStack struct {
	Stacks []string
}

func NewCrateStack() *CrateStack {
	return &CrateStack{
		Stacks: make([]string, 0),
	}
}

func (s *CrateStack) AddToStack(element string) {
	s.Stacks = append(s.Stacks, element)
}
func (s *CrateStack) AddMultipleToStack(elements []string) {
	s.Stacks = append(s.Stacks, elements...)
}

func (s *CrateStack) GetLastCrate() string {
	return s.Stacks[len(s.Stacks)-1]
}

func (s *CrateStack) MoveToStack(destinationStack *CrateStack, ammount int) {
	for i := 0; i < ammount; i++ {
		element := s.Stacks[len(s.Stacks)-1]
		destinationStack.AddToStack(element)
		s.Stacks = s.Stacks[:len(s.Stacks)-1]
	}
}

func (s *CrateStack) MoveInOrderToStack(destinationStack *CrateStack, ammount int) {
	length := len(s.Stacks)
	elements := s.Stacks[length-ammount : length]
	destinationStack.AddMultipleToStack(elements)
	s.Stacks = s.Stacks[:length-ammount]
}

func getStacks(header []string) map[int]*CrateStack {
	stacks := map[int]*CrateStack{}

	for i := len(header) - 3; i >= 0; i-- {
		line := header[i]
		for i := 0; i < len(line); i += 4 {
			currentChar := line[i+1 : i+2]
			if currentChar == " " {
				continue
			}
			if stacks[i/4] == nil {
				stacks[i/4] = NewCrateStack()
			}
			stacks[i/4].AddToStack(currentChar)
		}

	}
	return stacks
}


func run(scanner *bufio.Scanner, isPartOne bool) string {
	readingActions := false
	header := []string{}
	var stacks map[int]*CrateStack
	for scanner.Scan() {
		line := scanner.Text()
		if !readingActions {
			header = append(header, line)
		} else {
			splitted := strings.Split(line, " ")
			ammount, _ := strconv.Atoi(splitted[1])
			from, _ := strconv.Atoi(splitted[3])
			to, _ := strconv.Atoi(splitted[5])
			if isPartOne {
                stacks[from-1].MoveToStack(stacks[to-1], ammount)
			} else {
				stacks[from-1].MoveInOrderToStack(stacks[to-1], ammount)
			}
		}
		if strings.TrimSpace(line) == "" {
			stacks = getStacks(header)
			readingActions = true
		}
	}
	result := []string{}
	for i := 0; i < len(stacks); i++ {
		result = append(result, stacks[i].GetLastCrate())
	}
	return strings.Join(result, "")

}

func part1(scanner *bufio.Scanner) string {
    return run(scanner, true)
}

func part2(scanner *bufio.Scanner) string {
    return run(scanner, false)
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	fmt.Println(part1(scanner))
	file, err = os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner = bufio.NewScanner(file)
	fmt.Println(part2(scanner))
}
