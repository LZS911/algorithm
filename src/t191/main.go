package main

import "fmt"

func main() {
	fmt.Println(timeRequiredToBuy([]int{2, 3, 2}, 2))
	fmt.Println(timeRequiredToBuy([]int{100}, 0))
	fmt.Println(timeRequiredToBuy([]int{5, 1, 1, 1}, 0))
	fmt.Println(timeRequiredToBuy([]int{84, 49, 5, 24, 70, 77, 87, 8}, 3))

}

// func timeRequiredToBuy(tickets []int, k int) int {
// 	ans := 0

// 	if len(tickets) == 1 {
// 		return tickets[0]
// 	}

// 	for k != 0 || tickets[0] != 1 {
// 		if tickets[0] == 1 {
// 			tickets = tickets[1:]
// 		} else {
// 			st := tickets[0]
// 			for i := 0; i < len(tickets); i++ {
// 				if i == len(tickets)-1 {
// 					tickets[i] = st - 1
// 				} else {
// 					tickets[i] = tickets[i+1]
// 				}
// 			}
// 		}

// 		if k == 0 {
// 			k = len(tickets) - 1
// 		} else {
// 			k = k - 1
// 		}

// 		ans++
// 	}

// 	return ans + 1

// }

func timeRequiredToBuy(tickets []int, k int) int {
	ans := 0

	n := len(tickets)

	for i := 0; i < n; i++ {
		if i <= k {
			ans += min(tickets[i], tickets[k])
		} else {
			ans += min(tickets[i], tickets[k]-1)
		}
	}

	return ans
}

func min(a int, b int) int {
	if a > b {
		return b
	}

	return a
}
