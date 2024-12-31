package main

import "fmt"

func main() {
	fmt.Println(numberOfPairs([]int{1, 3, 4}, []int{1, 3, 4}, 1))
	fmt.Println(numberOfPairs([]int{1, 2, 4, 12}, []int{2, 4}, 3))

}

func numberOfPairs(nums1 []int, nums2 []int, k int) (ans int64) {

	cnt := make(map[int]int)

	for _, x := range nums1 {
		if x%k > 0 {
			continue
		}
		x /= k
		for d := 1; d*d <= x; d++ { // 枚举因子
			if x%d == 0 {
				cnt[d]++ // 统计因子
				if d*d < x {
					cnt[x/d]++ // 因子总是成对出现
				}
			}
		}
	}

	for _, x := range nums2 {
		ans += int64(cnt[x])
	}

	// for i := 0; i < len(nums1); i++ {
	// 	for j := 0; j < len(nums2); j++ {
	// 		if nums1[i]%(nums2[j]*k) == 0 {
	// 			ans++
	// 		}
	// 	}
	// }

	return
}
