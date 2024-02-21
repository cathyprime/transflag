package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/charmbracelet/lipgloss"
)

const ratio = 5.0 / 3.0

var (
	blue   = lipgloss.NewStyle().Background(lipgloss.Color("#5BCEFA")).Foreground(lipgloss.Color("#5BCEFA"))
	pink   = lipgloss.NewStyle().Background(lipgloss.Color("#F5A9B8")).Foreground(lipgloss.Color("#F5A9B8"))
	white  = lipgloss.NewStyle().Background(lipgloss.Color("#FFFFFF")).Foreground(lipgloss.Color("#FFFFFF"))
	height = 5
	width  = 0
)

func main() {
	args := os.Args[1:]
	switch len(args) {
	case 0:
	case 1:
		var err error
		height, err = strconv.Atoi(args[0])
		if err != nil {
			log.Fatal("number is required")
		}
	default:
		var err error
		height, err = strconv.Atoi(args[0])
		if err != nil {
			log.Fatal("number is required")
		}

		width, err = strconv.Atoi(args[1])
		if err != nil {
			log.Fatal("number is required")
		}
	}
	ch := "t4t"
	line := func() string {
		newWidth := func() int {
			if width == 0 {
				return int(float32(height) * ratio) - 1
			} else {
				return width
			}
		}()

		line := strings.Repeat(ch, newWidth)
		for range (height / 5) - 1 {
			line = line + "\n" + line
		}
		return line
	}()
	fin := make([]string, 5)
	for x := range 5 {
		switch x {
		case 0, 4:
			fin[x] = blue.Render(line)
		case 1, 3:
			fin[x] = pink.Render(line)
		case 2:
			fin[x] = white.Render(line)
		default:
			panic("What?")
		}
	}
	fmt.Println(strings.Join(fin, "\n"))
}
