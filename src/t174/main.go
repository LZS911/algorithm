package main

import "fmt"

func main() {
	a1 := []int{1, 1, 2, 2, 3, 3}
	fmt.Println(distributeCandies(a1))
}

func distributeCandies(candyType []int) int {
	set := map[int]struct{}{}
	for _, t := range candyType {
		set[t] = struct{}{}
	}
	ans := len(candyType) / 2
	if len(set) < ans {
		ans = len(set)
	}
	return ans
}
