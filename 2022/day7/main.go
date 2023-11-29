package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		file = os.Stdin
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
    nodes := processInput(scanner)
	part1 := part1(nodes)
	part2 := part2(nodes)
	fmt.Println(part1)
	fmt.Println(part2)

}

type Node struct {
	Parent *Node
	Path   string
	Size   int
}

func NewNode(path string) *Node {
	return &Node{
		Path:   path,
		Size:   0,
	}
}

func (n *Node) setParent(parent *Node) {
	n.Parent = parent
}

func (n *Node) addSize(size int) {
	n.Size += size
}

func processCommand(currentNode *Node, command string, nodes *[]*Node) *Node {
	if strings.Contains(command, "cd") {
		dir := strings.Split(command, " ")[2]
		if !strings.Contains(dir, "..") {
			if dir != "/" {
				dir = currentNode.Path + dir
			}
			newNode := NewNode(dir)
			newNode.setParent(currentNode)
			currentNode = newNode
			*nodes = append(*nodes, currentNode)
		} else {
			currentNode.Parent.addSize(currentNode.Size)
			currentNode = currentNode.Parent
		}
	}
	return currentNode
}

func processInput(scanner *bufio.Scanner) []*Node {
	nodes := []*Node{}
	var currentNode *Node
	for scanner.Scan() {
		line := scanner.Text()
		if strings.ContainsRune(line, '$') {
			currentNode = processCommand(currentNode, line, &nodes)
		} else {
			if !strings.Contains(line, "dir") {
				size, _ := strconv.Atoi(strings.Split(line, " ")[0])
				currentNode.addSize(size)
			}
		}

	}

	for currentNode.Path != "/" {
		currentNode.Parent.addSize(currentNode.Size)
		currentNode = currentNode.Parent
	}
    return nodes;
}

func part1(nodes []*Node) int {
	var sum int = 0
	for _, v := range nodes {
		if v.Size <= 100000 {
			sum += v.Size
		}
	}
	return sum
}

func part2(nodes []*Node) int {
    available := 70000000
    needed := 30000000
    total := nodes[0].Size
    toFree := needed - (available - total)
    min := available
    for _, v := range nodes {
        if v.Size > toFree && v.Size < min {
            min = v.Size
        }
    }
    return min 
}

