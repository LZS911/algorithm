package main

import (
	"fmt"
)

func main() {
	fmt.Println(findLUSlength("aba", "cdc"))
}

func findLUSlength(a, b string) int {
	if a != b {
		return max(len(a), len(b))
	}
	return -1
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}
