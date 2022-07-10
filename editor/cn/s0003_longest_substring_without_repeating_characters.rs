struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut m = HashMap::new();
        let (mut max_len, mut tmp_len, mut start) = (0, 0, 0);
        for (i, c) in s.chars().enumerate() {
            match m.get(&c) {
                Some(v) => {
                    start = start.max(*v as i32);
                    tmp_len = i as i32 - start;
                },
                None => tmp_len += 1,
            };
            max_len = max_len.max(tmp_len);
            m.insert(c, i);
        }
        max_len
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3)
    }
}