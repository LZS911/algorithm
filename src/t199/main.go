package main

import "fmt"

func main() {
	fmt.Println(squareIsWhite("a1"))
	fmt.Println(squareIsWhite("h3"))
	fmt.Println(squareIsWhite("c7"))
}

func squareIsWhite(coordinates string) bool {
	return ((coordinates[0]-'a'+1)+(coordinates[1]-'0'))%2 == 1
}
