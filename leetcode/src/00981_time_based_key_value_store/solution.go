package leetcode

type TimeMap struct {
	keyTimestampMap map[string][]timestampVal
}

type timestampVal struct {
	timestamp int
	val       string
}

func Constructor() TimeMap {
	return TimeMap{
		keyTimestampMap: map[string][]timestampVal{},
	}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {
	this.keyTimestampMap[key] = append(this.keyTimestampMap[key], timestampVal{
		timestamp: timestamp,
		val:       value,
	})
}

func (this *TimeMap) Get(key string, timestamp int) string {
	timestampVals, exist := this.keyTimestampMap[key]
	if !exist {
		return ""
	}
	lo, hi := 0, len(timestampVals)
	for lo < hi {
		mid := lo + (hi-lo)/2
		if timestampVals[mid].timestamp > timestamp {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	if lo-1 < 0 {
		return ""
	}
	return timestampVals[lo-1].val
}
