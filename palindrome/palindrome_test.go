package palindrome

import (
	"testing"
)

func TestPalindrome(t *testing.T) {
	for _, tcase := range []struct {
		input   string
		correct bool
	}{
		{"racecar", true},
		{"✓rac✓e✓car✓", true},
		{"albert", false},
		{"", true},
	} {
		if result := Palindrome(tcase.input); result != tcase.correct {
			t.Errorf(
				"got %v; wanted %v for input \"%s\"",
				result,
				tcase.correct,
				tcase.input,
			)
		}
	}

}
