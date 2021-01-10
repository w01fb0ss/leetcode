/*
 * @lc app=leetcode id=1 lang=golang
 *
 * [1] Two Sum
 */

// @lc code=start
func twoSum(nums []int, target int) []int {
	mp := make(map[int]int)
	for i, num := range nums {
		if j, ok := mp[target-num]; ok {
			return []int{j, i}
		}
		mp[num] = i
	}
	return []int{}
}

// @lc code=end

