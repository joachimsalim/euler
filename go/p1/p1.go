package main

import "fmt"

func main() {
	bound := 1000
	sum := multiplesOfThreeOrFive(bound)

	fmt.Println(sum)
}

func multiplesOfThreeOrFive(bound int) int {
	sum := 0

	for x := 1; x < bound; x++ {
		if x%3 == 0 || x%5 == 0 {
			sum += x
		}
	}

	return sum
}
