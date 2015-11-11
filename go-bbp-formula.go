# https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

package main

import(
	"fmt"
	"math"
)

func main() {
	numSteps := 11
	piSummation := 0.0
	
	for i := 0; i < numSteps; i++ {
		piSummation += (1.0/math.Pow(16.0, float64(i))) * ((4.0/(8.0*float64(i) + 1.0)) - (2.0/(8.0*float64(i) + 4.0)) - (1.0/(8.0*float64(i) + 5.0)) - (1.0/(8.0*float64(i) + 6.0)))	
	}
	
	fmt.Printf("Pi = %.20f", piSummation)
}

