package main

import (
	"bufio"
	"strings"
)

func getCommonElementForGroup(elvesRucksacks []string) rune {
	for _, element := range elvesRucksacks[0] {
		found := true
		for _, e := range elvesRucksacks[1:] {
			if !strings.ContainsRune(e, element) {
				found = false
				break
			}
		}
		if found {
			return element
		}
	}
	return ' '
}

func Part2(scanner *bufio.Scanner) int {
	elvesGroups := []string{}
	i := 0
	score := 0
    nGroup := 3
	for scanner.Scan() {
		line := scanner.Text()
		elvesGroups = append(elvesGroups, line)
		i++
		if i == nGroup {
			element := getCommonElementForGroup(elvesGroups)
			score += getPointsForElement(element)
			elvesGroups = []string{}
			i = 0
		}

	}
    return score
}
