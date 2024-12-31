package main

import "fmt"

func main() {
	fmt.Println(takeCharacters("aabaaaacaabc", 2))
	fmt.Println(takeCharacters("aabaaaaaabaaaacaabcabcabcccbabbaccabacaabc", 4))

}

func takeCharacters(s string, k int) int {
	arr := make([]int, 3)

	for _, v := range s {
		arr[v-'a']++
	}

	if arr[0] < k || arr[1] < k || arr[2] < k {
		return -1
	}

	n := len(s)
	record := make([]int, 3)
	res := 0

	right := 0

	for left := 0; left < n-res; left++ {
		for right < n && record[s[right]-'a'] < (arr[s[right]-'a']-k) {
			record[s[right]-'a']++
			right++
		}

		res = max(res, right-left)
		record[s[left]-'a']--
	}

	return n - res
}

func max(a int, b int) int {
	if a > b {
		return a
	}

	return b
}
