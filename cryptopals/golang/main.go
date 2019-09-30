package main

import (
	"fmt"
)

func main() {
	_, r01 := hextobase64()
	r02_ok := xor()
	r03 := singlexor()

	fmt.Println(r01)
	fmt.Println(r02_ok)
	fmt.Println(r03)
}
