package main

import "sort"

func main() {
}

func latestTimeCatchTheBus(buses []int, passengers []int, capacity int) int {
	sort.Ints(buses)
	sort.Ints(passengers)

	pos := 0
	space := 0

	for _, arrive := range buses {
		space = capacity
		for space > 0 && pos < len(passengers) && passengers[pos] <= arrive {
			space--
			pos++
		}
	}

	pos--
	lastCatchTime := buses[len(buses)-1]
	if space <= 0 {
		lastCatchTime = passengers[pos]
	}
	for pos >= 0 && passengers[pos] == lastCatchTime {
		pos--
		lastCatchTime--
	}

	return lastCatchTime

}
