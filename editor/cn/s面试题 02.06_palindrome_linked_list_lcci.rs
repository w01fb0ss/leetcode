struct Solution;
use std::collections
// use crate::*;

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut v = Vec::new();
        while head.is_some() {
            v.push(head.as_ref().unwrap().val);
            head = head.unwrap().next;
        }
        let c = v.clone();
        v.reverse();
        c == v
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        
    }
}