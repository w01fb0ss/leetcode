/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let other = target - &num;
            match mp.get(&other) {
                Some(v) => {
                    return vec![*v as i32, i as i32];
                }
                _ => {
                    mp.insert(num, i);
                }
            }
        }
        return vec![];
    }
}
// @lc code=end

