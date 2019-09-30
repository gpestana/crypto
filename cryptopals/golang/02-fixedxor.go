// Write a function that takes two equal-length buffers and produces their XOR
// combination.
package main

const (
	s1             = "1c0111001f010100061a024b53535009181c"
	s2             = "686974207468652062756c6c277320657965"
	expectedResult = "746865206b696420646f6e277420706c6179"
)

func xor() bool {
	result := make([]byte, len(s1))
	ok := true

	for i, _ := range s1 {
		result[i] = charToHex(s1[i]) ^ charToHex(s2[i])
		if result[i] != charToHex(expectedResult[i]) {
			return false
		}
	}
	return ok
}

func charToHex(c byte) byte {
	if c > 57 {
		return c - 87
	}
	return c - 48
}
