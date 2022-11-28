package main

import (
	"fmt"
	"sort"
)

func findWinners(matches [][]int) [][]int {
  players := make(map[int]int)

  for _, match := range matches {
    players[match[0]] += 0
    players[match[1]] += 1
  }

  fmt.Println(players)

  lost_none, lost_one := make([]int, 0), make([]int, 0)

  for player, losses := range players {
    if losses == 0 {
      lost_none = append(lost_none, player)
    }

    if losses == 1 {
      lost_one = append(lost_one, player)
    }
  }

  sort.Ints(lost_none)
  sort.Ints(lost_one)

	return [][]int{lost_none, lost_one}
}

func main() {
	fmt.Println(findWinners([][]int{{1, 3}, {2, 3}, {3, 6}, {5, 6}, {5, 7}, {4, 5}, {4, 8}, {4, 9}, {10, 4}, {10, 9}}))
}
