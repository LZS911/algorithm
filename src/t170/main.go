package main

import "math"

const MAX = math.MaxInt32 / 10

func main() {
	myAtoi("323")
	myAtoi("0")
	myAtoi("1")
	myAtoi("42")
}
func myAtoi(s string) int {
	length := len(s)
	index := 0
	for index < length && s[index] == ' ' {
		index++
	}
	if index == length {
		return 0
	}
	sign := 1
	if s[index] == '+' {
		index++
	} else if s[index] == '-' {
		sign = -1
		index++
	}
	num := 0
	for index < length && s[index] >= '0' && s[index] <= '9' {
		digit := int(s[index] - '0')
		if num > MAX || num < -MAX {
			if sign > 0 {
				return math.MaxInt32
			} else {
				return math.MinInt32
			}
		} else if num == MAX || num == -MAX {
			if sign > 0 && digit > 7 {
				return math.MaxInt32
			} else if sign < 0 && digit > 8 {
				return math.MinInt32
			}
		}
		num = num*10 + digit*sign
		index++
	}
	return num
}
