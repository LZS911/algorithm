package main

import "fmt"

func main() {
	fmt.Println(edgeScore([]int{1, 0, 0, 0, 0, 7, 7, 5}))
	fmt.Println(edgeScore([]int{2, 0, 0, 2}))
	fmt.Println(edgeScore([]int{1, 3, 1, 2}))

}

func edgeScore(edges []int) int {
	ans := make([]int, len(edges))

	// for i := 0; i < len(edges); i++ {
	// 	ans[i] = getSumWithIndex(edges, i)
	// }

	for i, v := range edges {
		ans[v] += i
	}

	return findMaxValueIndex(ans)
}

func getSumWithIndex(arr []int, index int) int {
	sum := 0
	for i, v := range arr {
		if v == index {
			sum += i
		}
	}

	return sum
}

func findMaxValueIndex(arr []int) int {
	if len(arr) == 0 {
		panic("slice len is empty")
	}
	res := 0
	max := arr[0]
	for i, v := range arr {
		if v > max {
			res = i
			max = v
		}
	}

	return res
}
