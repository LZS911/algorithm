package main

import (
	"fmt"
)

func main() {
	fmt.Println(numberOfPoints([][]int{[]int{1, 6}, []int{2, 7}, []int{4, 9}}))
	fmt.Println(numberOfPoints([][]int{[]int{1, 2}, []int{5, 7}, []int{4, 9}}))
}

func numberOfPoints(nums [][]int) int {
	var max int
	var ans int

	for _, arr := range nums {
		if arr[1] > max {
			max = arr[1]
		}
	}

	counts := make([]int, max+1)

	for _, arr := range nums {
		for i := arr[0]; i <= arr[1]; i++ {
			counts[i]++
		}
	}

	for _, v := range counts {
		if v != 0 {
			ans++
		}
	}

	return ans
}
