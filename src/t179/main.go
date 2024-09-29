package main

import (
	"fmt"
)

func main() {
	fmt.Println(removeStars("la*ewr*fdsf**fdsf****f"))
}

func removeStars(s string) string {
	var ans []rune
	for _, c := range s {
		if c == '*' {
			ans = ans[:len(ans)-1]
		} else {
			ans = append(ans, c)
		}
	}

	return string(ans)
}
