struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = Vec::new();
        Self::recur(String::new(), n, n, &mut ret);
        ret
    }

    fn recur(s: String, left: i32, right: i32, ret: &mut Vec<String>) {
        if left > right{
            return;
        }
        if right == 0 {
            ret.push(s);
            return;
        }
        if left > 0 {
            Self::recur(format!("{}{}", s, '('), left - 1, right, ret);
        }
        if right > 0 {
            Self::recur(format!("{}{}", s, ')'), left, right - 1, ret);
        }
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