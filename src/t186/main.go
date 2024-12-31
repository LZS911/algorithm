package main

import "fmt"

func main() {
	fmt.Println(findJudge(2, [][]int{
		{1, 2},
	}))
	fmt.Println(findJudge(3, [][]int{
		{1, 3},
		{2, 3},
	}))
	fmt.Println(findJudge(3, [][]int{
		{1, 3},
		{2, 3},
		{3, 1},
	}))

	fmt.Println(findJudge(4, [][]int{
		{1, 3},
		{1, 4},
		{2, 3},
		{2, 4},
		{4, 3},
	}))
}
func findJudge(n int, trust [][]int) int {
	ans := make([]int, n+1)
	used := make([]int, n+1)

	res := -1

	for _, arr := range trust {
		a, b := arr[0], arr[1]

		ans[b] += 1
		used[a] += 1
	}

	for i := 1; i < len(ans); i++ {
		if ans[i] == n-1 && used[i] == 0 {
			res = i
		}
	}

	return res
}
