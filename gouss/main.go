package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

func main() {
	const N int = 2000

	var startTime time.Time
	var finalTime float64
	var a [N][N]float64
	var b [N]float64
	var x [N]float64
	var seed int64
	var size int
	var input int
	var err error

	if len(os.Args) == 1 || len(os.Args) > 3 {
		panic("Usage: <matrix_dimension> [seed]")
	} else {
		input, err = strconv.Atoi(os.Args[1])
		if err != nil {
			panic(err)
		}
		if input < 1 || input > 2000 {
			panic("size is out of range.")
		}
	}

	size = input

	if len(os.Args) == 3 {
		input, err = strconv.Atoi(os.Args[2])
		if err != nil {
			panic(err)
		}
		seed = int64(input)
	} else {
		seed = time.Now().UnixNano()
	}

	rand.Seed(seed)

	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			a[i][j] = float64(rand.Intn(9999998)+1) / 100.0
		}
		b[i] = float64(rand.Intn(9999998)+1) / 100.0
	}

	if size < 10 {
		fmt.Println("A [")
		for i := 0; i < size; i++ {
			for j := 0; j < size; j++ {
				fmt.Printf("%9.2f", a[i][j])
			}
			fmt.Println("")
		}
		fmt.Println("]")
		fmt.Print("B [")

		for i := 0; i < size; i++ {
			fmt.Printf("%9.2f", b[i])
		}
		fmt.Println(" ]")
	}

	startTime = time.Now()
	for norm := 0; norm < size-1; norm++ {
		for row := norm + 1; row < size; row++ {
			var mult float64 = a[row][norm] / a[norm][norm]
			for col := norm; col < size; col++ {
				a[row][col] -= a[norm][col] * mult
			}
			b[row] -= b[norm] * mult
		}
	}

	for row := size - 1; row >= 0; row-- {
		x[row] = b[row]
		for col := size - 1; col > row; col-- {
			x[row] -= a[row][col] * x[col]
		}
		x[row] /= a[row][row]
	}
	finalTime = float64(time.Since(startTime).Nanoseconds()) / 1000000.0

	if size < 100 {
		fmt.Print("X [")

		for i := 0; i < size; i++ {
			fmt.Printf("%9.2f", x[i])
		}
		fmt.Println(" ]")
	}

	fmt.Printf("elapsed time: %.2fms \n", finalTime)
}
