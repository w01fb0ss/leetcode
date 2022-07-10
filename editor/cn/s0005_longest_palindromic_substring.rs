struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut start, mut end) = (0, 0);
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..n {
            let (mut l, mut r) = (i, i);
            while r + 1 < n && s[r+1] == s[l] {
                r += 1;
            }
            while l > 0 && r + 1 < n && s[l - 1] == s[r + 1] {
                l -= 1;
                r += 1;
            }
            if end - start < r - l {
                start = l;
                end = r;
            }
        }
        s[start..=end].iter().collect::<String>()
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        let s = "babad".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bab".to_string());
    }
}