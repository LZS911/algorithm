package main

import "fmt"

func main() {
	fmt.Println(distributeCandies(5, 2))
	fmt.Println(distributeCandies(3, 3))
}

func distributeCandies(n int, limit int) int {
	var ans = 0

	for i := 0; i <= limit; i++ {
		for j := 0; j <= limit; j++ {
			if i+j > n {
				break
			}

			if n-i-j <= limit {
				ans++
			}
		}
	}

	return ans
}
