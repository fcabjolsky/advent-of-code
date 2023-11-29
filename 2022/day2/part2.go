package main

import (
	"bufio"
	"strings"
)

type HowShouldEnd int

const (
	WINNING HowShouldEnd = iota
	LOSSING
	DRWAING
)

func getHowShouldEnd(op string) HowShouldEnd {
	opMap := map[string]HowShouldEnd{
		"X": LOSSING,
		"Y": DRWAING,
		"Z": WINNING,
	}
	return opMap[op]
}

func getNeededSelection(oponent GameOptions, howShouldEnd HowShouldEnd) GameOptions {
	switch oponent {
	case Rock:
		{
			switch howShouldEnd {
			case WINNING:
				return Paper
			case LOSSING:
				return Scissors
			}
		}
	case Scissors:
		{
			switch howShouldEnd {
			case WINNING:
				return Rock
			case LOSSING:
				return Paper
			}
		}
	case Paper:
		{
			switch howShouldEnd {
			case WINNING:
				return Scissors
			case LOSSING:
				return Rock
			}
		}
	}
	// drwaing is the same selection
	return oponent
}

func Day2_2(scanner *bufio.Scanner) int {
	score := 0
	for scanner.Scan() {
		text := scanner.Text()
		splitted := strings.Split(text, " ")
		oponent := getOponentOption(splitted[0])
		howShouldEnd := getHowShouldEnd(splitted[1])
		mine := getNeededSelection(oponent, howShouldEnd)
		status := getStatus(oponent, mine)
		score += int(status) + int(mine)

	}
	return score
}
