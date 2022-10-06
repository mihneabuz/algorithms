package main

import (
	"fmt"
	"sort"
)

type Item struct {
	time  int
	value string
}

type TimeMap struct {
	m map[string][]Item
}

func Constructor() TimeMap {
	return TimeMap{
		m: make(map[string][]Item),
	}
}

func (self *TimeMap) Set(key string, value string, timestamp int) {
	item := Item{
		time:  timestamp,
		value: value,
	}

	slice := self.m[key]

	self.m[key] = append(slice, item)
}

func (self *TimeMap) Get(key string, timestamp int) string {
	slice, found := self.m[key]
	if !found {
		return ""
	}

	index := sort.Search(len(slice), func(i int) bool {
		return slice[i].time > timestamp
	})

	if index < len(slice) && slice[index].time == timestamp {
		return slice[index].value
	} else {
		if index > 0 {
			return slice[index-1].value
		} else {
			return ""
		}
	}
}

func main() {
	fmt.Println("hello world!")
	timeMap := Constructor()

	timeMap.Set("foo", "high", 10)
	timeMap.Set("foo", "low", 20)

	fmt.Println(timeMap.Get("foo", 5))
	fmt.Println(timeMap.Get("foo", 10))
	fmt.Println(timeMap.Get("foo", 15))
	fmt.Println(timeMap.Get("foo", 20))
	fmt.Println(timeMap.Get("foo", 25))
}
