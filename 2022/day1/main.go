package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func part1() int {
	file, err := os.Open("input.txt")

	if err != nil {
		file = os.Stdin
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
    elvesCalories := []int{0}
    i := 0
    for scanner.Scan() {
        text := scanner.Text()
        if strings.TrimSpace(text) == "" {
            elvesCalories = append(elvesCalories, 0)
            i++
            continue
        }
        calories, err := strconv.Atoi(text)
        if err != nil {
            fmt.Println("error converting to int")
        }

        elvesCalories[i] += calories
    }
    sort.Ints(elvesCalories)
    
    topn := 3
    sum := 0
    for _, value := range elvesCalories[len(elvesCalories) - topn:] {
        sum += value
    }
    return sum
}

func main () {
    fmt.Println("max::", part1())
}
