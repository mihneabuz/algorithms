package main

import "fmt"

func breakPalindrome(palindrome string) string {
	if len(palindrome) == 1 {
		return ""
	}

	for i := 0; i < len(palindrome)/2; i++ {
		if palindrome[i] != 'a' {
			return palindrome[:i] + "a" + palindrome[i+1:]
		}
	}

	return palindrome[:len(palindrome)-1] + "b"
}

func main() {
	fmt.Println(breakPalindrome("abccba"))
	fmt.Println(breakPalindrome("a"))
}
