package main

import (
	"fmt"
	"sort"
)

func main() {
	fmt.Println(maximumBeauty([]int{4, 6, 1, 2}, 2))
	fmt.Println(maximumBeauty([]int{1, 1, 1, 1}, 10))
}

func maximumBeauty(nums []int, k int) int {
	res, n := 0, len(nums)
	sort.Ints(nums)
	for i, j := 0, 0; i < n; i++ {
		for nums[i]-2*k > nums[j] {
			j++
		}
		res = max(res, i-j+1)
	}
	return res
}
