package main

func main() {
	// fmt.Println(numRookCaptures())
}

func minMovesToCaptureTheQueen(a int, b int, c int, d int, e int, f int) int {
	// 车后同列 且 象另列 或 象不在车后中间
	if a == e && (c != e || (b-d)*(f-d) > 0) {
		return 1
	}
	// 车后同行 且 象另行 或 象不在车后中间
	if b == f && (d != f || (a-c)*(e-c) > 0) {
		return 1
	}
	// 车后同正斜线 且 象另正斜线 或 车不在象后中间
	if c+d == e+f && (c+d != a+b || (e-a)*(c-a) > 0) {
		return 1
	}
	// 车后同反斜线 且 象另反斜线 或 车不在象后中间
	if c-d == e-f && (c-d != a-b || (e-a)*(c-a) > 0) {
		return 1
	}
	return 2
}
