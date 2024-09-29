package main

import "slices"

func main() {
}

func maxNumOfMarkedIndices(nums []int) int {
	slices.Sort(nums)
	i := 0
	for _, x := range nums[(len(nums)+1)/2:] {
		if nums[i]*2 <= x {
			i++
		}
	}
	return i * 2
}
