package main

import (
	"fmt"
)

type Entry struct {
	word string
	freq int
}

type Heap []Entry

func (e Entry) Less(o Entry) bool {
	if e.freq == o.freq {
		return e.word < o.word
	}
	return e.freq > o.freq
}

func (h *Heap) Push(e Entry) {
	i := len(*h)
	*h = append(*h, e)
	h.heapifyUp(i)
}

func (h *Heap) Pop() Entry {
	e := (*h)[0]
	(*h)[0] = (*h)[len(*h)-1]
	*h = (*h)[:len(*h)-1]
	h.heapifyDown(0)
	return e
}

func (h Heap) heapifyUp(i int) {
  for i != 0 {
    p := (i - 1) / 2
    if h[i].Less(h[p]) {
      h[i], h[p] = h[p], h[i]
    } else {
      return
    }

    i = p
  }
}

func (h Heap) heapifyDown(i int) {
  for i < len(h) {
    l, r := (2 * i + 1), (2 * i + 2)

    if l >= len(h) {
      return
    }

    if r >= len(h) {
      if h[l].Less(h[i]) {
        h[i], h[l] = h[l], h[i]
      }

      return
    }

    iv, lv, rv := h[i], h[l], h[r]
    if lv.Less(rv) {
      if lv.Less(iv) {
        h[i], h[l] = lv, iv
        i = l
      } else {
        return
      }
    } else {
      if rv.Less(iv) {
        h[i], h[r] = rv, iv
        i = r
      } else {
        return
      }
    }
  }
}

func topKFrequent(words []string, k int) []string {
	freq := make(map[string]int)
	for _, word := range words {
		freq[word] += 1
	}

	heap := &Heap{}
	for word, freq := range freq {
		heap.Push(Entry{word: word, freq: freq})
	}

	res := make([]string, k)
	for i := 0; i < k; i++ {
		res[i] = heap.Pop().word
	}

	return res
}

func main() {
	fmt.Println(topKFrequent([]string{"i", "love", "leetcode", "i", "love", "coding"}, 2))
	fmt.Println(topKFrequent([]string{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"}, 4))
	fmt.Println(topKFrequent([]string{"plpaboutit","jnoqzdute","sfvkdqf","mjc","nkpllqzjzp","foqqenbey","ssnanizsav","nkpllqzjzp","sfvkdqf","isnjmy","pnqsz","hhqpvvt","fvvdtpnzx","jkqonvenhx","cyxwlef","hhqpvvt","fvvdtpnzx","plpaboutit","sfvkdqf","mjc","fvvdtpnzx","bwumsj","foqqenbey","isnjmy","nkpllqzjzp","hhqpvvt","foqqenbey","fvvdtpnzx","bwumsj","hhqpvvt","fvvdtpnzx","jkqonvenhx","jnoqzdute","foqqenbey","jnoqzdute","foqqenbey","hhqpvvt","ssnanizsav","mjc","foqqenbey","bwumsj","ssnanizsav","fvvdtpnzx","nkpllqzjzp","jkqonvenhx","hhqpvvt","mjc","isnjmy","bwumsj","pnqsz","hhqpvvt","nkpllqzjzp","jnoqzdute","pnqsz","nkpllqzjzp","jnoqzdute","foqqenbey","nkpllqzjzp","hhqpvvt","fvvdtpnzx","plpaboutit","jnoqzdute","sfvkdqf","fvvdtpnzx","jkqonvenhx","jnoqzdute","nkpllqzjzp","jnoqzdute","fvvdtpnzx","jkqonvenhx","hhqpvvt","isnjmy","jkqonvenhx","ssnanizsav","jnoqzdute","jkqonvenhx","fvvdtpnzx","hhqpvvt","bwumsj","nkpllqzjzp","bwumsj","jkqonvenhx","jnoqzdute","pnqsz","foqqenbey","sfvkdqf","sfvkdqf"}, 4))
}
