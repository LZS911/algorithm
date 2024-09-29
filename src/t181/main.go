package main

func main() {
}

func distanceBetweenBusStops(distance []int, start int, destination int) int {
	sum1, sum2 := 0, 0

	if start > destination {
		start, destination = destination, start
	}

	for i, v := range distance {
		if i >= start && i < destination {
			sum1 += v
		} else {
			sum2 += v
		}
	}

	if sum1 > sum2 {
		return sum2
	}

	return sum1
}
