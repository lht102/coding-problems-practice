package leetcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinRefuelStops(t *testing.T) {
	tests := []struct {
		target    int
		startFuel int
		stations  [][]int

		expected int
	}{
		{1, 1, [][]int{}, 0},
		{100, 1, [][]int{{10, 100}}, -1},
		{100, 10, [][]int{{10, 60}, {20, 30}, {30, 30}, {60, 40}}, 2},
	}

	for i, tt := range tests {
		tt := tt
		t.Run(fmt.Sprintf("Test %d", i), func(t *testing.T) {
			t.Parallel()
			actual := minRefuelStops(tt.target, tt.startFuel, tt.stations)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
