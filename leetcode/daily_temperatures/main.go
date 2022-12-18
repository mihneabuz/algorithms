package main

type Temp struct {
    val int
    idx int
}

func dailyTemperatures(temperatures []int) []int {
    res := make([]int, len(temperatures))
    stack := make([]Temp, 0)

    for i, t := range temperatures {
        for j := len(stack) - 1; j >= 0; j -= 1 {
            if t > stack[j].val {
                res[stack[j].idx] = i - stack[j].idx;
                stack = stack[0:j]
            } else {
                break
            }
        }

        stack = append(stack, Temp{ val: t, idx: i })
    }

    return res
}
