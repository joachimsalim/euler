package main

import "fmt"

func main() {
	fmt.Println(multiplesOfThreeOrFive())
}

func multiplesOfThreeOrFive() int {
	sum := 0

	for x := 1; x < 1000; x++ {
		sum += evaluate(x)
	}

	return sum
}

func evaluate(x int) int {
	if x%3 == 0 || x%5 == 0 {
		return x
	}
	return 0
}
