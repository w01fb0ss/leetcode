use std::collections::HashMap;

struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (index, &value) in nums.iter().enumerate() {
            let other = target - value;
            match m.get(&other) {
                Some(v) => return vec![*v as i32, index as i32],
                None => m.insert(value, index),
            };
        }
        vec![]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        
    }
}