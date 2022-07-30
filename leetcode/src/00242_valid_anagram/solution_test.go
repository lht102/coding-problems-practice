package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsAnagram(t *testing.T) {
	tests := []struct {
		s string
		t string

		expected bool
	}{
		{"anagram", "nagaram", true},
		{"rat", "car", false},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := isAnagram(tt.s, tt.t)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
