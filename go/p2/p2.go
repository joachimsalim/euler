package main

import "fmt"

func main() {
	var ceiling uint
	ceiling = 4000000
	sum := fibonacciSequenceOfEvens(1, 2, ceiling)

	fmt.Println(sum)
}

func fibonacciSequenceOfEvens(y uint, x uint, ceiling uint) uint {
	var w, z uint
	z = 0

	for {
		w = y + x
		y = x
		x = w
		if y >= ceiling {
			break
		}
		z += isEven(y)
	}

	return z
}

func isEven(x uint) uint {
	if x%2 == 0 {
		return x
	}
	return 0
}
