package main

import "fmt"

func main() {
	fmt.Println(distributeCandies(7, 4))
	fmt.Println(distributeCandies(10, 3))
}

func distributeCandies(candies int, num_people int) []int {
	ans := make([]int, num_people)
	i := 0
	for candies != 0 {
		ans[i%num_people] += min(candies, i+1)
		candies -= min(candies, i+1)
		i++
	}
	return ans
}
