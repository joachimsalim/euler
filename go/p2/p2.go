package main

import "fmt"

func main() {
	fmt.Println(fibonacciSequence(1, 2, 4000000))
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
		sum += evaluate(x)
	}

	return sum
}

func evaluate(x uint) uint {
	if x%2 == 0 {
		return x
	}
	return 0
}
