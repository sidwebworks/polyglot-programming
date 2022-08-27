package submarine

import (
	"log"
	"strconv"
	"strings"
)

func getInput() string {
	return `
		forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
	`
}

func Run() {
	lines := strings.Split(getInput(), "\n")
	result := point{}

	for _, line := range lines {
		amount := parseLine(line)
		result.x += amount.x
		result.y += amount.y
	}

	log.Println(result, result.x*result.y)
}

type point struct {
	x int
	y int
}

func parseLine(line string) point {
	parts := strings.Split(strings.Trim(line, " "), " ")

	if len(parts) < 2 {
		return point{x: 0, y: 0}
	}

	amount, err := strconv.Atoi(parts[1])

	if err != nil {
		log.Fatal("Shitttttt happend")
	}

	if parts[0] == "forward" {
		return point{x: amount, y: 0}
	} else if parts[0] == "up" {
		return point{x: 0, y: -amount}
	}

	return point{x: 0, y: amount}
}
