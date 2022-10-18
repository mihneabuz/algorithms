package main

import "fmt"

func countAndSay(n int) string {
  s := []byte{'1'}

  for k := 1; k < n; k++ {
    b, count, s2 := s[0], 0, make([]byte, 0)
    for _, n := range s {
      if b == n {
        count += 1
      } else {
        s2 = append(s2, byte(count) + '0')
        s2 = append(s2, b)

        b = n
        count = 1
      }
    }

    if count > 0 {
      s2 = append(s2, byte(count) + '0')
      s2 = append(s2, b)
    }

    s = s2
  }

  return string(s)
}

func main() {
  fmt.Println(countAndSay(1))
  fmt.Println(countAndSay(4))
}
