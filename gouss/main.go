package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

const MAXN = 2000

var N int

var a [MAXN][MAXN]float64
var b [MAXN]float64
var x [MAXN]float64

func main() {
	var startTime time.Time
	var finalTime float64
	Parameters(os.Args)
	Initialize()
	Printing()

	fmt.Println("Starting clock.")
	startTime = time.Now()

	Gauss()

	finalTime = float64(time.Since(startTime).Nanoseconds()) / 1000000.0
	fmt.Println("Stopped clock.")

	PrintingX()

	fmt.Printf("Time elapsed: %.2fms \n", finalTime)
}

func Parameters(args []string) {
	var input int
	var seed int64 = time.Now().UnixNano()
	var err error
	if len(args) >= 3 {
		input, err = strconv.Atoi(args[2])
		if err != nil {
			panic(err)
		}
		seed = int64(input)
		fmt.Println("Seed =", seed)
	}
	rand.Seed(seed)
	if len(args) >= 2 {
		N, err = strconv.Atoi(args[1])
		if err != nil {
			panic(err)
		}
		if N < 1 || N > MAXN {
			panic("Out of range.")
		}
	} else {
		panic("Usage: <matrix_dimension> [seed]")
	}

	fmt.Println("Matrix dimension =", N)
}

func Initialize() {
	fmt.Println("Initializing. . .")
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			a[i][j] = float64(rand.Intn(9999998)+1) / 100.0
		}
		b[i] = float64(rand.Intn(9999998)+1) / 100.0
	}
}

func Printing() {
	if N < 10 {
		fmt.Println("A [")
		for i := 0; i < N; i++ {
			for j := 0; j < N; j++ {
				fmt.Printf("%9.2f", a[i][j])
			}
			fmt.Println("")
		}
		fmt.Println("\nB [ ")
		for i := 0; i < N; i++ {
			fmt.Printf("%9.2f", b[i])
		}
		fmt.Println(" ]")
	}
}

func PrintingX() {
	if N < 100 {
		fmt.Print("X [")

		for i := 0; i < N; i++ {
			fmt.Printf("%9.2f", x[i])
		}
		fmt.Println(" ]")
	}
}

func Gauss() {
	fmt.Println("Computing. . .")
	for norm := 0; norm < N-1; norm++ {
		for row := norm + 1; row < N; row++ {
			var mult float64 = a[row][norm] / a[norm][norm]
			for col := norm; col < N; col++ {
				a[row][col] -= a[norm][col] * mult
			}
			b[row] -= b[norm] * mult
		}
	}

	for row := N - 1; row >= 0; row-- {
		x[row] = b[row]
		for col := N - 1; col > row; col-- {
			x[row] -= a[row][col] * x[col]
		}
		x[row] /= a[row][row]
	}
}
