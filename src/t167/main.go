package main

func main() {
	lengthOfLongestSubstring("abcabcbb")
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func lengthOfLongestSubstring(s string) (ans int) {
	window := map[rune]bool{}
	for i, ch := range s {
		for window[ch] {
			delete(window, rune(s[i-len(window)]))
		}
		window[ch] = true
		ans = max(ans, len(window))
	}
	return
}
