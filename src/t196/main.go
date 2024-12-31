package main

import (
	"fmt"
	"sort"
)

func main() {
	fmt.Println(minimumAverage([]int{7, 8, 3, 4, 15, 13, 4, 1}))
}

func minimumAverage(nums []int) float64 {
	n := len(nums)
	sort.Ints(nums)
	var res float64 = float64(nums[0]+nums[n-1]) / 2.0

	for i := 1; i < n/2; i++ {
		min := nums[i]
		max := nums[n-1-i]

		res = getMin(float64(min+max)/2.0, res)
	}

	return res
}

func getMin(a float64, b float64) float64 {
	if a < b {
		return a
	}
	return b
}
