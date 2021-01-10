/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() || nums.len() < 3 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;
        let mut best = std::i32::MAX;
        
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target
                }

                if (sum - target).abs() < best {
                    ans = sum;
                    best = (sum - target).abs()
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        ans
    }
}
// @lc code=end

