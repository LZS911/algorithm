package main

import (
	"container/heap"
	"fmt"
)

func main() {
	fmt.Println(eatenApples([]int{1, 2, 3, 5, 2}, []int{3, 2, 1, 4, 2}))
	fmt.Println(eatenApples([]int{3, 0, 0, 0, 0, 2}, []int{3, 0, 0, 0, 0, 2}))
}

func eatenApples(apples, days []int) (ans int) {
	h := hp{}
	for i := 0; i < len(apples) || h.Len() > 0; i++ {
		for len(h) > 0 && h[0].rottenDay == i { // 已腐烂
			heap.Pop(&h)
		}
		if i < len(apples) && apples[i] > 0 {
			heap.Push(&h, pair{i + days[i], apples[i]})
		}
		if len(h) > 0 {
			// 吃一个最早腐烂的苹果
			ans++
			h[0].num--
			if h[0].num == 0 {
				heap.Pop(&h)
			}
		}
	}
	return
}

type pair struct{ rottenDay, num int }
type hp []pair

func (h hp) Len() int           { return len(h) }
func (h hp) Less(i, j int) bool { return h[i].rottenDay < h[j].rottenDay }
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v any)        { *h = append(*h, v.(pair)) }
func (h *hp) Pop() any          { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
