package main

func main() {
	// fmt.Println(numRookCaptures())
}

func numRookCaptures(board [][]byte) int {

	cnt := 0
	st, ed := -1, -1

	dx := []int{1, 0, -1, 0}
	dy := []int{0, 1, 0, -1}

	for i, item := range board {

		for j, value := range item {
			if value == 'R' {
				st, ed = i, j
				break
			}

			if st != -1 && ed != -1 {
				break
			}
		}
	}

	for i := 0; i < 4; i++ {
		for step := 1; ; step++ {
			tx := st + step*dx[i]
			ty := ed + step*dy[i]
			if tx < 0 || tx >= 8 || ty < 0 || ty >= 8 || board[tx][ty] == 'B' {
				break
			}

			if board[tx][ty] == 'p' {
				cnt++
				break
			}

		}
	}

	return cnt
}
