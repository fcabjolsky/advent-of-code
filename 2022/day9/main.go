package main

import (
	"bufio"
	"fmt"
	"math"
	"os"

	// "os"
	"strconv"
	"strings"
)

type Knot struct {
	X         int
	Y         int
	tail      *Knot
	head      *Knot
    endTail *Knot
	visited   map[string]bool
}

func (p *Knot) moveTowardHead() {
	offsetX := p.head.X - p.X
	offsetY := p.head.Y - p.Y

	distanceX := int(math.Abs(float64(offsetX)))
	distanceY := int(math.Abs(float64(offsetY)))

	if distanceX > 1 || (distanceX > 0 && distanceY > 1) {
		p.X += 1 * (offsetX / distanceX)
	}

	if distanceY > 1 || (distanceY > 0 && distanceX > 1) {
		p.Y += 1 * (offsetY / distanceY)
	}
	p.visited[p.Direction()] = true
}

func NewKnot(tails int) *Knot {
	head := &Knot{
		X:       0,
		Y:       0,
		visited: map[string]bool{"00": true},
	}
	currHead := head
	for i := 0; i < tails; i++ {
		newPoint := &Knot{
			X:       0,
			Y:       0,
			visited: map[string]bool{"00": true},
			head:    currHead,
		}

		currHead.tail = newPoint
		currHead = newPoint
	}
    head.endTail = currHead
	return head
}

func (p *Knot) move(direction string) {
	switch direction {
	case "U":
		p.X--
	case "D":
		p.X++
	case "R":
		p.Y++
	case "L":
		p.Y--
	}
	p.visited[p.Direction()] = true
}

func (p *Knot) MoveHead(direction string) {
	p.move(direction)
	p.tail.MoveTail()
}

func (p *Knot) MoveTail() {
	if p == nil {
		return
	}
	p.moveTowardHead()
	if p.tail != nil {
		p.tail.MoveTail()
	}
}

func (p *Knot) GetAmmountVissited() int {
    return len(p.visited)
}

func (p *Knot) Direction() string {
	return fmt.Sprintf("%d%d", p.X, p.Y)
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	part1, part2  := run(scanner)
    fmt.Println("Part 1: ", part1)
    fmt.Println("Part 2: ", part2)

}

func run(scanner *bufio.Scanner) (int, int) {
	head := NewKnot(9)
	for scanner.Scan() {
		line := scanner.Text()
		splitted := strings.Split(line, " ")
		direction := splitted[0]
		ammount, _ := strconv.Atoi(splitted[1])
		for i := 0; i < ammount; i++ {
			head.MoveHead(direction)
		}
	}
	return head.tail.GetAmmountVissited(), head.endTail.GetAmmountVissited()
}
