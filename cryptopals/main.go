package main

import (
	"fmt"
)

func main() {
	ok, msg := hextobase64()
	fmt.Printf("01. hex to base64: %v. %v \n", ok, msg)
}
