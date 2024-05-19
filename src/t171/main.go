package main

func main() {
	intToRoman(3749)
	intToRoman(58)
	intToRoman(1994)
}

func intToRoman(num int) (ans string) {
	type roman struct {
		val int
		sym string
	}
	symbols := []roman{
		{1000, "M"},
		{900, "CM"},
		{500, "D"},
		{400, "CD"},
		{100, "C"},
		{90, "XC"},
		{50, "L"},
		{40, "XL"},
		{10, "X"},
		{9, "IX"},
		{5, "V"},
		{4, "IV"},
		{1, "I"},
	}

	for _, roman := range symbols {
		for num >= roman.val {
			ans += roman.sym
			num -= roman.val
		}
	}

	return
}
