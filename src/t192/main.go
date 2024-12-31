package main

import "fmt"

func main() {
	// fmt.Println(destCity([][]string{{"B", "C"}, {"D", "B"}, {"C", "A"}}))
	fmt.Println(destCity([][]string{{"qMTSlfgZlC", "ePvzZaqLXj"}, {"xKhZXfuBeC", "TtnllZpKKg"}, {"ePvzZaqLXj", "sxrvXFcqgG"}, {"sxrvXFcqgG", "xKhZXfuBeC"}, {"TtnllZpKKg", "OAxMijOZgW"}}))
}

// [["ZyRLxE xmA","WQztariTJd"],["pSoBauoJox","ZyRLxE xmA"],["lbEWEqcBKg","tKsFZosRmT"],["QfKdgCRgGv","lbEWEqcBKg"],["WQztariTJd","QfKdgCRgGv"],["tKsFZosRmT","NhNPVREEsh"]]

func destCity(paths [][]string) string {
	dest := paths[0][1]

	if len(paths) == 1 {
		return dest
	}

	starts := map[string]bool{}

	for i := 0; i < len(paths); i++ {
		starts[paths[i][0]] = true
	}

	for i := 0; i < len(paths); i++ {
		if !starts[paths[i][1]] {
			return paths[i][1]
		}
	}

	return dest
}
