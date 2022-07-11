struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;

        let mut m = HashMap::new();
        m.insert(')', '(');
        m.insert('}', '{');
        m.insert(']', '[');

        let mut stack = Vec::new();

        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let pop = stack.pop().unwrap();
                if pop != *m.get(&c).unwrap() {
                    return false;
                }
            }

        }


        stack.is_empty()
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