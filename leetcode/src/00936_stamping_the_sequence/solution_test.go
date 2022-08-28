package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMovesToStamp(t *testing.T) {
	tests := []struct {
		stamp  string
		target string

		expected []int
	}{
		{"abc", "ababc", []int{0, 2}},
		{"abca", "aabcaca", []int{0, 3, 1}},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := movesToStamp(tt.stamp, tt.target)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
