package main

import (
	"fmt"
)

func main() {
	fmt.Println(differenceOfSum([]int{1, 15, 6, 3}))
	fmt.Println(differenceOfSum([]int{1, 2, 3, 4}))
}

func differenceOfSum(nums []int) int {
	sum1, sum2 := 0, 0

	for _, v := range nums {
		sum1 += v
		for v > 0 {
			sum2 += v % 10
			v /= 10
		}
	}

	return sum1 - sum2
}
