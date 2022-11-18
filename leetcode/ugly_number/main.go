package main

import "fmt"

var factors = []int{2, 3, 5}

func isUgly(n int) bool {
	if n <= 0 {
		return false
	}

	for _, factor := range factors {
		for n%factor == 0 {
			n = n / factor
		}
	}

	return n == 1
}

func main() {
	fmt.Println(isUgly(6))
	fmt.Println(isUgly(1))
	fmt.Println(isUgly(14))
}
