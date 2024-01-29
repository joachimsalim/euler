package main

import "fmt"

func main() {
	ceiling := 1000
	sum := multiplesOfThreeOrFive(ceiling)

	fmt.Println(sum)
}

func multiplesOfThreeOrFive(ceiling int) int {
	y := 0

	for x := 3; x < ceiling; x++ {
		if x%3 == 0 || x%5 == 0 {
			y += x
		}
	}

	return y
}
