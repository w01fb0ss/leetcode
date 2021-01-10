/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() || nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        let mut ans = Vec::new();
        nums.sort();
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                return ans;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut target = -nums[i];
            let mut x = i + 1;
            let mut y = nums.len() - 1;
            while x < y {
                match nums[x as usize] + nums[y as usize] {
                    i if i < target => {
                        x += 1;
                        while x < y && nums[x as usize] == nums[x as usize - 1] {
                            x += 1;
                        }
                    }
                    i if i > target => {
                        y -= 1;
                        while x < y && nums[y as usize] == nums[y as usize + 1] {
                            y -= 1;
                        }
                    }
                    _ => {
                        let t = vec![nums[i], nums[x as usize], nums[y as usize]];
                        ans.push(t);
                        x += 1;
                        while x < y && nums[x as usize] == nums[x as usize - 1] {
                            x += 1;
                        }
                        y -= 1;
                        while x < y && nums[y as usize] == nums[y as usize + 1] {
                            y -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}
// @lc code=end

