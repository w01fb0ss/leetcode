struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() || nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();

        let mut ret:Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                return ret;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = -nums[i];
            let(mut x, mut y) = (i + 1, nums.len() - 1);
            while x < y {
                match nums[x] + nums[y] {
                    i if i < target => {
                        x += 1;
                        while x < y && nums[x] == nums[x - 1]{
                            x += 1;
                        }
                    },
                    i if i > target => {
                        y -= 1;
                        while x < y && nums[y] == nums[y + 1] {
                            y -= 1;
                        }
                    },
                    _ => {
                        let t = vec![nums[i], nums[x], nums[y]];
                        ret.push(t);
                        x += 1;
                        while x < y && nums[x] == nums[x - 1] {
                            x += 1;
                        }
                        y -= 1;
                        while x < y && nums[x] == nums[x - 1] {
                            y -= 1;
                        }
                    }
                }
            }
        }
        ret
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