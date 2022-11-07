package main

import (
  "sort"
  "bytes"
)

func orderlyQueue(s string, k int) string {
    if k > 1 {
        return lexicoSort(s)
    } else {
        return rotate(s)
    }
}

func lexicoSort(s string) string {
    b := []byte(s)
    sort.Slice(b, func (i, j int) bool { return b[i] < b[j] })
    return string(b)
}

func rotate(s string) string {
    b := []byte(s)
    min := make([]byte, len(b))
    copy(min, b)

    for i := 0; i < len(b); i += 1 {
        b = append(b[1:], b[0])
        if bytes.Compare(min, b) > 0 {
            copy(min, b)
        }
    }

    return string(min)
}
