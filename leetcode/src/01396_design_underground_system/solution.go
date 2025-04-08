package leetcode

import (
	"math"
)

type StationTime struct {
	s string
	t int
}

type UndergroundSystem struct {
	idToStationTime map[int]StationTime
	routeToSumCnt   map[[2]string][2]int
}

func Constructor() UndergroundSystem {
	return UndergroundSystem{
		idToStationTime: map[int]StationTime{},
		routeToSumCnt:   map[[2]string][2]int{},
	}
}

func (us *UndergroundSystem) CheckIn(id int, stationName string, t int) {
	us.idToStationTime[id] = StationTime{
		s: stationName,
		t: t,
	}
}

func (us *UndergroundSystem) CheckOut(id int, stationName string, t int) {
	stationTime, ok := us.idToStationTime[id]
	if ok {
		route := [2]string{stationTime.s, stationName}
		sumCnt := us.routeToSumCnt[route]
		us.routeToSumCnt[route] = [2]int{sumCnt[0] + t - stationTime.t, sumCnt[1] + 1}
	}
}

func (us *UndergroundSystem) GetAverageTime(startStation string, endStation string) float64 {
	route := [2]string{startStation, endStation}
	sumCnt := us.routeToSumCnt[route]
	return math.Round(float64(sumCnt[0])/float64(sumCnt[1])*100000) / 100000
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * obj := Constructor();
 * obj.CheckIn(id,stationName,t);
 * obj.CheckOut(id,stationName,t);
 * param_3 := obj.GetAverageTime(startStation,endStation);
 */
