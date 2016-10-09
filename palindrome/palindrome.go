package palindrome

import (
	"unicode/utf8"
)

// Palindrome returns true if s (a utf8 string) is a palindrome.
func Palindrome(str string) bool {
	s := str[:]
	for _, ru := range str {
		backru, size := utf8.DecodeLastRuneInString(s)
		if ru != backru {
			return false
		}
		s = s[:len(s)-size]
	}

	return true
}
