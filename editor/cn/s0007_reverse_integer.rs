struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.abs().to_string()
            .chars().rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0) * x.signum()
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        println!("{}", "012".parse::<i32>().unwrap());
    }
}