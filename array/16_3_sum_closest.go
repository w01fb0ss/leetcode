/*
 * @lc app=leetcode id=16 lang=golang
 *
 * [16] 3Sum Closest
 */

// @lc code=start
func threeSumClosest(nums []int, target int) int {
	if nums == nil || len(nums) < 3 {
		return 0
	}
	sort.Ints(nums)
	best := math.MaxInt32
	ans := 0
	for i := 0; i < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		j, k := i+1, len(nums)-1
		for j < k {
			sum := nums[i] + nums[j] + nums[k]
			if sum == target {
				return target
			}
			if abs(sum-target) < best {
				ans, best = sum, abs(sum-target)
			}
			if sum < target {
				j++
			} else {
				k--
			}
		}
	}
	return ans

}

func abs(a int) int {
	if a > 0 {
		return a
	}
	return -a
}

// @lc code=end

