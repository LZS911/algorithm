package main

import "fmt"

func main() {
	fmt.Println(maxScoreSightseeingPair([]int{8, 1, 5, 2, 6}))
}
func maxScoreSightseeingPair(values []int) int {
	n := len(values)
	if n == 2 {
		return values[0] + values[1] - 1
	}

	ans, mx := 0, values[0]+0
	for j := 1; j < n; j++ {
		ans = max(ans, mx+values[j]-j)
		mx = max(mx, values[j]+j)
	}
	return ans

}

func max(a int, b int) int {
	if a > b {
		return a
	}

	return b
}
