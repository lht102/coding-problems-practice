package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReorderedPowerOf2(t *testing.T) {
	tests := []struct {
		n int

		expected bool
	}{
		{1, true},
		{10, false},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := reorderedPowerOf2(tt.n)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
