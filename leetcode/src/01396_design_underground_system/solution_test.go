package leetcode

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUndergroundSystem(t *testing.T) {
	system1 := Constructor()
	system1.CheckIn(45, "Leyton", 3)
	system1.CheckIn(32, "Paradise", 8)
	system1.CheckIn(27, "Leyton", 10)
	system1.CheckOut(45, "Waterloo", 15)
	system1.CheckOut(27, "Waterloo", 20)
	system1.CheckOut(32, "Cambridge", 22)
	assert.Equal(t, 14.0, system1.GetAverageTime("Paradise", "Cambridge"))
	assert.Equal(t, 11.0, system1.GetAverageTime("Leyton", "Waterloo"))
	system1.CheckIn(10, "Leyton", 24)
	system1.CheckOut(10, "Waterloo", 38)
	assert.Equal(t, 12.0, system1.GetAverageTime("Leyton", "Waterloo"))

	system2 := Constructor()
	system2.CheckIn(10, "Leyton", 3)
	system2.CheckOut(10, "Paradise", 8)
	assert.Equal(t, 5.0, system2.GetAverageTime("Leyton", "Paradise"))
	system2.CheckIn(5, "Leyton", 10)
	system2.CheckOut(5, "Paradise", 16)
	assert.Equal(t, 5.5, system2.GetAverageTime("Leyton", "Paradise"))
	system2.CheckIn(2, "Leyton", 21)
	system2.CheckOut(2, "Paradise", 30)
	assert.Equal(t, 6.66667, system2.GetAverageTime("Leyton", "Paradise"))
}
