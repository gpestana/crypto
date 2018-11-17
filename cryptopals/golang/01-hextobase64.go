// Always operate on raw bytes, never on encoded strings. Only use hex and
//base64 for pretty-printing.
package main

import b64 "encoding/base64" // used to print the plaintext
import "fmt"

var base64 = map[int]string{
	0: "A", 1: "B", 2: "C", 3: "D", 4: "E", 5: "F", 6: "G", 7: "H", 8: "I",
	9: "J", 10: "K", 11: "L", 12: "M", 13: "N", 14: "O", 15: "P", 16: "Q", 17: "R",
	18: "S", 19: "T", 20: "U", 21: "V", 22: "W", 23: "X", 24: "Y", 25: "Z", 26: "a",
	27: "b", 28: "c", 29: "d", 30: "e", 31: "f", 32: "g", 33: "h", 34: "i", 35: "j",
	36: "k", 37: "l", 38: "m", 39: "n", 40: "o", 41: "p", 42: "q", 43: "r", 44: "s",
	45: "t", 46: "u", 47: "v", 48: "w", 49: "x", 50: "y", 51: "z", 52: "0", 53: "1",
	54: "2", 55: "3", 56: "4", 57: "5", 58: "6", 59: "7", 60: "8", 61: "9", 62: "+",
	63: "/",
}

var hex = map[string]int{
	"0": 0x0, "1": 0x1, "2": 0x2, "3": 0x3, "4": 0x4, "5": 0x5, "6": 0x6, "7": 0x7,
	"8": 0x8, "9": 0x9, "a": 0xA, "b": 0xB, "c": 0xC, "d": 0xD, "e": 0xE, "f": 0xF,
}

const MASK_RIGHT = 0xFC
const MASK_LEFT = 0x3F

func hextobase64() (bool, string) {
	secret := "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
	_plaintext := "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"

	ok := true
	var pt string
	for i := 0; i < len(secret)/3; i++ {
		indexBase := i * 3
		l, m, r := hex[string(secret[indexBase])], hex[string(secret[indexBase+1])],
			hex[string(secret[indexBase+2])]
		a, b := hexToBase64(l, m, r)
		pt = pt + base64[a] + base64[b]
	}

	if pt != _plaintext {
		fmt.Println(_plaintext, pt)
		return false, ""
	}
	msg := base64ToString(pt)
	return ok, msg
}

// converts 3 hex symbols into 2 base64 symbols
func hexToBase64(l, m, r int) (int, int) {
	return (l<<4 | m&MASK_RIGHT) >> 2, (m<<4 | r) & MASK_LEFT
}

func base64ToString(b string) string {
	s, _ := b64.StdEncoding.DecodeString(b)
	return string(s)
}
