package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	inputMap := processInput(scanner)
	part1, part2 := part1And2(inputMap)
	fmt.Println(part1, part2)

}

func reverseCheck(input *[][]int, start, curr, extra int, check func(*[][]int, int, int) int) (bool, int) {
    tot := 0
	for i := start - 1; i >= 0; i-- {
		tot++
		if check(input, i, extra) >= curr {
			return false, tot
		}

	}
	return true, tot
}

func normalCheck(input *[][]int, start, curr, extra int, check func(*[][]int, int, int) int) (bool, int) {
    tot := 0
	for i := start + 1; i < len(*input); i++ {
		tot++
		if check(input, i, extra) >= curr {
			return false, tot
		}

	}
	return true, tot
}



func processInput(scanner *bufio.Scanner) [][]int {
	inputMap := [][]int{}
	for scanner.Scan() {
		line := scanner.Text()
		currentRow := []int{}
		for _, v := range line {
			tree, _ := strconv.Atoi(string(v))
			currentRow = append(currentRow, tree)
		}
		inputMap = append(inputMap, currentRow)
	}
	return inputMap
}


func part1And2(inputMap [][]int) (int, int) {
	total := ((len(inputMap) * 2) + (len(inputMap[0]) * 2)) - 4
	max := 0
    checkInRow := func (input *[][]int, row int, col int) int { return (*input)[col][row]}
    checkInCol := func (input *[][]int, col int, row int) int { return (*input)[col][row]}

	for i := 1; i < len(inputMap)-1; i++ {
		for j := 1; j < len(inputMap[i])-1; j++ {
            
            visibleT, currT := reverseCheck(&inputMap, i, inputMap[i][j], j, checkInCol) 
            visibleB, currB := normalCheck(&inputMap, i, inputMap[i][j], j, checkInCol) 
			visibleR, currR := normalCheck(&inputMap, j, inputMap[i][j], i, checkInRow)
			visibleL, currL := reverseCheck(&inputMap, j, inputMap[i][j], i, checkInRow) 
			if visibleT || visibleB || visibleL || visibleR {
				tot := currB * currT * currR * currL
				if tot > max {
					max = tot
				}
				total++
			}

		}

	}
	return total, max
}
