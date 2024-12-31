package main

import "fmt"

func main() {
	fmt.Println(maxHeightOfTriangle(2, 4))
	fmt.Println(maxHeightOfTriangle(2, 1))
	fmt.Println(maxHeightOfTriangle(1, 1))
	fmt.Println(maxHeightOfTriangle(10, 1))
	fmt.Println(maxHeightOfTriangle(10, 10))
	fmt.Println(maxHeightOfTriangle(4, 9))
}

func maxHeightOfTriangle(red int, blue int) int {
	if red == blue {
		return getTriangleMax(red, blue)
	}

	return getMax(getTriangleMax(red, blue), getTriangleMax(blue, red))
}

func getTriangleMax(num1 int, num2 int) int {
	res := 0

	for num1 >= res+1 {
		res++
		num1 -= res
		if num2 >= res+1 {
			res++
			num2 -= res
		} else {
			break
		}
	}

	return res
}

func getMax(a int, b int) int {
	if a > b {
		return a
	}
	return b
}
