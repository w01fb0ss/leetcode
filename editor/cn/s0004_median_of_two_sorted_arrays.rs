struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        nums1.extend_from_slice(&nums2);
        nums1.sort();

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