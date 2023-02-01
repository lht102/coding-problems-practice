package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGcdOfStrings(t *testing.T) {
	tests := []struct {
		str1 string
		str2 string

		expected string
	}{
		{"ABCABC", "ABC", "ABC"},
		{"ABABAB", "ABAB", "AB"},
		{"LEET", "CODE", ""},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := gcdOfStrings(tt.str1, tt.str2)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
