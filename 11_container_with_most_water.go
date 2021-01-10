/*
 * @lc app=leetcode id=11 lang=golang
 *
 * [11] Container With Most Water
 */

// @lc code=start
func maxArea(height []int) int {
	left, right := 0, len(height)-1
	m := 0
	for left < right {
		m = max(m, min(height[left], height[right])*long(left, right))
		if height[left] < height[right] {
			left++
			continue
		}
		right--
	}
	return m
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func long(a, b int) int {
	return b - a
}

// @lc code=end

