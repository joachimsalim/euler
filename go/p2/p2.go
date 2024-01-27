package main

import "fmt"

func main() {
	var limit uint
	limit = 4000000
	sum := fibonacciSequence(1, 2, limit)

	fmt.Println(sum)
}

func fibonacciSequence(x uint, y uint, limit uint) uint {
	var z, sum uint
	sum = 0

	for {
		z = x + y
		x = y
		y = z
		if x >= limit {
			break
		}
		sum += isEven(x)
	}

	return sum
}

func isEven(x uint) uint {
	if x%2 == 0 {
		return x
	}
	return 0
}
