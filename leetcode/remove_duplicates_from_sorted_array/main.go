func removeDuplicates(nums []int) int {
    k := 0
    for i, num := range nums {
        if num == nums[i - k] {
            k += 1
        } else {
            nums[i - k + 1] = num
        }
    }

    return len(nums) - k + 1
}
