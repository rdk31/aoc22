package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"io"
	"log"
)


func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		line := fileScanner.Text()
		lines = append(lines, line)
	}

	part1 := 0

	for _, line := range lines {
		middle := len(line) / 2

		first := line[:middle]
		second := line[middle:]

		m_contains := map[rune]bool{}
		for _, ch := range first {
			if strings.Contains(second, string(ch)) {
				m_contains[ch] = true
			}
		}

		for k, _ := range m_contains {
			if int(k) > int('Z') {
				part1 += int(k) - int('a') + 1
			} else {
				part1 += int(k) - int('A') + 27
			}
		}
	}

	fmt.Println(part1)

	part2 := 0

	_, err = file.Seek(0, io.SeekStart)
	if err != nil {
		log.Fatal(err)
	}

	for i := 0; i < len(lines) / 3; i++ {
		line1 := lines[i * 3]
		line2 := lines[i * 3 + 1]
		line3 := lines[i * 3 + 2]

		m_contains := map[rune]bool{}
		for _, ch := range line1 {
			if strings.Contains(line2, string(ch)) {
				m_contains[ch] = true
			}
		}

		for k, _ := range m_contains {
			if !strings.Contains(line3, string(k)) {
				m_contains[k] = false
			}
		}

		for k, v := range m_contains {
			if v {
				if int(k) > int('Z') {
					part2 += int(k) - int('a') + 1
				} else {
					part2 += int(k) - int('A') + 27
				}
			}
		}
	}

	fmt.Println(part2)
}
