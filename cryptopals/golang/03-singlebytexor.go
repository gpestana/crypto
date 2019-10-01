// string has been XOR'd against a single character. Find the key, decrypt the
// message.
package main

import (
	"fmt"
	"math"
)

const (
	st = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
)

func singlexor() string {

	for x := 97; x <= 121; x++ {
		for _, c := range st {
			fmt.Println(x, c)
		}
	}
	return st
}

func chiSquared(expected, observed float64) float64 {
	return math.Pow((expected-observed), 2) / expected
}

func letterFrequencyEnglish() map[string]float64 {
	return map[string]float64{
		"a": 11.682,
		"b": 4.434,
		"c": 5.238,
		"d": 3.174,
		"e": 2.799,
		"f": 4.027,
		"g": 1.642,
		"h": 4.200,
		"i": 7.294,
		"j": 0.511,
		"k": 0.456,
		"l": 2.415,
		"m": 3.826,
		"n": 2.284,
		"o": 7.631,
		"p": 4.319,
		"q": 0.222,
		"r": 2.826,
		"s": 6.686,
		"t": 15.978,
		"u": 1.183,
		"v": 0.824,
		"w": 5.497,
		"x": 0.045,
		"y": 0.763,
		"z": 0.045,
	}
}
