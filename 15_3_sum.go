/*
 * @lc app=leetcode id=15 lang=golang
 *
 * [15] 3Sum
 */

// @lc code=start
func threeSum(nums []int) [][]int {
	ans := make([][]int, 0)
	if nums == nil || len(nums) < 0 {
		return ans
	}
	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] > 0 {
			return ans
		}
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		target := -nums[i]
		x, y := i+1, len(nums)-1
		for x < y {
			switch {
			case nums[x]+nums[y] < target:
				x++
				for x < y && nums[x] == nums[x-1] {
					x++
				}
			case nums[x]+nums[y] > target:
				y--
				for x < y && nums[y] == nums[y+1] {
					y--
				}
			case nums[x]+nums[y] == target:
				ans = append(ans, []int{nums[i], nums[x], nums[y]})
				x++
				y--
				for x < y && nums[x] == nums[x-1] {
					x++
				}
				for x < y && nums[y] == nums[y+1] {
					y--
				}
			}
		}
	}
	return ans
}

// @lc code=end

