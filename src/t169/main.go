package main

import "strings"

func main() {
	convert("PAYPALISHIRING", 3)
	convert("PAYPALISHIRING", 4)
	convert("A", 1)
}
func convert(s string, numRows int) string {
	if numRows == 1 {
		return s
	}
	var b = []rune(s)
	var res = make([]string, numRows)
	var length = len(b)
	var period = numRows*2 - 2
	for i := 0; i < length; i++ {
		var mod = i % period
		if mod < numRows {
			res[mod] += string(b[i])
		} else {
			res[period-mod] += string(b[i])
		}
	}
	return strings.Join(res, "")
}
